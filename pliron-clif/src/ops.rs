//! [Op]s defined in the CLIF dialect.
//!
//! This module defines a set of operations (ops) that mirror the CLIF dialect using the Pliron framework.
//! It includes both binary and unary arithmetic operations along with a return operation.

use combine::Parser;
use pliron::common_traits::Named;
use pliron::derive::{def_op, derive_op_interface_impl, op_interface_impl};

use pliron::builtin::op_interfaces;
use pliron::identifier::Identifier;
use pliron::irfmt::parsers::{
    block_opd_parser, delimited_list_parser, process_parsed_ssa_defs, spaced, ssa_opd_parser,
};
use pliron::irfmt::printers::iter_with_sep;
use pliron::location::{Located, Location};
use pliron::op::OpObj;
use pliron::parsable::{IntoParseResult, Parsable, ParseResult, StateStream};
use pliron::printable::Printable;
use pliron::{
    basic_block::BasicBlock,
    builtin::op_interfaces::{
        BranchOpInterface, IsTerminatorInterface, OneResultInterface, SameOperandsAndResultType,
        SameOperandsType, SameResultsType, ZeroResultInterface,
    },
    context::{Context, Ptr},
    derive::format_op,
    impl_verify_succ,
    op::Op,
    operation::Operation,
    value::Value,
};
use pliron::{input_err, irfmt};

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

// Equivalent to CLIF's Unconditional Branch.
/// ### Operands
/// | operand     |           description                     |
/// |-----|-------|
/// | `dest_opds` | Any number of operands with any CLIF type |
///
/// ### Successors:
///
/// | Successor   |  description  |
/// |-----|-------|
/// |   `dest`    | Any successor |
#[def_op("clif.jump")]
#[format_op("succ($0) `(` operands(CharSpace(`,`)) `)`")]
#[derive_op_interface_impl(IsTerminatorInterface, ZeroResultInterface)]
pub struct JumpOp;
impl_verify_succ!(JumpOp);

#[op_interface_impl]
impl BranchOpInterface for JumpOp {
    fn successor_operands(&self, ctx: &Context, succ_idx: usize) -> Vec<Value> {
        assert!(succ_idx == 0, "JumpOp has exactly one successor");
        self.get_operation().deref(ctx).operands().collect()
    }
}

impl JumpOp {
    /// Create a new [JumpOp].
    pub fn new(ctx: &mut Context, dest: Ptr<BasicBlock>, dest_opds: Vec<Value>) -> Self {
        JumpOp {
            op: Operation::new(
                ctx,
                Self::get_opid_static(),
                vec![],
                dest_opds,
                vec![dest],
                0,
            ),
        }
    }
}

// Equivalent to CLIF's Conditional Branch.
/// ### Operands
/// | operand           |                description                |
/// |-----|-------------|
/// | `condition`       | 1-bit signless integer                    |
/// | `true_dest_opds`  | Any number of operands with any CLIF type |
/// | `false_dest_opds` | Any number of operands with any CLIF type |
///
/// ### Successors:
///
/// | Successor    | description   |
/// |-----|--------|
/// | `true_dest`  | Any successor |
/// | `false_dest` | Any successor |
#[def_op("clif.brif")]
#[derive_op_interface_impl(IsTerminatorInterface, ZeroResultInterface)]
pub struct BrifOp;
impl BrifOp {
    /// Create anew [BrifOp].
    pub fn new(
        ctx: &mut Context,
        condition: Value,
        true_dest: Ptr<BasicBlock>,
        mut true_dest_opds: Vec<Value>,
        false_dest: Ptr<BasicBlock>,
        mut false_dest_opds: Vec<Value>,
    ) -> Self {
        let mut operands = vec![condition];
        operands.append(&mut true_dest_opds);
        operands.append(&mut false_dest_opds);
        BrifOp {
            op: Operation::new(
                ctx,
                Self::get_opid_static(),
                vec![],
                operands,
                vec![true_dest, false_dest],
                0,
            ),
        }
    }

    /// Get the condition value for the branch.
    pub fn condition(&self, ctx: &Context) -> Value {
        self.op.deref(ctx).get_operand(0)
    }
}

impl Printable for BrifOp {
    fn fmt(
        &self,
        ctx: &Context,
        _state: &pliron::printable::State,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let op = self.get_operation().deref(ctx);
        let condition = op.get_operand(0);
        let true_dest_opds = self.successor_operands(ctx, 0);
        let false_dest_opds = self.successor_operands(ctx, 1);
        let res = write!(
            f,
            "{} if {} ^{}({}) else ^{}({})",
            op.get_opid(),
            condition.disp(ctx),
            op.get_successor(0).deref(ctx).unique_name(ctx),
            iter_with_sep(
                true_dest_opds.iter(),
                pliron::printable::ListSeparator::CharSpace(',')
            )
            .disp(ctx),
            op.get_successor(1).deref(ctx).unique_name(ctx),
            iter_with_sep(
                false_dest_opds.iter(),
                pliron::printable::ListSeparator::CharSpace(',')
            )
            .disp(ctx),
        );
        res
    }
}

impl Parsable for BrifOp {
    type Arg = Vec<(Identifier, Location)>;
    type Parsed = OpObj;
    fn parse<'a>(
        state_stream: &mut StateStream<'a>,
        results: Self::Arg,
    ) -> ParseResult<'a, Self::Parsed> {
        if !results.is_empty() {
            input_err!(
                state_stream.loc(),
                op_interfaces::ZeroResultVerifyErr(Self::get_opid_static().to_string())
            )?
        }

        // Parse the condition operand.
        let r#if = irfmt::parsers::spaced::<StateStream, _>(combine::parser::char::string("if"));
        let condition = ssa_opd_parser();
        let true_operands = delimited_list_parser('(', ')', ',', ssa_opd_parser());
        
        let r_else =
            irfmt::parsers::spaced::<StateStream, _>(combine::parser::char::string("else"));
        let false_operands = delimited_list_parser('(', ')', ',', ssa_opd_parser());

        let final_parser = r#if
            .with(spaced(condition))
            .and(spaced(block_opd_parser()))
            .and(true_operands)
            .and(spaced(r_else).with(spaced(block_opd_parser()).and(false_operands)));

        final_parser
            .then(
                move |(((condition, true_dest), true_dest_opds), (false_dest, false_dest_opds))| {
                    let results = results.clone();
                    let mut operands = vec![condition];
                    operands.extend(true_dest_opds);
                    operands.extend(false_dest_opds);
                    combine::parser(move |parsable_state: &mut StateStream<'a>| {
                        let ctx = &mut parsable_state.state.ctx;
                        let op = Operation::new(
                            ctx,
                            Self::get_opid_static(),
                            vec![],
                            operands.clone(),
                            vec![true_dest, false_dest],
                            0,
                        );

                        process_parsed_ssa_defs(parsable_state, &results, op)?;
                        let op: OpObj = Box::new(BrifOp { op });
                        Ok(op).into_parse_result()
                    })
                },
            )
            .parse_stream(state_stream)
            .into()
    }
}

impl_verify_succ!(BrifOp);

#[op_interface_impl]
impl BranchOpInterface for BrifOp {
    fn successor_operands(&self, ctx: &Context, succ_idx: usize) -> Vec<Value> {
        assert!(
            succ_idx == 0 || succ_idx == 1,
            "BrifOp has exactly two successors"
        );
        let num_opds_succ0 = self
            .get_operation()
            .deref(ctx)
            .get_successor(0)
            .deref(ctx)
            .get_num_arguments();
        if succ_idx == 0 {
            // Skip `condition` operand and take num_opds_succ0 operands after that.
            self.get_operation()
                .deref(ctx)
                .operands()
                .skip(1)
                .take(num_opds_succ0)
                .collect()
        } else {
            // Skip `condition` and `true_dest_opds`. Take the remaining.
            self.get_operation()
                .deref(ctx)
                .operands()
                .skip(1 + num_opds_succ0)
                .collect()
        }
    }
}

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
    JumpOp::register(ctx, JumpOp::parser_fn);
    BrifOp::register(ctx, BrifOp::parser_fn);
}
