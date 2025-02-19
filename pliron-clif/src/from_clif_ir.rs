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
            // No return values
            0 => {
                let return_op = ReturnOp::new(ctx, None);
                let op = return_op.get_operation();
                return Ok(op);
            }
            // a single return value
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

/// Convert a Cranelift [Function] to a Pliron [FuncOp], translating all Cranelift  
/// entities (instructions, blocks, operands, types, etc.) into their Pliron equivalents  
/// and storing them in the converted entity store.
fn convert_function(
    ctx: &mut Context,
    store: &mut ConversionStore,
    func: Function,
) -> Result<FuncOp> {
    // does the clif-pliron conversion and links relevant fn (components or) entities. 
    // Todo: this impl does cycle through `Ops` with nested regions yet.
    fn convert_and_link(ctx: &mut Context, store: &mut ConversionStore, func: Function) {
        let dfg = &func.dfg;
        for (idx, block) in func.layout.blocks().enumerate() {
            let bb = convert_block(ctx, &dfg, block).unwrap();
            match idx {
                0 => store.bbs.push(bb), // the entry block should already be linked, just push to store.
                _ => match store.bbs.get(idx) {
                    Some(prev_bb) => {
                        bb.insert_after(ctx, *prev_bb);
                        store.bbs.push(bb);
                    }
                    None => {}
                },
            }
            // A Clif layout (via Layout type) contains an RPO ordered list of instructions within a block, which we
            // can simply iterate over.
            for (idx, inst) in func.layout.block_insts(block).enumerate() {
                let op = convert_clif_instruction(ctx, &dfg, inst).unwrap();
                match idx {
                    0 => {
                        let container = store
                            .entry_block
                            .expect("Failed to retrieve function EntryBlock");
                        op.insert_at_front(container, ctx);
                        store.ops.push(op);
                        // println!("idx: {}, op: {}", idx, store.ops.get(idx).unwrap().deref(ctx).disp(ctx));
                    }
                    _ => match store.ops.get(idx) {
                        Some(prev_op) => {
                            op.insert_after(ctx, *prev_op);
                            store.ops.push(op);
                        }
                        None => {}
                    },
                }
            }
        }
    }

    let func_name = func.name.to_string().split_off(1);
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
    store.entry_block = Some(func_op.get_entry_block(ctx));
    store.regs.push(func_op.get_region(ctx));
    store.ops.push(func_op.get_operation());

    convert_and_link(ctx, store, func);

    Ok(func_op)
}

/// Stores converted Pliron entities during IR transformation.
///
/// This struct tracks various Pliron components—operations, regions, and basic blocks—ensuring
/// efficient lookup and preventing redundant conversions.
///
/// # Fields
/// - `ops`: Pointers to converted `Operation` entities.
/// - `regs`: Pointers to converted `Region` entities.
/// - `bbs`: Pointers to converted `BasicBlock` entities.
/// - `entry_block`: The entry `BasicBlock` of the function being processed, if available.
#[derive(Default)]
struct ConversionStore {
    /// A store for converted Pliron `Operation` entities.
    ops: Vec<Ptr<Operation>>,
    /// A store for converted Pliron `Region` entities.
    regs: Vec<Ptr<Region>>,
    /// A store for converted Pliron `BasicBlock` entities.
    bbs: Vec<Ptr<BasicBlock>>,
    /// The entry `BasicBlock` of the function being processed, if available.
    entry_block: Option<Ptr<BasicBlock>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use cranelift_reader::parse_functions;
    use pliron::{builtin, printable::Printable};

    #[test]
    fn test_convert_and_print_clif_add_to_pliron() {
        let clif_code = r#"
        function %add(i32, i32) -> i32 apple_aarch64 {
            block0(v0: i32, v1: i32):
                v2 = iadd v0, v1
                return v2
        }
    "#;

        let functions = parse_functions(clif_code).expect("Failed to parse .clif");

        for func in functions {
            let mut store = ConversionStore::default();
            let mut ctx = Context::new();
            builtin::register(&mut ctx);
            crate::register(&mut ctx);
            let func_op = match convert_function(&mut ctx, &mut store, func) {
                Ok(op) => op,
                Err(e) => panic!("Error: {}", e),
            };
            let print_func = func_op.disp(&ctx);
            println!("{}", print_func);
            assert_eq!(
                "builtin.func @add: builtin.function <(builtin.int <si32>, builtin.int <si32>)->(builtin.int <si32>)> 
{
  ^entry_block_1v1(block_1v1_arg0:builtin.int <si32>,block_1v1_arg1:builtin.int <si32>):
    op_2v1_res0 = clif.iadd block_3v1_arg0,block_4v1_arg1:builtin.int <si32>;
    clif.return (op_3v1_res0) [] []: <(builtin.int <si32>) -> ()>
}",
                format!("{}", print_func)
            );
        }
    }
}
