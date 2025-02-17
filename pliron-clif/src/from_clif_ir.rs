use pliron::{
    basic_block::BasicBlock,
    builtin::types::{IntegerType, Signedness},
    context::{Context, Ptr},
    identifier::Identifier,
    op::Op,
    operation::Operation,
    r#type::TypeObj,
    result::Result,
    value::Value as PlironValue,
};

use cranelift_codegen::ir::{
    types::Type as ClifType, Block as ClifBasicBlock, DataFlowGraph, Inst, Opcode,
    Value as ClifValue, ValueDef,
};

use crate::{
    op_interfaces::BinArithOp,
    ops::{IAddOp, ReturnOp},
};

fn convert_operands(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    operands: &[ClifValue],
) -> Result<Vec<PlironValue>> {
    let mut pliron_operands = vec![];
    for operand in operands {
        let value_def = dfg.value_def(*operand);
        match value_def {
            // Is this a value defined by an instruction?
            ValueDef::Result(inst, idx) => {
                let op = convert_clif_instruction(ctx, dfg, inst)?;
                let pliron_value = PlironValue::OpResult { op, res_idx: idx };
                pliron_operands.push(pliron_value);
            }
            // Is this value a BasicBlock parameter?
            ValueDef::Param(block, idx) => {
                let block = convert_block(ctx, dfg, block)?;
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

fn convert_clif_instruction(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    inst: Inst,
) -> Result<Ptr<Operation>> {
    let clif_opcode = dfg.insts[inst].opcode();
    let inst_args = dfg.inst_args(inst);
    match clif_opcode {
        Opcode::Iadd => {
            let operands = convert_operands(ctx, dfg, &inst_args)?;
            let iadd_op = IAddOp::new(ctx, operands[0], operands[1]);
            let op = iadd_op.get_operation();
            return Ok(op);
        }
        Opcode::Return => match inst_args.len() {
            0 => {
                let return_op = ReturnOp::new(ctx, None);
                let op = return_op.get_operation();
                return Ok(op);
            }
            1 => {
                let operands = convert_operands(ctx, dfg, &inst_args)?;
                let return_op = ReturnOp::new(ctx, Some(operands[0]));
                let op = return_op.get_operation();
                return Ok(op);
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

/// Convert [ClifBasicBlock] to pliron's [BasicBlock].
fn convert_block(
    ctx: &mut Context,
    dfg: &DataFlowGraph,
    block: ClifBasicBlock,
) -> Result<Ptr<BasicBlock>> {
    let label = Identifier::try_new(format!("{}", block))?;
    let block_params = dfg.block_params(block);
    let mut arg_types = vec![];
    for param in block_params {
        let param_type = dfg.value_type(*param);
        let pliron_type = convert_clif_type(ctx, param_type)?;
        arg_types.push(pliron_type);
    }
    Ok(BasicBlock::new(ctx, Some(label), arg_types))
}

fn convert_clif_type(ctx: &mut Context, ty: ClifType) -> Result<Ptr<TypeObj>> {
    match ty.to_string().as_str() {
        "i32" => {
            let pliron_int_type_ptr = IntegerType::get(ctx, 32, Signedness::Signed);
            return Ok(pliron_int_type_ptr.to_ptr());
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cranelift_reader::parse_functions;
    use pliron::builtin;

    #[test]
    fn test_parse_clif_add() {
        let clif_code = r#"
        function %add(i32, i32) -> i32 apple_aarch64 {
            block0(v0: i32, v1: i32):
                v2 = iadd v0, v1
                return v2
        }
    "#;

        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        for func in functions {
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            let entry_block = func.layout.entry_block().unwrap();
            let dfg = &func.dfg;
            let block = convert_block(&mut ctx, &dfg, entry_block).unwrap();
            for inst in func.layout.block_insts(entry_block) {
                let op = convert_clif_instruction(&mut ctx, &dfg, inst).unwrap();
            }
        }
    }
}
