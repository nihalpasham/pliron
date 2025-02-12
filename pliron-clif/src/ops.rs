//! [Op]s defined in the CLIF dialect

use pliron::{
    builtin::{op_interfaces::IsTerminatorInterface}, 
    context::{Context}, 
    impl_canonical_syntax, impl_verify_succ, 
    op::Op, 
    operation::Operation, 
    value::Value,
};
use pliron::derive::{def_op, derive_op_interface_impl};

/// Equivalent to CLIF's return opcode.
///
/// Operands:
///
/// | operand | description |
/// |-----|-------|
/// | `arg` | any type |
#[def_op("clif.return")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct ReturnOp;
impl ReturnOp {
    /// Create a new [ReturnOp]
    pub fn new(ctx: &mut Context, value: Option<Value>) -> Self {
        let op = Operation::new(
            ctx,
            Self::get_opid_static(),
            vec![],
            value.into_iter().collect(),
            vec![],
            0,
        );
        ReturnOp { op }
    }

    /// Get the returned value, if it exists.
    pub fn retval(&self, ctx: &Context) -> Option<Value> {
        let op = &*self.get_operation().deref(ctx);
        if op.get_num_operands() == 1 {
            Some(op.get_operand(0))
        } else {
            None
        }
    }
}
impl_canonical_syntax!(ReturnOp);
impl_verify_succ!(ReturnOp);
