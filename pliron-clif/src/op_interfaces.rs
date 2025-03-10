// Import required traits and macros from the Pliron framework.
// - `SameOperandsType` and `SameResultsType` ensure that the operation’s operands/results are of the same type.
// - `op_interface` is a macro to derive op interface implementations.
use pliron::{
    builtin::op_interfaces::{SameOperandsType, SameResultsType},
    derive::op_interface,
};
// Import the `Error` derive macro from the thiserror crate.
use thiserror::Error;

use pliron::{
    // Import additional op interfaces and types needed for binary/unary arithmetic operations.
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

/// -------------------------------------------------------------------------
/// Error Types for Arithmetic Operations
/// -------------------------------------------------------------------------
#[derive(Error, Debug)]
#[error("Binary Arithmetic Op must have exactly two operands and one result")]
pub struct BinArithOpErr;

/// Error for unary arithmetic operations if they do not have exactly one operand and one result.
#[derive(Error, Debug)]
#[error("Unary Arithmetic Op must have exactly one operand and one result")]
pub struct UnaryArithOpErr;

/// Error for integer unary arithmetic operations if the result/operand type is not a signless integer.
#[derive(Error, Debug)]
#[error("Integer unary arithmetic Op can only have signless integer result/operand type")]
pub struct IntUnaryArithOpErr;

/// Error for integer binary arithmetic operations if the result/operand type is not a signless integer.
#[derive(Error, Debug)]
#[error("Integer binary arithmetic Op can only have signless integer result/operand type")]
pub struct IntBinArithOpErr;

/// -------------------------------------------------------------------------
/// Binary Arithmetic Operation Trait
/// -------------------------------------------------------------------------

/// Trait for binary arithmetic operations.
///
/// This trait extends `SameOperandsAndResultType` and `OneResultInterface` to ensure that
/// binary operations have two operands with the same type and one result with that type.
/// It also provides a helper method `new` to create a new binary op and a `verify` method to
/// ensure the op has exactly two operands.
#[op_interface]
pub trait BinArithOp: SameOperandsAndResultType + OneResultInterface {
    /// Create a new binary arithmetic operation given two operands.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context in which the op is created.
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    ///
    /// # Returns
    ///
    /// A new instance of the binary arithmetic operation.
    fn new(ctx: &mut Context, lhs: Value, rhs: Value) -> Self
    where
        Self: Sized,
    {
        // Create a new operation with the type of the left-hand side
        let op = Operation::new(
            ctx,
            Self::get_opid_static(),
            vec![lhs.get_type(ctx)],
            vec![lhs, rhs],
            vec![],
            0,
        );
        // Downcast the generic operation to the specific operation type.
        *Operation::get_op(op, ctx).downcast::<Self>().ok().unwrap()
    }

    /// Verify that the operation has exactly two operands.
    ///
    /// # Arguments
    ///
    /// * `op` - The operation to verify.
    /// * `ctx` - The context for type and operand resolution.
    ///
    /// # Returns
    ///
    /// `Ok(())` if verification succeeds, or an error wrapped in a `Result` otherwise.
    fn verify(op: &dyn Op, ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        // Get the underlying operation using the context.
        let op = op.get_operation().deref(ctx);
        // Check if the number of operands is exactly 2.
        if op.get_num_operands() != 2 {
            return verify_err!(op.loc(), BinArithOpErr);
        }

        Ok(())
    }
}

/// -------------------------------------------------------------------------
/// Unary Arithmetic Operation Trait
/// -------------------------------------------------------------------------

/// Trait for unary arithmetic operations.
///
/// This trait extends `SameOperandsType`, `SameResultsType`, and `OneResultInterface` to ensure that
/// unary operations have one operand and one result of the same type. It provides a helper method `new`
/// for creation and a `verify` method to enforce the operand count.
#[op_interface]
pub trait UnaryArithOp: SameOperandsType + SameResultsType + OneResultInterface {
    /// Create a new unary arithmetic operation given a single operand.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context in which the op is created.
    /// * `x` - The operand.
    ///
    /// # Returns
    ///
    /// A new instance of the unary arithmetic operation.
    fn new(ctx: &mut Context, x: Value) -> Self
    where
        Self: Sized,
    {
        // Create a new operation with the type of the operand.
        let op = Operation::new(
            ctx,
            Self::get_opid_static(),
            vec![x.get_type(ctx)],
            vec![x],
            vec![],
            0,
        );
        // Downcast the generic operation to the specific operation type.
        *Operation::get_op(op, ctx).downcast::<Self>().ok().unwrap()
    }

    /// Verify that the operation has exactly one operand.
    ///
    /// # Arguments
    ///
    /// * `op` - The operation to verify.
    /// * `ctx` - The context for type and operand resolution.
    ///
    /// # Returns
    ///
    /// `Ok(())` if verification succeeds, or an error wrapped in a `Result` otherwise.
    fn verify(op: &dyn Op, ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        let op = op.get_operation().deref(ctx);
        // Check if the number of operands is exactly 1.
        if op.get_num_operands() != 1 {
            return verify_err!(op.loc(), UnaryArithOpErr);
        }
        Ok(())
    }
}

/// -------------------------------------------------------------------------
/// Integer Unary Arithmetic Operation Trait
/// -------------------------------------------------------------------------

/// Trait for integer unary arithmetic operations.
///
/// This trait extends `UnaryArithOp` and adds additional verification to ensure that
/// the operand and result types are signless integers.
#[op_interface]
pub trait IntUnaryArithOp: UnaryArithOp {
    /// Verify that the operand and result types are signless integers.
    ///
    /// # Arguments
    ///
    /// * `op` - The operation to verify.
    /// * `ctx` - The context for type and operand resolution.
    ///
    /// # Returns
    ///
    /// `Ok(())` if verification succeeds, or an error wrapped in a `Result` otherwise.
    fn verify(op: &dyn Op, ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        // Get the type of the op (from the trait ensuring operands and results have the same type).
        let ty = op_cast::<dyn SameOperandsAndResultType>(op)
            .expect("Op must implement SameOperandsAndResultType")
            .get_type(ctx)
            .deref(ctx);
        // Attempt to downcast the type to an IntegerType.
        let Some(int_ty) = ty.downcast_ref::<IntegerType>() else {
            return verify_err!(op.get_operation().deref(ctx).loc(), IntUnaryArithOpErr);
        };

        // Check that the integer type is signless.
        if int_ty.get_signedness() != Signedness::Signless {
            return verify_err!(op.get_operation().deref(ctx).loc(), IntUnaryArithOpErr);
        }

        Ok(())
    }
}

/// -------------------------------------------------------------------------
/// Integer Binary Arithmetic Operation Trait
/// -------------------------------------------------------------------------

/// Trait for integer binary arithmetic operations.
///
/// This trait extends `BinArithOp` and adds additional verification to ensure that
/// the operand and result types are signless integers.
#[op_interface]
pub trait IntBinArithOp: BinArithOp {
    /// Verify that the operand and result types are signless integers.
    ///
    /// # Arguments
    ///
    /// * `op` - The operation to verify.
    /// * `ctx` - The context for type and operand resolution.
    ///
    /// # Returns
    ///
    /// `Ok(())` if verification succeeds, or an error wrapped in a `Result` otherwise.
    fn verify(op: &dyn Op, ctx: &Context) -> Result<()>
    where
        Self: Sized,
    {
        // Get the type of the op from the trait ensuring operands and results are the same.
        let ty = op_cast::<dyn SameOperandsAndResultType>(op)
            .expect("Op must impl SameOperandsAndResultType")
            .get_type(ctx)
            .deref(ctx);
        // Attempt to downcast the type to an IntegerType.
        let Some(int_ty) = ty.downcast_ref::<IntegerType>() else {
            return verify_err!(op.get_operation().deref(ctx).loc(), IntBinArithOpErr);
        };

        // Check that the integer type is signless.
        if int_ty.get_signedness() != Signedness::Signless {
            return verify_err!(op.get_operation().deref(ctx).loc(), IntBinArithOpErr);
        }

        Ok(())
    }
}
