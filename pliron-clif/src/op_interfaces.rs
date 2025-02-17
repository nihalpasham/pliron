use pliron::derive::op_interface;
use thiserror::Error;

use pliron::{
    builtin::{
        op_interfaces::{OneResultInterface, SameOperandsAndResultType},
        types::{IntegerType, Signedness},
    },
    context::Context,
    location::Located,
    op::op_cast,
    op::Op,
    operation::Operation,
    r#type::Typed,
    result::Result,
    value::Value,
    verify_err,
};

#[derive(Error, Debug)]
#[error("Binary Arithmetic Op must have exactly two operands and one result")]
pub struct BinArithOpErr;

/// Binary arithmetic [Op].
#[op_interface]
pub trait BinArithOp: SameOperandsAndResultType + OneResultInterface {
    /// Create a new binary arithmetic operation given the operands.
    fn new(ctx: &mut Context, lhs: Value, rhs: Value) -> Self
    where
        Self: Sized,
    {
        let op = Operation::new(
            ctx,
            Self::get_opid_static(),
            vec![lhs.get_type(ctx)],
            vec![lhs, rhs],
            vec![],
            0,
        );
        *Operation::get_op(op, ctx).downcast::<Self>().ok().unwrap()
    }

    fn verify(op: &dyn Op, ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        let op = op.get_operation().deref(ctx);
        if op.get_num_operands() != 2 {
            return verify_err!(op.loc(), BinArithOpErr);
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
#[error("Integer binary arithmetic Op can only have signless integer result/operand type")]
pub struct IntBinArithOpErr;

/// Integer binary arithmetic [Op]
#[op_interface]
pub trait IntBinArithOp: BinArithOp {
    fn verify(op: &dyn Op, ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        let ty = op_cast::<dyn SameOperandsAndResultType>(op)
            .expect("Op must impl SameOperandsAndResultType")
            .get_type(ctx)
            .deref(ctx);
        let Some(int_ty) = ty.downcast_ref::<IntegerType>() else {
            return verify_err!(op.get_operation().deref(ctx).loc(), IntBinArithOpErr);
        };

        if int_ty.get_signedness() != Signedness::Signless {
            return verify_err!(op.get_operation().deref(ctx).loc(), IntBinArithOpErr);
        }

        Ok(())
    }
}
