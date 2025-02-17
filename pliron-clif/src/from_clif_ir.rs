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

use cranelift_codegen::ir::{
    types::Type as ClifType, Block as ClifBasicBlock, DataFlowGraph, Function, Inst, Opcode,
    Value as ClifValue, ValueDef,
};

use crate::{
    op_interfaces::BinArithOp,
    ops::{IAddOp, ReturnOp},
};

/// Convert a [ClifValue] to pliron's [PlironValue].
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

/// Convert a cranelift [Inst] to pliron's [Ptr<Operation>].
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

/// Convert [ClifType] to pliron's [TypeObj].
fn convert_clif_type(ctx: &mut Context, ty: ClifType) -> Result<Ptr<TypeObj>> {
    match ty.to_string().as_str() {
        "i32" => {
            let pliron_int_type_ptr = IntegerType::get(ctx, 32, Signedness::Signed);
            return Ok(pliron_int_type_ptr.to_ptr());
        }
        _ => unimplemented!(),
    }
}

fn populate_converted_pliron_entitystore(
    ctx: &mut Context,
    store: ConvertedPlironEntityStore,
    func: Function,
) {
    let mut store = ConvertedPlironEntityStore::default();
    let entry_block = func.layout.entry_block().unwrap();
    let dfg = &func.dfg;
    let block = convert_block(ctx, &dfg, entry_block).unwrap();
    store.entry_block = Some(block);
    for block in func.layout.blocks() {
        let ptr_bb = convert_block(ctx, &dfg, block).unwrap();
        store.bbs.push(ptr_bb);
        for inst in func.layout.block_insts(block) {
            let op = convert_clif_instruction(ctx, &dfg, inst).unwrap();
            if op.deref(ctx).num_regions() > 0 {
                todo!()
            } else {
                store.ops.push(op);
            }
        }
    }
}

/// Convert a Cranelift [Function] to a Pliron [FuncOp], translating all Cranelift  
/// entities (instructions, blocks, operands, types, etc.) into their Pliron equivalents  
/// and storing them in the converted entity store.
fn convert_function(ctx: &mut Context, func: Function) -> Result<FuncOp> {
    let func_name = func.name.to_string();
    let func_type = func.signature.clone();
    let func_params_types: Vec<_> = func_type
        .params
        .iter()
        .map(|param| {
            convert_clif_type(ctx, param.value_type)
                .expect("Failed to convert Clif Function Parameter Type")
        })
        .collect();
    let func_return_types: Vec<_> = func_type
        .returns
        .iter()
        .map(|ret| {
            convert_clif_type(ctx, ret.value_type)
                .expect("Failed to convert Clif Function Return Type")
        })
        .collect();
    let pliron_func_type = FunctionType::get(ctx, func_params_types, func_return_types);
    let func_op = FuncOp::new(ctx, &Identifier::try_new(func_name)?, pliron_func_type);

    let mut store = ConvertedPlironEntityStore::default();
    store.ops.push(func_op.get_operation());
    store.regs.push(func_op.get_region(ctx));

    populate_converted_pliron_entitystore(ctx, store, func);

    Ok(func_op)
}
/// Storage for converted Pliron entities.
#[derive(Default)]
struct ConvertedPlironEntityStore {
    /// A store for converted pliron's Operations
    ops: Vec<Ptr<Operation>>,
    /// A store for converted pliron's Regions
    regs: Vec<Ptr<Region>>,
    /// A store for converted pliron's BasicBlocks.
    bbs: Vec<Ptr<BasicBlock>>,
    /// Entry block of the function we're processing.
    entry_block: Option<Ptr<BasicBlock>>,
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
            let store = ConvertedPlironEntityStore::default();
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            populate_converted_pliron_entitystore(&mut ctx, store, func);
        }
    }
}
