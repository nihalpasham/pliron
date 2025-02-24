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

use cranelift_codegen::{
    dominator_tree::DominatorTree,
    flowgraph::ControlFlowGraph,
    ir::{
        types::Type as ClifType, Block as ClifBasicBlock, DataFlowGraph, Function, Inst, Opcode,
        Value as ClifValue, ValueDef,
    },
};
use rustc_hash::FxHashMap;

use crate::{
    op_interfaces::BinArithOp,
    ops::{IAddOp, ReturnOp},
};

/// Converts a slice of [ClifValue]s to Pliron's [PlironValue]s.
///
/// This function processes each operand, determining if it is defined by an instruction or
/// a block parameter, and converts it into the corresponding `PlironValue`.
///
/// # Arguments
/// - `ctx`: The Pliron context for creating IR entities.
/// - `dfg`: The data flow graph containing the original operand definitions.
/// - `cctx`: The conversion context for caching converted entities.
/// - `operands`: The slice of `ClifValue` to convert.
///
/// # Returns
/// A `Result` containing a vector of converted `PlironValue` or an error if conversion fails.
///
/// # Panics
/// Panics if a `Union` value definition is encountered, as it is not yet implemented.
fn convert_operands(
    ctx: &mut Context,
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
                let op = convert_instruction(ctx, dfg, cctx, inst)?;
                let pliron_value = PlironValue::OpResult { op, res_idx: idx };
                pliron_operands.push(pliron_value);
            }
            // Is this value a BasicBlock parameter? If so, convert block parameter
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

/// Converts a Cranelift [Inst] to Pliron's [Ptr<Operation>].
///
/// This function checks if the instruction has already been converted (using `cctx` for caching).
/// If not, it converts the instruction based on its opcode (like `Iadd` and `Return`).
///
/// # Arguments
/// - `ctx`: The Pliron context for creating IR entities.
/// - `dfg`: The data flow graph containing the original instruction's information.
/// - `cctx`: The conversion context for caching converted entities.
/// - `inst`: The Cranelift `Inst` to convert.
///
/// # Returns
/// A `Result` containing the converted `Operation` or an error if conversion fails.
///
/// # Panics
/// Panics if the opcode is not yet implemented
fn convert_instruction(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    cctx: &mut ConversionCtx,
    inst: Inst,
) -> Result<Ptr<Operation>> {
    // Check if the instruction has already been converted
    if let Some(op) = cctx.ops.get(&inst) {
        return Ok(*op);
    };

    // Convert the instruction based on its opcode
    let clif_opcode = dfg.insts[inst].opcode();
    let inst_args = dfg.inst_args(inst);
    match clif_opcode {
        Opcode::Iadd => {
            let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
            let iadd_op = IAddOp::new(ctx, operands[0], operands[1]);
            let op = iadd_op.get_operation();
            return Ok(op);
        }
        Opcode::Return => match inst_args.len() {
            // No return values
            0 => {
                let return_op = ReturnOp::new(ctx, None);
                let op = return_op.get_operation();
                return Ok(op);
            }
            // a single return value
            1 => {
                let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
                let return_op = ReturnOp::new(ctx, Some(operands[0]));
                let op = return_op.get_operation();
                return Ok(op);
            }
            _ => unimplemented!("Multiple return values are not supported"),
        },
        _ => unimplemented!("Opcode {} is not implemented", clif_opcode),
    }
}

/// Converts a [ClifBasicBlock] to Pliron's [BasicBlock].
///
/// This function checks if the block has already been converted (using `cctx` for caching).
/// If not, it creates a new `BasicBlock` in Pliron's IR, deriving its label and argument types
/// from the provided `ClifBasicBlock`.
///
/// # Arguments
/// - `ctx`: The Pliron context for creating IR entities.
/// - `dfg`: The data flow graph containing the original block's information.
/// - `cctx`: The conversion context for caching converted entities.
/// - `block`: The `ClifBasicBlock` to convert.
///
/// # Returns
/// A `Result` containing the converted `BasicBlock` or an error if conversion fails.
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

/// Converts a [ClifType] to Pliron's [TypeObj].
///
/// This function maps Cranelift types to their corresponding Pliron types. Currently, only `i32`
/// is supported.
///
/// # Arguments
/// - `ctx`: The Pliron context for creating IR entities.
/// - `ty`: The `ClifType` to convert.
///
/// # Returns
/// A `Result` containing the converted `TypeObj` or an error if conversion fails.
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

/// Converts a Cranelift [Function] to a Pliron [FuncOp].
///
/// This function translates all Cranelift entities (instructions, blocks, operands, types, etc.)
/// into their Pliron equivalents and stores them in the conversion context (`cctx`).
///
/// # Arguments
/// - `ctx`: The Pliron context for creating IR entities.
/// - `cctx`: The conversion context for caching converted entities.
/// - `func`: The Cranelift `Function` to convert.
///
/// # Returns
/// A `Result` containing the converted `FuncOp` or an error if conversion fails.
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

fn convert_instruction_w_rpo(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    cctx: &mut ConversionCtx,
    inst: Inst,
) -> Result<Ptr<Operation>> {
    // Convert the instruction based on its opcode
    let clif_opcode = dfg.insts[inst].opcode();
    let inst_args = dfg.inst_args(inst);
    match clif_opcode {
        Opcode::Iadd => {
            let operands = convert_operands_w_rpo(dfg, cctx, &inst_args)?;
            let iadd_op = IAddOp::new(ctx, operands[0], operands[1]);
            let op = iadd_op.get_operation();
            return Ok(op);
        }
        Opcode::Return => match inst_args.len() {
            // No return values
            0 => {
                let return_op = ReturnOp::new(ctx, None);
                let op = return_op.get_operation();
                return Ok(op);
            }
            // a single return value
            1 => {
                let operands = convert_operands_w_rpo(dfg, cctx, &inst_args)?;
                let return_op = ReturnOp::new(ctx, Some(operands[0]));
                let op = return_op.get_operation();
                return Ok(op);
            }
            _ => unimplemented!("Multiple return values are not supported"),
        },
        _ => unimplemented!("Opcode {} is not implemented", clif_opcode),
    }
}

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
/// Tracks converted Pliron entities during IR transformation.
///
/// This struct ensures efficient lookup and prevents redundant conversions of operations,
/// regions, and basic blocks during the transformation process.
///
/// # Fields
/// - `ops`: Maps original instructions to their converted `Operation` entities.
/// - `regs`: Stores pointers to converted `Region` entities.
/// - `bbs`: Maps `ClifBasicBlock`s to their corresponding Pliron `BasicBlock`s.
/// - `entry_block`: The entry `BasicBlock` of the function being processed, if any.
/// - `func_op`: The root `Operation` representing the function being processed, if any.
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
    use super::*;
    use cranelift_reader::parse_functions;
    use pliron::{builtin, printable::Printable};

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
}
