// Import required types and modules from Pliron and Cranelift.
// Pliron imports: Basic IR entities such as context, operations, regions, types, etc.
use pliron::{
    basic_block::BasicBlock,
    builtin::{
        op_interfaces::OneRegionInterface,
        ops::FuncOp,
        types::{FunctionType, IntegerType, Signedness},
    },
    context::{Context, Ptr},
    identifier::Identifier,
    op::Op,
    operation::Operation,
    r#type::TypeObj,
    region::Region,
    result::Result,
    value::Value as PlironValue,
};

// Cranelift imports: Provide IR structures and analysis tools for Cranelift IR.
use cranelift_codegen::{
    dominator_tree::DominatorTree,
    flowgraph::ControlFlowGraph,
    ir::{
        types::Type as ClifType, Block as ClifBasicBlock, DataFlowGraph, Function, Inst, Opcode,
        Value as ClifValue, ValueDef,
    },
};
// Import a fast hash map from rustc_hash crate.
use rustc_hash::FxHashMap;

// Import our own modules that define op interfaces and operations.
use crate::{
    op_interfaces::{BinArithOp, UnaryArithOp},
    ops::{IAddOp, ISubOp, IabsOp, InegOp, ReturnOp, UmaxOp, UminOp},
};

/// Converts a slice of Cranelift IR values ([ClifValue]) into Pliron IR values ([PlironValue]).
///
/// This function iterates over the provided Cranelift operands, determines whether each operand
/// is defined by an instruction result or a block parameter, and converts it accordingly.
///
/// # Arguments
/// - `ctx`: The Pliron context used to create or lookup IR entities.
/// - `dfg`: The Cranelift Data Flow Graph that holds definitions for values.
/// - `cctx`: The conversion context which caches already converted entities.
/// - `operands`: A slice of Cranelift values to be converted.
///
/// # Returns
/// A Result containing a vector of converted [PlironValue]s or an error if conversion fails.
///
/// # Panics
/// Panics if a `Union` value definition is encountered (not yet implemented).
fn convert_operands(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    cctx: &mut ConversionCtx,
    operands: &[ClifValue],
) -> Result<Vec<PlironValue>> {
     // Preallocate a vector with the capacity of the operands length.
    let mut pliron_operands = Vec::with_capacity(operands.len());
    for operand in operands {
        let operand_def = dfg.value_def(*operand);
        match operand_def {
            // If the value is defined as a result of an instruction:
            // Convert the instruction (if not already converted) and create a corresponding op-result value.
            ValueDef::Result(inst, idx) => {
                let op = convert_instruction(ctx, dfg, cctx, inst)?;
                let pliron_value = PlironValue::OpResult { op, res_idx: idx };
                pliron_operands.push(pliron_value);
            }
            // If the value is a block parameter:
            // Convert the corresponding basic block and create a block argument value.
            ValueDef::Param(block, idx) => {
                let block = convert_block(ctx, dfg, cctx, block)?;
                let pliron_value = PlironValue::BlockArgument {
                    block,
                    arg_idx: idx,
                };
                pliron_operands.push(pliron_value);
            }
            ValueDef::Union(_value, _value1) => todo!(),
        }
    }
    Ok(pliron_operands)
}

/// Converts a Cranelift instruction ([Inst]) to a Pliron operation ([Ptr<Operation>]).
///
/// This function first checks whether the instruction has already been converted (caching is used).
/// If not, it determines the opcode and converts the instruction accordingly.
/// Supported opcodes include: Iadd, Umin, Umax, Ineg, Iabs, and Return.
///
/// # Arguments
/// - `ctx`: The Pliron context for IR entity creation.
/// - `dfg`: The Cranelift Data Flow Graph containing instruction definitions.
/// - `cctx`: The conversion context for caching converted instructions.
/// - `inst`: The Cranelift instruction to convert.
///
/// # Returns
/// A Result containing a pointer to the converted [Operation] or an error if conversion fails.
///
/// # Panics
/// Panics if an unsupported opcode is encountered.
fn convert_instruction(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    cctx: &mut ConversionCtx,
    inst: Inst,
) -> Result<Ptr<Operation>> {
    // If the instruction has already been converted, return the cached operation.
    if let Some(op) = cctx.ops.get(&inst) {
        return Ok(*op);
    };

    // Retrieve the opcode and arguments for the instruction.
    let clif_opcode = dfg.insts[inst].opcode();
    let inst_args = dfg.inst_args(inst);
    match clif_opcode {
        // Handle integer addition.
        Opcode::Iadd => {
            let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
            let iadd_op = IAddOp::new(ctx, operands[0], operands[1]);
            let op = iadd_op.get_operation();
            return Ok(op);
        }
        // Handle integer subtraction.
        Opcode::Isub => {
            let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
            let isub_op = ISubOp::new(ctx, operands[0], operands[1]);
            let op = isub_op.get_operation();
            return Ok(op);
        }
        // Handle unsigned minimum.
        Opcode::Umin => {
            let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
            let umin_op = UminOp::new(ctx, operands[0], operands[1]);
            let op = umin_op.get_operation();
            return Ok(op);
        }
        // Handle unsigned maximum.
        Opcode::Umax => {
            let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
            let umax_op = UmaxOp::new(ctx, operands[0], operands[1]);
            let op = umax_op.get_operation();
            return Ok(op);
        }
        // Handle integer negation.
        Opcode::Ineg => {
            let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
            let ineg_op = InegOp::new(ctx, operands[0]);
            let op = ineg_op.get_operation();
            return Ok(op);
        }
        // Handle integer absolute value.
        Opcode::Iabs => {
            let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
            let iabs_op = IabsOp::new(ctx, operands[0]);
            let op = iabs_op.get_operation();
            return Ok(op);
        }
        // Handle return instructions.
        Opcode::Return => match inst_args.len() {
            // Return with no operands.
            0 => {
                let return_op = ReturnOp::new(ctx, None);
                let op = return_op.get_operation();
                return Ok(op);
            }
            // Return with a single operand.
            1 => {
                let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
                let return_op = ReturnOp::new(ctx, Some(operands[0]));
                let op = return_op.get_operation();
                return Ok(op);
            }
            // Multiple return values are not supported.
            _ => unimplemented!("Multiple return values are not supported"),
        },
        // For any unsupported opcode, panic with an error message.
        _ => unimplemented!("Opcode {} is not implemented", clif_opcode),
    }
}

/// Converts a Cranelift basic block ([ClifBasicBlock]) to a Pliron basic block ([BasicBlock]).
///
/// This function checks the conversion cache; if the block is not yet converted, it creates a new
/// Pliron basic block using a generated label and by converting the argument types of the block.
///
/// # Arguments
/// - `ctx`: The Pliron context used for creating IR entities.
/// - `dfg`: The Cranelift Data Flow Graph holding block parameter information.
/// - `cctx`: The conversion context for caching converted basic blocks.
/// - `block`: The Cranelift basic block to convert.
///
/// # Returns
/// A Result containing a pointer to the converted [BasicBlock] or an error if conversion fails.
fn convert_block(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    cctx: &mut ConversionCtx,
    block: ClifBasicBlock,
) -> Result<Ptr<BasicBlock>> {
    // Check if the block has already been converted
    if let Some(bb) = cctx.bbs.get(&block) {
        return Ok(*bb);
    };

    // Create a new Pliron `BasicBlock` with a label and argument types
    let label = Identifier::try_new(format!("{}", block))?;
    let block_params = dfg.block_params(block);
    let mut arg_types = vec![];
    for param in block_params {
        let param_type = dfg.value_type(*param);
        let pliron_type = convert_type(ctx, param_type)?;
        arg_types.push(pliron_type);
    }

    let pliron_block = BasicBlock::new(ctx, Some(label), arg_types);
    Ok(pliron_block)
}

/// Converts a Cranelift type ([ClifType]) to a Pliron type ([TypeObj]).
///
/// This function currently supports conversion for `i32` only, mapping it to a 32-bit signed integer type.
/// 
/// # Arguments
/// - `ctx`: The Pliron context used for creating type objects.
/// - `ty`: The Cranelift type to convert.
///
/// # Returns
/// A Result containing a pointer to the converted [TypeObj] or an error if conversion fails.
///
/// # Panics
/// Panics if the provided type is not yet implemented.
fn convert_type(ctx: &mut Context, ty: ClifType) -> Result<Ptr<TypeObj>> {
    match ty.to_string().as_str() {
        "i32" => {
            let pliron_int_type_ptr = IntegerType::get(ctx, 32, Signedness::Signed);
            return Ok(pliron_int_type_ptr.to_ptr());
        }
        _ => unimplemented!("Type {:?} is not implemented", ty),
    }
}

/// Converts a Cranelift function ([Function]) to a Pliron function operation ([FuncOp]).
///
/// This function translates the function signature, entry block, and all blocks and instructions,
/// storing converted entities in the conversion context for caching and linking.
///
/// # Arguments
/// - `ctx`: The Pliron context used for IR creation.
/// - `cctx`: The conversion context used to cache converted entities.
/// - `func`: The Cranelift function to convert.
///
/// # Returns
/// A Result containing the converted [FuncOp] or an error if conversion fails.
///
/// # Panics
/// Panics if the function's entry block or types cannot be converted.
fn convert_function(ctx: &mut Context, cctx: &mut ConversionCtx, func: Function) -> Result<FuncOp> {
    // Helper function to convert and link blocks and instructions within the function.
    fn convert_and_link(ctx: &mut Context, cctx: &mut ConversionCtx, func: Function) {
        let dfg = &func.dfg;
        let mut prev_bb = match cctx.entry_block {
            Some(entry) => entry,
            None => return,
        };
        for (idx, block) in func.layout.blocks().enumerate() {
            let bb = convert_block(ctx, &dfg, cctx, block).unwrap();
            match idx {
                0 => {
                    // the entry block should already be linked and inserted into context.
                }
                _ => {
                    bb.insert_after(ctx, prev_bb);
                    cctx.bbs.insert(block, bb);
                    prev_bb = bb;
                }
            }
            // A Clif layout (i.e., Clif Layout type) is NOT an RPO-ordered list of blocks.
            // It maintains the insertion order of blocks. Keeping this in mind,
            // we ensure that while iterating in insertion order, every block and instruction
            // is always checked for prior conversion.
            let mut prev_inst = None;
            for (idx, inst) in func.layout.block_insts(block).enumerate() {
                let op = convert_instruction(ctx, &dfg, cctx, inst).unwrap();
                match idx {
                    0 => {
                        op.insert_at_front(bb, ctx);
                        cctx.ops.insert(inst, op);
                        prev_inst = Some(inst);
                    }
                    _ => match prev_inst {
                        Some(prev_ins) => {
                            let prev_op = cctx.ops.get(&prev_ins).unwrap();
                            op.insert_after(ctx, *prev_op);
                            cctx.ops.insert(inst, op);
                            prev_inst = Some(inst);
                        }
                        None => todo!(),
                    },
                }
            }
        }
    }

    // Convert function signature (name, params, return types)
    let func_name = func.name.to_string().split_off(1);
    let func_type = func.signature.clone();
    let func_params_types: Vec<_> = func_type
        .params
        .iter()
        .map(|param| {
            convert_type(ctx, param.value_type)
                .expect("Failed to convert Clif Function Parameter Type")
        })
        .collect();
    let func_return_types: Vec<_> = func_type
        .returns
        .iter()
        .map(|ret| {
            convert_type(ctx, ret.value_type).expect("Failed to convert Clif Function Return Type")
        })
        .collect();
    let pliron_func_type = FunctionType::get(ctx, func_params_types, func_return_types);

    // Create the Pliron `FuncOp`
    let func_op = FuncOp::new(ctx, &Identifier::try_new(func_name)?, pliron_func_type);

    // Update the conversion context
    let pliron_entry_blk = func_op.get_entry_block(ctx);
    if let Some(blk) = func.layout.entry_block() {
        cctx.bbs.insert(blk, pliron_entry_blk);
    } else {
        println!("Function has no entry block");
    }
    cctx.entry_block = Some(pliron_entry_blk);
    cctx.regs.push(func_op.get_region(ctx));
    cctx.func_op = Some(func_op.get_operation());

    // Convert and link all blocks and instructions
    convert_and_link(ctx, cctx, func);

    Ok(func_op)
}

/// Converts operands in reverse postorder (RPO).
///
/// With RPO, we assume that any operand's definition has already been converted,
/// so we can safely unwrap conversion results.
///
/// # Arguments
/// - `dfg`: The Cranelift data flow graph.
/// - `cctx`: The conversion context caching converted entities.
/// - `operands`: The slice of Cranelift values to convert.
///
/// # Returns
/// A Result containing a vector of converted Pliron values.
fn convert_operands_w_rpo(
    dfg: &DataFlowGraph,
    cctx: &mut ConversionCtx,
    operands: &[ClifValue],
) -> Result<Vec<PlironValue>> {
    let mut pliron_operands = Vec::with_capacity(operands.len());
    for operand in operands {
        let operand_def = dfg.value_def(*operand);
        match operand_def {
            // Is this a value defined by an instruction? If so, convert instruction result
            ValueDef::Result(inst, idx) => {
                // With RPO, a definition is always converted before its use.
                // Therefore, unwrapping is safe here—when we encounter a use (as an operand),
                // we can be certain that its definition has already been converted.
                let op = cctx.ops.get(&inst).unwrap();
                let pliron_value = PlironValue::OpResult {
                    op: *op,
                    res_idx: idx,
                };
                pliron_operands.push(pliron_value);
            }
            // Is this value a BasicBlock parameter? If so, convert block parameter
            ValueDef::Param(block, idx) => {
                // With RPO, a definition is always converted before its use.
                // Therefore, unwrapping is safe here—when we encounter a use (as an operand),
                // we can be certain that its definition has already been converted.
                let block = cctx.bbs.get(&block).unwrap();
                let pliron_value = PlironValue::BlockArgument {
                    block: *block,
                    arg_idx: idx,
                };
                pliron_operands.push(pliron_value);
            }
            ValueDef::Union(_value, _value1) => todo!(),
        }
    }
    Ok(pliron_operands)
}

/// Converts a Cranelift instruction ([Inst]) to a Pliron operation using RPO assumptions.
///
/// This version of conversion assumes that operand definitions have already been converted,
/// allowing safe unwrapping when retrieving operands.
///
/// # Arguments
/// - `ctx`: The Pliron context for IR creation.
/// - `dfg`: The Cranelift data flow graph.
/// - `cctx`: The conversion context caching converted entities.
/// - `inst`: The Cranelift instruction to convert.
///
/// # Returns
/// A Result containing a pointer to the converted [Operation].
fn convert_instruction_w_rpo(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    cctx: &mut ConversionCtx,
    inst: Inst,
) -> Result<Ptr<Operation>> {
    // Retrieve the opcode and arguments for the instruction.
    let clif_opcode = dfg.insts[inst].opcode();
    let inst_args = dfg.inst_args(inst);
    match clif_opcode {
        // Handle integer addition.
        Opcode::Iadd => {
            let operands = convert_operands_w_rpo(dfg, cctx, &inst_args)?;
            let iadd_op = IAddOp::new(ctx, operands[0], operands[1]);
            let op = iadd_op.get_operation();
            return Ok(op);
        }
        // Handle integer subtraction.
        Opcode::Isub => {
            let operands = convert_operands_w_rpo(dfg, cctx, &inst_args)?;
            let isub_op = ISubOp::new(ctx, operands[0], operands[1]);
            let op = isub_op.get_operation();
            return Ok(op);
        }
        // Handle unsigned minimum.
        Opcode::Umin => {
            let operands = convert_operands_w_rpo(dfg, cctx, &inst_args)?;
            let umin_op = UminOp::new(ctx, operands[0], operands[1]);
            let op = umin_op.get_operation();
            return Ok(op);
        }
        // Handle unsigned maximum.
        Opcode::Umax => {
            let operands = convert_operands_w_rpo(dfg, cctx, &inst_args)?;
            let umax_op = UmaxOp::new(ctx, operands[0], operands[1]);
            let op = umax_op.get_operation();
            return Ok(op);
        }
        // Handle integer negation.
        Opcode::Ineg => {
            let operands = convert_operands_w_rpo(dfg, cctx, &inst_args)?;
            let ineg_op = InegOp::new(ctx, operands[0]);
            let op = ineg_op.get_operation();
            return Ok(op);
        }
        // Handle integer absolute value.
        Opcode::Iabs => {
            let operands = convert_operands_w_rpo(dfg, cctx, &inst_args)?;
            let iabs_op = IabsOp::new(ctx, operands[0]);
            let op = iabs_op.get_operation();
            return Ok(op);
        }
        // Handle return instructions.
        Opcode::Return => match inst_args.len() {
            // Return with no operands.
            0 => {
                let return_op = ReturnOp::new(ctx, None);
                let op = return_op.get_operation();
                return Ok(op);
            }
            // Return with a single operand.
            1 => {
                let operands = convert_operands_w_rpo(dfg, cctx, &inst_args)?;
                let return_op = ReturnOp::new(ctx, Some(operands[0]));
                let op = return_op.get_operation();
                return Ok(op);
            }
            // Multiple return values are not supported.
            _ => unimplemented!("Multiple return values are not supported"),
        },
        // For any unsupported opcode, panic with an error message.
        _ => unimplemented!("Opcode {} is not implemented", clif_opcode),
    }
}

/// Converts a Cranelift basic block to a Pliron basic block using RPO conversion.
///
/// This version does not use the conversion cache since RPO ordering ensures that operands are already
/// converted in the proper order.
///
/// # Arguments
/// - `ctx`: The Pliron context for IR creation.
/// - `dfg`: The Cranelift data flow graph.
/// - `block`: The Cranelift basic block to convert.
///
/// # Returns
/// A Result containing a pointer to the converted [BasicBlock].
fn convert_block_w_rpo(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    block: ClifBasicBlock,
) -> Result<Ptr<BasicBlock>> {
    // Create a new Pliron `BasicBlock` with a label and argument types
    let label = Identifier::try_new(format!("{}", block))?;
    let block_params = dfg.block_params(block);
    let mut arg_types = vec![];
    for param in block_params {
        let param_type = dfg.value_type(*param);
        let pliron_type = convert_type(ctx, param_type)?;
        arg_types.push(pliron_type);
    }

    let pliron_block = BasicBlock::new(ctx, Some(label), arg_types);
    Ok(pliron_block)
}

/// Converts a Cranelift function to a Pliron function operation using Reverse Postorder (RPO).
///
/// This function is similar to `convert_function` but leverages RPO ordering,
/// which guarantees that definitions are processed before their uses.
///
/// # Arguments
/// - `ctx`: The Pliron context for IR creation.
/// - `cctx`: The conversion context caching converted entities.
/// - `func`: The Cranelift function to convert.
///
/// # Returns
/// A Result containing the converted [FuncOp] or an error if conversion fails.
fn convert_function_w_rpo(
    ctx: &mut Context,
    cctx: &mut ConversionCtx,
    func: Function,
) -> Result<FuncOp> {
    // Helper function to convert and link blocks and instructions within the function.
    fn convert_and_link(ctx: &mut Context, cctx: &mut ConversionCtx, func: Function) {
        let dfg = &func.dfg;
        let cfg = ControlFlowGraph::with_function(&func);
        let dom_tree = DominatorTree::with_function(&func, &cfg);
        let rpo = dom_tree.cfg_rpo();

        let mut prev_bb = match cctx.entry_block {
            Some(entry) => entry,
            None => return,
        };
        // RPO ordered list of blocks
        for (idx, block) in rpo.enumerate() {
            let mut bb = prev_bb;
            match idx {
                0 => {
                    // the entry block should already be linked and inserted into context.
                }
                _ => {
                    bb = convert_block_w_rpo(ctx, &dfg, *block).unwrap();
                    bb.insert_after(ctx, prev_bb);
                    cctx.bbs.insert(*block, bb);
                    prev_bb = bb;
                }
            }
            let mut prev_inst = None;
            for (idx, inst) in func.layout.block_insts(*block).enumerate() {
                let op = convert_instruction_w_rpo(ctx, &dfg, cctx, inst).unwrap();
                match idx {
                    0 => {
                        op.insert_at_front(bb, ctx);
                        cctx.ops.insert(inst, op);
                        prev_inst = Some(inst);
                    }
                    _ => match prev_inst {
                        Some(prev_ins) => {
                            let prev_op = cctx.ops.get(&prev_ins).unwrap();
                            op.insert_after(ctx, *prev_op);
                            cctx.ops.insert(inst, op);
                            prev_inst = Some(inst);
                        }
                        None => todo!(),
                    },
                }
            }
        }
    }

    // Convert function signature (name, params, return types)
    let func_name = func.name.to_string().split_off(1);
    let func_type = func.signature.clone();
    let func_params_types: Vec<_> = func_type
        .params
        .iter()
        .map(|param| {
            convert_type(ctx, param.value_type)
                .expect("Failed to convert Clif Function Parameter Type")
        })
        .collect();
    let func_return_types: Vec<_> = func_type
        .returns
        .iter()
        .map(|ret| {
            convert_type(ctx, ret.value_type).expect("Failed to convert Clif Function Return Type")
        })
        .collect();
    let pliron_func_type = FunctionType::get(ctx, func_params_types, func_return_types);

    // Create the Pliron `FuncOp`
    let func_op = FuncOp::new(ctx, &Identifier::try_new(func_name)?, pliron_func_type);

    // Update the conversion context:
    // Map the Cranelift entry block to the Pliron entry block,
    // store the function's region and the function op.
    let pliron_entry_blk = func_op.get_entry_block(ctx);
    if let Some(blk) = func.layout.entry_block() {
        cctx.bbs.insert(blk, pliron_entry_blk);
    } else {
        println!("Function has no entry block");
    }
    cctx.entry_block = Some(pliron_entry_blk);
    cctx.regs.push(func_op.get_region(ctx));
    cctx.func_op = Some(func_op.get_operation());

    // Convert and link all blocks and instructions
    convert_and_link(ctx, cctx, func);

    Ok(func_op)
}
/// Tracks converted Pliron entities during IR transformation.
///
/// This structure caches converted operations, regions, and basic blocks,
/// ensuring efficient lookups and preventing redundant conversions.
///
/// # Fields
/// - `ops`: Maps Cranelift instructions (Inst) to their corresponding Pliron [Operation] pointers.
/// - `regs`: Stores pointers to converted [Region] entities.
/// - `bbs`: Maps Cranelift basic blocks to their corresponding Pliron [BasicBlock] pointers.
/// - `entry_block`: The entry [BasicBlock] of the function being processed.
/// - `func_op`: The root [Operation] representing the function being processed.
#[derive(Default)]
struct ConversionCtx {
    ops: FxHashMap<Inst, Ptr<Operation>>,
    regs: Vec<Ptr<Region>>,
    bbs: FxHashMap<ClifBasicBlock, Ptr<BasicBlock>>,
    entry_block: Option<Ptr<BasicBlock>>,
    func_op: Option<Ptr<Operation>>,
}

#[cfg(test)]
mod tests {
    // Import required items for testing.
    use super::*;
    use cranelift_reader::parse_functions;
    use pliron::{builtin, printable::Printable};

    /// Test converting a Cranelift function that adds two i32 values to a Pliron function.
    #[test]
    fn test_add_fn_convert_clif_to_pliron() {
        let clif_code = r#"
        function %add(i32, i32) -> i32 apple_aarch64 {
            block0(v0: i32, v1: i32):
                v2 = iadd v0, v1
                return v2
        }
    "#;

        // Parse the CLIF code to obtain Cranelift functions.
        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        // Process each function.
        for func in functions {
            let mut cctx = ConversionCtx::default();
            let mut ctx = Context::new();
            // Register built-in and custom dialects/ops.
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            // Convert the Cranelift function to a Pliron function.
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
                Ok(op) => op,
                Err(e) => panic!("Error: {}", e),
            };
            // Print the resulting function.
            let print_func = func_op.disp(&ctx);
            println!("{}", print_func);
            // Assert that the printed function matches the expected output.
            assert_eq!(
                            "builtin.func @add: builtin.function <(builtin.int <si32>, builtin.int <si32>)->(builtin.int <si32>)> 
{
  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>,block_1v1_arg1:builtin.int <si32>):
    op_2v1_res0 = clif.iadd block_1v1_arg0,block_1v1_arg1:builtin.int <si32>;
    clif.return (op_2v1_res0)
}",
                            format!("{}", print_func)
                        );
        }
    }

    /// Test converting a Cranelift function to Pliron using RPO ordering.
    #[test]
    fn test_add_fn_convert_clif_to_pliron_w_rpo() {
        let clif_code = r#"
        function %add(i32, i32) -> i32 apple_aarch64 {
            block0(v0: i32, v1: i32):
                v2 = iadd v0, v1
                return v2
        }
    "#;

        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        for func in functions {
            let mut cctx = ConversionCtx::default();
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            let func_op = match convert_function_w_rpo(&mut ctx, &mut cctx, func) {
                Ok(op) => op,
                Err(e) => panic!("Error: {}", e),
            };
            let print_func = func_op.disp(&ctx);
            println!("{}", print_func);
            assert_eq!("builtin.func @add: builtin.function <(builtin.int <si32>, builtin.int <si32>)->(builtin.int <si32>)> 
{
  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>,block_1v1_arg1:builtin.int <si32>):
    op_2v1_res0 = clif.iadd block_1v1_arg0,block_1v1_arg1:builtin.int <si32>;
    clif.return (op_2v1_res0)
}", format!("{}", print_func)
            );
        }
    }

    /// Test converting a Cranelift function that implements unsigned minimum (umin) to Pliron.
    #[test]
    fn test_umin_fn_convert_clif_to_pliron_w_rpo() {
        let clif_code = r#"
    function %umin(i32, i32) -> i32 apple_aarch64 {
        block0(v0: i32, v1: i32):
            v2 = umin v0, v1
            return v2
    }
    "#;

        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        for func in functions {
            let mut cctx = ConversionCtx::default();
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            let func_op = match convert_function_w_rpo(&mut ctx, &mut cctx, func) {
                Ok(op) => op,
                Err(e) => panic!("Error: {}", e),
            };
            let print_func = func_op.disp(&ctx);
            println!("{}", print_func);
            assert_eq!(
            "builtin.func @umin: builtin.function <(builtin.int <si32>, builtin.int <si32>)->(builtin.int <si32>)> 
{
  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>,block_1v1_arg1:builtin.int <si32>):
    op_2v1_res0 = clif.umin block_1v1_arg0,block_1v1_arg1:builtin.int <si32>;
    clif.return (op_2v1_res0)
}",
            format!("{}", print_func)
        );
        }
    }

    /// Test converting a Cranelift function that implements integer negation (ineg) to Pliron.
    #[test]
    fn test_ineg_fn_convert_clif_to_pliron_w_rpo() {
        let clif_code = r#"
    function %ineg(i32) -> i32 apple_aarch64 {
        block0(v0: i32):
            v1 = ineg v0
            return v1
    }
    "#;

        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        for func in functions {
            let mut cctx = ConversionCtx::default();
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            let func_op = match convert_function_w_rpo(&mut ctx, &mut cctx, func) {
                Ok(op) => op,
                Err(e) => panic!("Error: {}", e),
            };
            let print_func = func_op.disp(&ctx);
            println!("{}", print_func);
            assert_eq!(
                "builtin.func @ineg: builtin.function <(builtin.int <si32>)->(builtin.int <si32>)> 
{
  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>):
    op_2v1_res0 = clif.ineg block_1v1_arg0;
    clif.return (op_2v1_res0)
}",
                format!("{}", print_func)
            );
        }
    }

    /// Test converting a Cranelift function that implements unsigned maximum (umax) to Pliron.
    #[test]
    fn test_umax_fn_convert_clif_to_pliron_w_rpo() {
        let clif_code = r#"
    function %umax(i32, i32) -> i32 apple_aarch64 {
        block0(v0: i32, v1: i32):
            v2 = umax v0, v1
            return v2
    }
    "#;

        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        for func in functions {
            let mut cctx = ConversionCtx::default();
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            let func_op = match convert_function_w_rpo(&mut ctx, &mut cctx, func) {
                Ok(op) => op,
                Err(e) => panic!("Error: {}", e),
            };
            let print_func = func_op.disp(&ctx);
            println!("{}", print_func);
            assert_eq!(
            "builtin.func @umax: builtin.function <(builtin.int <si32>, builtin.int <si32>)->(builtin.int <si32>)> 
{
  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>,block_1v1_arg1:builtin.int <si32>):
    op_2v1_res0 = clif.umax block_1v1_arg0,block_1v1_arg1:builtin.int <si32>;
    clif.return (op_2v1_res0)
}",
            format!("{}", print_func)
        );
        }
    }

     /// Test converting a Cranelift function that implements integer subtraction (isub) to Pliron.
     #[test]
     fn test_sub_fn_convert_clif_to_pliron_w_rpo() {
         let clif_code = r#"
         function %sub(i32, i32) -> i32 apple_aarch64 {
             block0(v0: i32, v1: i32):
                 v2 = isub v0, v1
                 return v2
         }
     "#;
 
         let functions = parse_functions(clif_code).expect("Failed to parse .clif");
 
         for func in functions {
             let mut cctx = ConversionCtx::default();
             let mut ctx = Context::new();
             builtin::register(&mut ctx);
             crate::register(&mut ctx);
             let func_op = match convert_function_w_rpo(&mut ctx, &mut cctx, func) {
                 Ok(op) => op,
                 Err(e) => panic!("Error: {}", e),
             };
             let print_func = func_op.disp(&ctx);
             println!("{}", print_func);
             assert_eq!(
                 "builtin.func @sub: builtin.function <(builtin.int <si32>, builtin.int <si32>)->(builtin.int <si32>)> \n{\
 \n  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>,block_1v1_arg1:builtin.int <si32>):\
 \n    op_2v1_res0 = clif.isub block_1v1_arg0,block_1v1_arg1:builtin.int <si32>;\
 \n    clif.return (op_2v1_res0)\
 \n}",
                 format!("{}", print_func)
             );
         }
     }
 
     /// Test converting a Cranelift function that implements integer absolute value (iabs) to Pliron.
     #[test]
     fn test_abs_fn_convert_clif_to_pliron_w_rpo() {
         let clif_code = r#"
         function %abs(i32) -> i32 apple_aarch64 {
             block0(v0: i32):
                 v1 = iabs v0
                 return v1
         }
     "#;
 
         let functions = parse_functions(clif_code).expect("Failed to parse .clif");
 
         for func in functions {
             let mut cctx = ConversionCtx::default();
             let mut ctx = Context::new();
             builtin::register(&mut ctx);
             crate::register(&mut ctx);
             let func_op = match convert_function_w_rpo(&mut ctx, &mut cctx, func) {
                 Ok(op) => op,
                 Err(e) => panic!("Error: {}", e),
             };
             let print_func = func_op.disp(&ctx);
             println!("{}", print_func);
             assert_eq!(
                 "builtin.func @abs: builtin.function <(builtin.int <si32>)->(builtin.int <si32>)> \n{\
 \n  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>):\
 \n    op_2v1_res0 = clif.iabs block_1v1_arg0;\
 \n    clif.return (op_2v1_res0)\
 \n}",
                 format!("{}", print_func)
             );
         }
     }
 
}
