//! [Op]s defined in the CLIF dialect.
//! 
//! This module defines a set of operations (ops) that mirror the CLIF dialect using the Pliron framework.
//! It includes both binary and unary arithmetic operations along with a return operation.

use pliron::derive::{def_op, derive_op_interface_impl};

use pliron::parsable::Parsable;
use pliron::{
    builtin::op_interfaces::{
        IsTerminatorInterface, OneResultInterface, SameOperandsAndResultType, SameOperandsType,
        SameResultsType,
    },
    context::Context,
    derive::format_op,
    impl_verify_succ,
    op::Op,
    operation::Operation,
    value::Value,
};

use crate::op_interfaces::{BinArithOp, IntBinArithOp, IntUnaryArithOp, UnaryArithOp};

/// -------------------------------------------------------------------------
/// ReturnOp
/// -------------------------------------------------------------------------
///
/// This op is equivalent to CLIF's `return` opcode. It takes an optional value
/// (of any type) as an operand and returns it. When no operand is provided,
/// it represents a void return.
///
/// **Operands:**
/// - `arg`: any type
#[def_op("clif.return")]
#[format_op("`(` operands(CharSpace(`,`)) `)`")]
#[derive_op_interface_impl(IsTerminatorInterface)]
pub struct ReturnOp;
impl ReturnOp {
    /// Creates a new ReturnOp.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The current context in which the op is created.
    /// * `value` - An optional value to return.
    ///
    /// # Returns
    ///
    /// A new instance of ReturnOp.
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

    /// Retrieves the return value from the op if one exists.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The current context.
    ///
    /// # Returns
    ///
    /// * `Some(Value)` if there is exactly one operand,
    /// * `None` otherwise.
    pub fn retval(&self, ctx: &Context) -> Option<Value> {
        let op = &*self.get_operation().deref(ctx);
        if op.get_num_operands() == 1 {
            Some(op.get_operand(0))
        } else {
            None
        }
    }
}
// impl_canonical_syntax!(ReturnOp);
impl_verify_succ!(ReturnOp);

/// -------------------------------------------------------------------------
/// Macros for Defining Integer Binary Operations
/// -------------------------------------------------------------------------

/// Macro to define a binary op for integer arithmetic without a custom formatting string.
/// It sets up the op with its operand and result interfaces.
/// 
/// Parameters:
/// - `$op_name`: Identifier for the op struct.
/// - `$op_id`: The string identifier for the op (e.g., "clif.iadd").
macro_rules! new_int_bin_op_without_format {
    (   $(#[$outer:meta])*
        $op_name:ident, $op_id:literal
    ) => {
        #[def_op($op_id)]
        $(#[$outer])*
        /// ### Operands:
        ///
        /// | operand | description      |
        /// |---------|------------------|
        /// | `lhs`   | Signless integer |
        /// | `rhs`   | Signless integer |
        ///
        /// ### Result(s):
        /// | result | description      |
        /// |--------|------------------|
        /// | `res`  | Signless integer |
        #[pliron::derive::derive_op_interface_impl(
            OneResultInterface, SameOperandsType, SameResultsType,
            SameOperandsAndResultType, BinArithOp, IntBinArithOp
        )]
        pub struct $op_name;

        impl_verify_succ!($op_name);
    }
}

/// Macro to define a binary op for integer arithmetic with a specific formatting string.
/// It builds on `new_int_bin_op_without_format!` while adding a format specification.
///
/// The provided format string prints the first operand, a literal comma, the second operand,
/// and appends a colon with the type of the first operand.
macro_rules! new_int_bin_op {
    (   $(#[$outer:meta])*
        $op_name:ident, $op_id:literal
    ) => {
        new_int_bin_op_without_format!(
            $(#[$outer])*
            #[format_op("$0 `,` $1 `:` type($0)")]
            $op_name,
            $op_id
        );
    }
}

/// -------------------------------------------------------------------------
/// Macros for Defining Integer Unary Operations
/// -------------------------------------------------------------------------

/// Macro to define a unary op for integer arithmetic without a custom formatting string.
/// It sets up the op with its operand and result interfaces.
///
/// Parameters:
/// - `$op_name`: Identifier for the op struct.
/// - `$op_id`: The string identifier for the op (e.g., "clif.ineg").
macro_rules! new_int_unary_op_without_format {
    (   $(#[$outer:meta])*
        $op_name:ident, $op_id:literal
    ) => {
        #[def_op($op_id)]
        $(#[$outer])*
        /// ### Operand:
        ///
        /// | operand | description      |
        /// |---------|------------------|
        /// | `x`     | Signless integer |
        ///
        /// ### Result:
        /// | result | description      |
        /// |--------|------------------|
        /// | `res`  | Signless integer |
        #[pliron::derive::derive_op_interface_impl(
            OneResultInterface, SameOperandsType, SameResultsType,
            UnaryArithOp, IntUnaryArithOp
        )]
        pub struct $op_name;

        impl_verify_succ!($op_name);
    }
}

/// Macro to define a unary op for integer arithmetic with a specific formatting string.
/// It builds on `new_int_unary_op_without_format!` while adding a format specification.
///
/// The format string prints the operand, followed by a colon and its type.
macro_rules! new_int_unary_op {
    (   $(#[$outer:meta])*
        $op_name:ident, $op_id:literal
    ) => {
        new_int_unary_op_without_format!(
            $(#[$outer])*
            #[format_op("$0':'type($0)")]
            $op_name,
            $op_id
        );
    }
}

// Define the integer addition op (`clif.iadd`).
new_int_bin_op!(
    /// Equivalent to CLIF's standard integer addition (with no overflow) opcode.
    IAddOp,
    "clif.iadd"
);

// Define the integer subtraction op (`clif.isub`).
new_int_bin_op!(
    /// Equivalent to CLIF's standard integer subtraction (with no overflow) opcode.
    ISubOp,
    "clif.isub"
);

// Define the unsigned minimum op (`clif.umin`).
new_int_bin_op!(
    /// Equivalent to CLIF's standard integer subtraction (with no overflow) opcode.
    UminOp,
    "clif.umin"
);

// Define the unsigned maximum op (`clif.umax`).
new_int_bin_op!(
    /// Equivalent to CLIF's standard integer subtraction (with no overflow) opcode.
    UmaxOp,
    "clif.umax"
);

// Define the integer negation op (`clif.ineg`).
new_int_unary_op!(
    /// Equivalent to CLIF's integer negation (`ineg`) opcode.
    InegOp,
    "clif.ineg"
);

// Define the integer absolute value op (`clif.iabs`).
new_int_unary_op!(
    /// Equivalent to CLIF's integer negation (`ineg`) opcode.
    IabsOp,
    "clif.iabs"
);

/// -------------------------------------------------------------------------
/// Registration Function
/// -------------------------------------------------------------------------
///
/// This function registers all the defined ops into the given context.
/// It ensures that each op is available for parsing and conversion.
pub fn register(ctx: &mut Context) {
    ReturnOp::register(ctx, ReturnOp::parser_fn);
    IAddOp::register(ctx, IAddOp::parser_fn);
    ISubOp::register(ctx, ISubOp::parser_fn);
    UminOp::register(ctx, UminOp::parser_fn);
    UmaxOp::register(ctx, UmaxOp::parser_fn);
    InegOp::register(ctx, InegOp::parser_fn);
    IabsOp::register(ctx, IabsOp::parser_fn);
}
