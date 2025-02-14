//! [Op]s defined in the CLIF dialect

use combine::parser::Parser;
use pliron::derive::{def_op, derive_op_interface_impl};

use pliron::{
    builtin::op_interfaces::{
        IsTerminatorInterface, OneResultInterface, SameOperandsAndResultType, SameOperandsType,
        SameResultsType,
    },
    context::Context,
    derive::format_op,
    impl_canonical_syntax, impl_verify_succ,
    op::Op,
    operation::Operation,
    value::Value,
};

use crate::op_interfaces::{BinArithOp, IntBinArithOp};

/// Equivalent to CLIF's return opcode.
///
/// Operands:
///
/// | Operand | Description |
/// |---------|-------------|
/// | `arg`   | any type    |
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
        ///
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

new_int_bin_op!(
    /// Equivalent to CLIF's standard integer addition (with no overflow) opcode.
    IAddOp,
    "clif.iadd"
);

new_int_bin_op!(
    /// Equivalent to CLIF's standard integer subtraction (with no overflow) opcode.
    ISubOp,
    "clif.isub"
);
