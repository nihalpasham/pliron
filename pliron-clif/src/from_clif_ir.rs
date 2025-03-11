// Import required types and modules from Pliron and Cranelift.
// Pliron imports: Basic IR entities such as context, operations, regions, types, etc.
use pliron::{
    basic_block::BasicBlock,
    builtin::{
        op_interfaces::{CallOpCallable, OneRegionInterface},
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
        types::Type as ClifType, Block as ClifBasicBlock, DataFlowGraph, Function, Inst,
        InstructionData, Opcode, Value as ClifValue, ValueDef,
    },
};
// Import a fast hash map from rustc_hash crate.
use rustc_hash::FxHashMap;

// Import our own modules that define op interfaces and operations.
use crate::{
    op_interfaces::{BinArithOp, UnaryArithOp},
    ops::{BrifOp, CallOp, IAddOp, ISubOp, IabsOp, InegOp, JumpOp, ReturnOp, UmaxOp, UminOp},
};

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

/// Converts operands with reverse postorder (RPO) assumptions.
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
fn convert_operands(
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
fn convert_instruction(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    cctx: &mut ConversionCtx,
    inst: Inst,
) -> Result<Ptr<Operation>> {
    // Retrieve the instruction data for the given instruction.
    let inst_data = dfg.insts[inst];

    match inst_data {
        InstructionData::Brif { arg, blocks, .. } => {
            // Convert a conditional branch instruction (`brif`).
            let clif_then_block = blocks[0].block(&dfg.value_lists);
            let clif_then_block_args = blocks[0].args_slice(&dfg.value_lists);

            let clif_else_block = blocks[1].block(&dfg.value_lists);
            let clif_else_block_args = blocks[1].args_slice(&dfg.value_lists);

            let cond = convert_operands(dfg, cctx, &[arg])?;

            let true_dest = convert_block(ctx, cctx, dfg, clif_then_block)?;
            let true_dest_opds = convert_operands(dfg, cctx, &clif_then_block_args)?;

            let false_dest = convert_block(ctx, cctx, dfg, clif_else_block)?;
            let false_dest_opds = convert_operands(dfg, cctx, &clif_else_block_args)?;

            let brif_op = BrifOp::new(
                ctx,
                cond[0],
                true_dest,
                true_dest_opds,
                false_dest,
                false_dest_opds,
            );
            let op = brif_op.get_operation();
            return Ok(op);
        }
        InstructionData::BranchTable { .. } => {
            // Branch table (`br_table`) instructions, used for jump tables, are not yet implemented.
            todo!()
        }
        InstructionData::Jump { destination, .. } => {
            // Convert a direct jump instruction.
            // Extract the destination block and its arguments.
            let clif_block = destination.block(&dfg.value_lists);
            let clif_block_args = destination.args_slice(&dfg.value_lists);

            let dest = convert_block(ctx, cctx, dfg, clif_block)?;
            let dest_opds = convert_operands(dfg, cctx, &clif_block_args)?;

            let jump_op = JumpOp::new(ctx, dest, dest_opds);
            let op = jump_op.get_operation();
            return Ok(op);
        }
        InstructionData::Call { args, func_ref, .. } => {
            // Convert a direct call instruction. (i.e. a call to a statically known function)
            // Extract the function arguments and convert them.
            let clif_args_slice = args.as_slice(&dfg.value_lists);
            let args = convert_operands(dfg, cctx, &clif_args_slice)?;

            // Extract the function name and instantiate a CallOpCallable. (Direct call)
            let ext_name = dfg.ext_funcs[func_ref].name.clone();
            let displayable_name = ext_name.display(None).to_string().split_off(1);
            let callee = CallOpCallable::Direct(
                Identifier::try_new(format!("{}", displayable_name)).unwrap(),
            );
            // Extract the function signature and convert to Pliron `FunctionType`.
            let sig_ref = dfg.ext_funcs[func_ref].signature.clone();
            let func_sig = dfg.signatures[sig_ref].clone();

            // Convert function signature (name, params, return types)
            let func_params_types: Vec<_> = func_sig
                .params
                .iter()
                .map(|param| {
                    convert_type(ctx, param.value_type)
                        .expect("Failed to convert Clif Function Parameter Type")
                })
                .collect();
            let func_return_types: Vec<_> = func_sig
                .returns
                .iter()
                .map(|ret| {
                    convert_type(ctx, ret.value_type)
                        .expect("Failed to convert Clif Function Return Type")
                })
                .collect();
            let callee_ty = FunctionType::get(ctx, func_params_types, func_return_types);

            let call_op = CallOp::new(ctx, callee, callee_ty, args);
            let op = call_op.get_operation();
            return Ok(op);
        }
        _ => {
            // Retrieve the opcode and value arguments for the instruction.
            let clif_opcode = dfg.insts[inst].opcode();
            let inst_args = dfg.inst_args(inst);
            match clif_opcode {
                // Handle integer addition.
                Opcode::Iadd => {
                    let operands = convert_operands(dfg, cctx, &inst_args)?;
                    let iadd_op = IAddOp::new(ctx, operands[0], operands[1]);
                    let op = iadd_op.get_operation();
                    return Ok(op);
                }
                // Handle integer subtraction.
                Opcode::Isub => {
                    let operands = convert_operands(dfg, cctx, &inst_args)?;
                    let isub_op = ISubOp::new(ctx, operands[0], operands[1]);
                    let op = isub_op.get_operation();
                    return Ok(op);
                }
                // Handle unsigned minimum.
                Opcode::Umin => {
                    let operands = convert_operands(dfg, cctx, &inst_args)?;
                    let umin_op = UminOp::new(ctx, operands[0], operands[1]);
                    let op = umin_op.get_operation();
                    return Ok(op);
                }
                // Handle unsigned maximum.
                Opcode::Umax => {
                    let operands = convert_operands(dfg, cctx, &inst_args)?;
                    let umax_op = UmaxOp::new(ctx, operands[0], operands[1]);
                    let op = umax_op.get_operation();
                    return Ok(op);
                }
                // Handle integer negation.
                Opcode::Ineg => {
                    let operands = convert_operands(dfg, cctx, &inst_args)?;
                    let ineg_op = InegOp::new(ctx, operands[0]);
                    let op = ineg_op.get_operation();
                    return Ok(op);
                }
                // Handle integer absolute value.
                Opcode::Iabs => {
                    let operands = convert_operands(dfg, cctx, &inst_args)?;
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
                        let operands = convert_operands(dfg, cctx, &inst_args)?;
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
    }
}

/// Converts a Cranelift basic block to a Pliron basic block.
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
fn convert_block(
    ctx: &mut Context,
    cctx: &mut ConversionCtx,
    dfg: &DataFlowGraph,
    block: ClifBasicBlock,
) -> Result<Ptr<BasicBlock>> {
    // Check if we have already converted this block.
    if let Some(pliron_block) = cctx.bbs.get(&block) {
        return Ok(*pliron_block);
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
    // Update the conversion context with the newly converted block
    cctx.bbs.insert(block, pliron_block);
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
fn convert_function(ctx: &mut Context, cctx: &mut ConversionCtx, func: Function) -> Result<FuncOp> {
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
                    // the entry block should already be linked and inserted into the ConversionCtx.
                }
                _ => {
                    // safe to unwrap since we are iterating in RPO order.
                    // So, we can be certain that blocks have already been converted and cached.
                    bb = *cctx.bbs.get(&block).unwrap();
                    bb.insert_after(ctx, prev_bb);
                    prev_bb = bb;
                }
            }
            let mut prev_inst = None;
            for (idx, inst) in func.layout.block_insts(*block).enumerate() {
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
                        None => unreachable!(),
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
        unreachable!("Clif functions must possess an entry block");
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

    /// Test converting a Cranelift function to Pliron using RPO ordering.
    #[test]
    fn test_add_fn_convert_clif_to_pliron() {
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
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
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
    fn test_umin_fn_convert_clif_to_pliron() {
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
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
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
    fn test_ineg_fn_convert_clif_to_pliron() {
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
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
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
    fn test_umax_fn_convert_clif_to_pliron() {
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
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
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
    fn test_sub_fn_convert_clif_to_pliron() {
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
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
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
    fn test_abs_fn_convert_clif_to_pliron() {
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
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
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

    /// Test converting a Cranelift function that implements multiple unconditional branches or `jump`s to Pliron.
    #[test]
    fn test_jump_fn_convert_clif_to_pliron() {
        let clif_code = r#"
         function %jump(i32) -> i32 {
            block0(v0: i32):
                v1 = iadd v0, v0
                jump block1(v1)

            block1(v2: i32):
                jump block2(v2)
            
            block2(v3: i32):
                return v3
        }
     "#;

        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        for func in functions {
            let mut cctx = ConversionCtx::default();
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
                Ok(op) => op,
                Err(e) => panic!("Error: {}", e),
            };
            let print_func = func_op.disp(&ctx);
            println!("{}", print_func);
            assert_eq!(
                "builtin.func @jump: builtin.function <(builtin.int <si32>)->(builtin.int <si32>)> 
{
  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>):
    op_2v1_res0 = clif.iadd block_1v1_arg0,block_1v1_arg0:builtin.int <si32>;
    clif.jump ^block1_block_2v1(op_2v1_res0)
  ^block1_block_2v1(block_2v1_arg0:builtin.int <si32>):
    clif.jump ^block2_block_3v1(block_2v1_arg0)
  ^block2_block_3v1(block_3v1_arg0:builtin.int <si32>):
    clif.return (block_3v1_arg0)
}",
                format!("{}", print_func)
            );
        }
    }

    /// Test converting a Cranelift function that implements conditional branches or `brif`s to Pliron.
    ///
    #[test]
    fn test_brif_fn_convert_clif_to_pliron() {
        let clif_code = r#"
        function %brif(i32, i32, i32) -> i32 {
            block0(v0: i32, v1: i32, v2: i32):
                brif v0, block1(v1), block2(v2)

            block1(v3: i32):
                jump block2(v3)

            block2(v4: i32):
                return v4
        }
     "#;

        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        for func in functions {
            let mut cctx = ConversionCtx::default();
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
                Ok(op) => op,
                Err(e) => panic!("Error: {}", e),
            };
            let print_func = func_op.disp(&ctx);
            println!("{}", print_func);
            assert_eq!(
                "builtin.func @brif: builtin.function <(builtin.int <si32>, builtin.int <si32>, builtin.int <si32>)->(builtin.int <si32>)> 
{
  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>,block_1v1_arg1:builtin.int <si32>,block_1v1_arg2:builtin.int <si32>):
    clif.brif if block_1v1_arg0 ^block1_block_2v1(block_1v1_arg1) else ^block2_block_3v1(block_1v1_arg2)
  ^block1_block_2v1(block_2v1_arg0:builtin.int <si32>):
    clif.jump ^block2_block_3v1(block_2v1_arg0)
  ^block2_block_3v1(block_3v1_arg0:builtin.int <si32>):
    clif.return (block_3v1_arg0)
}",
                format!("{}", print_func)
            );
        }
    }

    /// Test converting a Cranelift function that implements a `direct call`
    /// (for statically known functions) to Pliron.
    #[test]
    fn test_call_fn_convert_clif_to_pliron() {
        let clif_code = r#"
        function %call(i32) -> i32 {
            fn0 = %g(i32) -> i32

            block0(v0: i32):
                v1 = call fn0(v0)
                return v1
        }
     "#;

        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        for func in functions {
            let mut cctx = ConversionCtx::default();
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            let func_op = match convert_function(&mut ctx, &mut cctx, func) {
                Ok(op) => op,
                Err(e) => panic!("Error: {}", e),
            };
            let print_func = func_op.disp(&ctx);
            println!("{}", print_func);
            // assert_eq!(
            //     "",
            //     format!("{}", print_func)
            // );
        }
    }
}
