#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod attributes {}
pub mod op_interfaces {
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
    #[error("Binary Arithmetic Op must have exactly two operands and one result")]
    pub struct BinArithOpErr;
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl std::error::Error for BinArithOpErr {}
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for BinArithOpErr {
        #[allow(clippy::used_underscore_binding)]
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            #[allow(unused_variables, deprecated)]
            let Self {} = self;
            __formatter
                .write_str("Binary Arithmetic Op must have exactly two operands and one result")
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BinArithOpErr {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "BinArithOpErr")
        }
    }
    /// Binary arithmetic [Op].
    pub trait BinArithOp:
        ::pliron::op::Op + SameOperandsAndResultType + OneResultInterface
    {
        /// Create a new binary arithmetic operation given the operands.
        fn new(ctx: &mut Context, lhs: Value, rhs: Value) -> Self
        where
            Self: Sized,
        {
            let op = Operation::new(
                ctx,
                Self::get_opid_static(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([lhs.get_type(ctx)]),
                ),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([lhs, rhs]),
                ),
                ::alloc::vec::Vec::new(),
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
                return Err(::pliron::result::Error {
                    kind: ::pliron::result::ErrorKind::VerificationFailed,
                    err: Box::new(BinArithOpErr),
                    loc: op.loc(),
                    backtrace: std::backtrace::Backtrace::capture(),
                });
            }
            Ok(())
        }
    }
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmey0bMCDG8,regular,no_dead_strip")]
        static INTERFACE_DEP: std::sync::LazyLock<(std::any::TypeId, Vec<std::any::TypeId>)> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(std::any::TypeId,
                                            Vec<std::any::TypeId>)> { || &INTERFACE_DEP };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_DEPS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    std::any::TypeId::of::<dyn BinArithOp>(),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            std::any::TypeId::of::<dyn SameOperandsAndResultType>(),
                            std::any::TypeId::of::<dyn OneResultInterface>(),
                        ]),
                    ),
                )
            })
        };
    };
    #[error("Integer binary arithmetic Op can only have signless integer result/operand type")]
    pub struct IntBinArithOpErr;
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl std::error::Error for IntBinArithOpErr {}
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for IntBinArithOpErr {
        #[allow(clippy::used_underscore_binding)]
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            #[allow(unused_variables, deprecated)]
            let Self {} = self;
            __formatter.write_str(
                "Integer binary arithmetic Op can only have signless integer result/operand type",
            )
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IntBinArithOpErr {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "IntBinArithOpErr")
        }
    }
    /// Integer binary arithmetic [Op]
    pub trait IntBinArithOp: ::pliron::op::Op + BinArithOp {
        fn verify(op: &dyn Op, ctx: &Context) -> Result<()>
        where
            Self: Sized,
        {
            let ty = op_cast::<dyn SameOperandsAndResultType>(op)
                .expect("Op must impl SameOperandsAndResultType")
                .get_type(ctx)
                .deref(ctx);
            let Some(int_ty) = ty.downcast_ref::<IntegerType>() else {
                return Err(::pliron::result::Error {
                    kind: ::pliron::result::ErrorKind::VerificationFailed,
                    err: Box::new(IntBinArithOpErr),
                    loc: op.get_operation().deref(ctx).loc(),
                    backtrace: std::backtrace::Backtrace::capture(),
                });
            };
            if int_ty.get_signedness() != Signedness::Signless {
                return Err(::pliron::result::Error {
                    kind: ::pliron::result::ErrorKind::VerificationFailed,
                    err: Box::new(IntBinArithOpErr),
                    loc: op.get_operation().deref(ctx).loc(),
                    backtrace: std::backtrace::Backtrace::capture(),
                });
            }
            Ok(())
        }
    }
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmey0bMCDG8,regular,no_dead_strip")]
        static INTERFACE_DEP: std::sync::LazyLock<(std::any::TypeId, Vec<std::any::TypeId>)> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(std::any::TypeId,
                                            Vec<std::any::TypeId>)> { || &INTERFACE_DEP };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_DEPS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    std::any::TypeId::of::<dyn IntBinArithOp>(),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([std::any::TypeId::of::<dyn BinArithOp>()]),
                    ),
                )
            })
        };
    };
}
pub mod ops {
    //! [Op]s defined in the CLIF dialect
    //!
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
    pub struct ReturnOp {
        op: ::pliron::context::Ptr<::pliron::operation::Operation>,
    }
    impl IsTerminatorInterface for ReturnOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<ReturnOp>(),
                        std::any::TypeId::of::<dyn IsTerminatorInterface>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn IsTerminatorInterface + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn IsTerminatorInterface + 'static)> {
            r.downcast_ref::<ReturnOp>()
                .map(|s| s as &dyn IsTerminatorInterface)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    ReturnOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn IsTerminatorInterface>(),
                        <ReturnOp as IsTerminatorInterface>::verify,
                    ),
                )
            })
        };
    };
    #[automatically_derived]
    impl ::core::clone::Clone for ReturnOp {
        #[inline]
        fn clone(&self) -> ReturnOp {
            let _: ::core::clone::AssertParamIsClone<
                ::pliron::context::Ptr<::pliron::operation::Operation>,
            >;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ReturnOp {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ReturnOp {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ReturnOp {
        #[inline]
        fn eq(&self, other: &ReturnOp) -> bool {
            self.op == other.op
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ReturnOp {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                ::pliron::context::Ptr<::pliron::operation::Operation>,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ReturnOp {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.op, state)
        }
    }
    impl ::pliron::op::Op for ReturnOp {
        fn get_operation(&self) -> ::pliron::context::Ptr<::pliron::operation::Operation> {
            self.op
        }
        fn wrap_operation(
            op: ::pliron::context::Ptr<::pliron::operation::Operation>,
        ) -> ::pliron::op::OpObj {
            Box::new(ReturnOp { op })
        }
        fn get_opid(&self) -> ::pliron::op::OpId {
            Self::get_opid_static()
        }
        fn get_opid_static() -> ::pliron::op::OpId {
            ::pliron::op::OpId {
                name: ::pliron::op::OpName::new("return"),
                dialect: ::pliron::dialect::DialectName::new("clif"),
            }
        }
        fn verify_interfaces(
            &self,
            ctx: &::pliron::context::Context,
        ) -> ::pliron::result::Result<()> {
            if let Some(interface_verifiers) =
                ::pliron::op::OP_INTERFACE_VERIFIERS_MAP.get(&Self::get_opid_static())
            {
                for (_, verifier) in interface_verifiers {
                    verifier(self, ctx)?;
                }
            }
            Ok(())
        }
    }
    impl ReturnOp {
        /// Create a new [ReturnOp]
        pub fn new(ctx: &mut Context, value: Option<Value>) -> Self {
            let op = Operation::new(
                ctx,
                Self::get_opid_static(),
                ::alloc::vec::Vec::new(),
                value.into_iter().collect(),
                ::alloc::vec::Vec::new(),
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
    impl ::pliron::printable::Printable for ReturnOp {
        fn fmt(
            &self,
            ctx: &::pliron::context::Context,
            state: &::pliron::printable::State,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            ::pliron::op::canonical_syntax_print(Box::new(*self), ctx, state, f)
        }
    }
    impl ::pliron::parsable::Parsable for ReturnOp {
        type Arg = Vec<(
            ::pliron::identifier::Identifier,
            ::pliron::location::Location,
        )>;
        type Parsed = ::pliron::op::OpObj;
        fn parse<'a>(
            state_stream: &mut ::pliron::parsable::StateStream<'a>,
            results: Self::Arg,
        ) -> ::pliron::parsable::ParseResult<'a, Self::Parsed> {
            ::pliron::op::canonical_syntax_parser(
                <Self as ::pliron::op::Op>::get_opid_static(),
                results,
            )
            .parse_stream(state_stream)
            .into()
        }
    }
    impl ::pliron::common_traits::Verify for ReturnOp {
        fn verify(&self, _ctx: &::pliron::context::Context) -> ::pliron::result::Result<()> {
            Ok(())
        }
    }
    macro_rules! new_int_bin_op_without_format {
        ($(#[$outer:meta])* $op_name:ident, $op_id:literal) =>
        {
            #[def_op($op_id)] $(#[$outer])* /// ### Operands:
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
            #[pliron::derive::derive_op_interface_impl(OneResultInterface,
            SameOperandsType, SameResultsType, SameOperandsAndResultType,
            BinArithOp, IntBinArithOp)] pub struct $op_name;
            impl_verify_succ!($op_name);
        }
    }
    macro_rules! new_int_bin_op {
        ($(#[$outer:meta])* $op_name:ident, $op_id:literal) =>
        {
            new_int_bin_op_without_format!($(#[$outer])*
            #[format_op("$0 `,` $1 `:` type($0)")] $op_name, $op_id);
        }
    }
    impl ::pliron::printable::Printable for IAddOp {
        fn fmt(
            &self,
            ctx: &::pliron::context::Context,
            state: &::pliron::printable::State,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            use ::pliron::op::Op;
            use ::pliron::irfmt::printers::iter_with_sep;
            use ::pliron::common_traits::Named;
            let op = self.get_operation().deref(ctx);
            if op.get_num_results() > 0 {
                let sep = ::pliron::printable::ListSeparator::CharSpace(',');
                let results = iter_with_sep(op.results(), sep);
                fmt.write_fmt(format_args!("{0} = ", results.disp(ctx)))?;
            }
            fmt.write_fmt(format_args!("{0} ", self.get_opid()))?;
            let opd = self.get_operation().deref(ctx).get_operand(0usize);
            ::pliron::printable::Printable::fmt(&opd, ctx, state, fmt)?;
            ::pliron::printable::Printable::fmt(&",", ctx, state, fmt)?;
            let opd = self.get_operation().deref(ctx).get_operand(1usize);
            ::pliron::printable::Printable::fmt(&opd, ctx, state, fmt)?;
            ::pliron::printable::Printable::fmt(&":", ctx, state, fmt)?;
            let res = self.get_operation().deref(ctx).get_type(0usize);
            ::pliron::printable::Printable::fmt(&res, ctx, state, fmt)?;
            Ok(())
        }
    }
    impl ::pliron::parsable::Parsable for IAddOp {
        type Arg = Vec<(
            ::pliron::identifier::Identifier,
            ::pliron::location::Location,
        )>;
        type Parsed = ::pliron::op::OpObj;
        fn parse<'a>(
            state_stream: &mut ::pliron::parsable::StateStream<'a>,
            arg: Self::Arg,
        ) -> ::pliron::parsable::ParseResult<'a, Self::Parsed> {
            use ::pliron::parsable::IntoParseResult;
            use ::combine::Parser;
            use ::pliron::input_err;
            use ::pliron::location::Located;
            let cur_loc = state_stream.loc();
            use ::pliron::op::Op;
            use ::pliron::operation::Operation;
            use ::pliron::irfmt::parsers::{
                process_parsed_ssa_defs, ssa_opd_parser, block_opd_parser, attr_parser,
            };
            let opd_0 = ssa_opd_parser().parse_stream(state_stream).into_result()?.0;
            ::pliron::irfmt::parsers::spaced(::combine::parser::char::string(","))
                .parse_stream(state_stream)
                .into_result()?;
            let opd_1 = ssa_opd_parser().parse_stream(state_stream).into_result()?.0;
            ::pliron::irfmt::parsers::spaced(::combine::parser::char::string(":"))
                .parse_stream(state_stream)
                .into_result()?;
            let res_0 = ::pliron::irfmt::parsers::type_parser()
                .parse_stream(state_stream)
                .into_result()?
                .0;
            if arg.len() != 1usize {
                return Err(::pliron::result::Error {
                    kind: ::pliron::result::ErrorKind::InvalidInput,
                    err: Box::new(::pliron::result::StringError(::alloc::__export::must_use(
                        {
                            let res = ::alloc::fmt::format(format_args!(
                                "expected {0} results as per spec, got {1} during parsing",
                                1usize,
                                arg.len()
                            ));
                            res
                        },
                    ))),
                    loc: cur_loc,
                    backtrace: std::backtrace::Backtrace::capture(),
                })?;
            }
            let op = ::pliron::operation::Operation::new(
                state_stream.state.ctx,
                Self::get_opid_static(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([res_0]),
                ),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([opd_0, opd_1]),
                ),
                ::alloc::vec::Vec::new(),
                0,
            );
            for region in ::alloc::vec::Vec::new() {
                ::pliron::region::Region::move_to_op(region, op, state_stream.state.ctx);
            }
            process_parsed_ssa_defs(state_stream, &arg, op)?;
            let final_ret_value = Operation::get_op(op, state_stream.state.ctx);
            Ok(final_ret_value).into_parse_result()
        }
    }
    /// Equivalent to CLIF's standard integer addition (with no overflow) opcode.
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
    pub struct IAddOp {
        op: ::pliron::context::Ptr<::pliron::operation::Operation>,
    }
    impl OneResultInterface for IAddOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<IAddOp>(),
                        std::any::TypeId::of::<dyn OneResultInterface>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn OneResultInterface + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn OneResultInterface + 'static)> {
            r.downcast_ref::<IAddOp>()
                .map(|s| s as &dyn OneResultInterface)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    IAddOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn OneResultInterface>(),
                        <IAddOp as OneResultInterface>::verify,
                    ),
                )
            })
        };
    };
    impl SameOperandsType for IAddOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<IAddOp>(),
                        std::any::TypeId::of::<dyn SameOperandsType>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn SameOperandsType + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn SameOperandsType + 'static)> {
            r.downcast_ref::<IAddOp>()
                .map(|s| s as &dyn SameOperandsType)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    IAddOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn SameOperandsType>(),
                        <IAddOp as SameOperandsType>::verify,
                    ),
                )
            })
        };
    };
    impl SameResultsType for IAddOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<IAddOp>(),
                        std::any::TypeId::of::<dyn SameResultsType>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn SameResultsType + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn SameResultsType + 'static)> {
            r.downcast_ref::<IAddOp>()
                .map(|s| s as &dyn SameResultsType)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    IAddOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn SameResultsType>(),
                        <IAddOp as SameResultsType>::verify,
                    ),
                )
            })
        };
    };
    impl SameOperandsAndResultType for IAddOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<IAddOp>(),
                        std::any::TypeId::of::<dyn SameOperandsAndResultType>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn SameOperandsAndResultType + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn SameOperandsAndResultType + 'static)> {
            r.downcast_ref::<IAddOp>()
                .map(|s| s as &dyn SameOperandsAndResultType)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    IAddOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn SameOperandsAndResultType>(),
                        <IAddOp as SameOperandsAndResultType>::verify,
                    ),
                )
            })
        };
    };
    impl BinArithOp for IAddOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<IAddOp>(),
                        std::any::TypeId::of::<dyn BinArithOp>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn BinArithOp + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn BinArithOp + 'static)> {
            r.downcast_ref::<IAddOp>().map(|s| s as &dyn BinArithOp)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    IAddOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn BinArithOp>(),
                        <IAddOp as BinArithOp>::verify,
                    ),
                )
            })
        };
    };
    impl IntBinArithOp for IAddOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<IAddOp>(),
                        std::any::TypeId::of::<dyn IntBinArithOp>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn IntBinArithOp + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn IntBinArithOp + 'static)> {
            r.downcast_ref::<IAddOp>().map(|s| s as &dyn IntBinArithOp)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    IAddOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn IntBinArithOp>(),
                        <IAddOp as IntBinArithOp>::verify,
                    ),
                )
            })
        };
    };
    #[automatically_derived]
    impl ::core::clone::Clone for IAddOp {
        #[inline]
        fn clone(&self) -> IAddOp {
            let _: ::core::clone::AssertParamIsClone<
                ::pliron::context::Ptr<::pliron::operation::Operation>,
            >;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for IAddOp {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for IAddOp {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for IAddOp {
        #[inline]
        fn eq(&self, other: &IAddOp) -> bool {
            self.op == other.op
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for IAddOp {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                ::pliron::context::Ptr<::pliron::operation::Operation>,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for IAddOp {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.op, state)
        }
    }
    impl ::pliron::op::Op for IAddOp {
        fn get_operation(&self) -> ::pliron::context::Ptr<::pliron::operation::Operation> {
            self.op
        }
        fn wrap_operation(
            op: ::pliron::context::Ptr<::pliron::operation::Operation>,
        ) -> ::pliron::op::OpObj {
            Box::new(IAddOp { op })
        }
        fn get_opid(&self) -> ::pliron::op::OpId {
            Self::get_opid_static()
        }
        fn get_opid_static() -> ::pliron::op::OpId {
            ::pliron::op::OpId {
                name: ::pliron::op::OpName::new("iadd"),
                dialect: ::pliron::dialect::DialectName::new("clif"),
            }
        }
        fn verify_interfaces(
            &self,
            ctx: &::pliron::context::Context,
        ) -> ::pliron::result::Result<()> {
            if let Some(interface_verifiers) =
                ::pliron::op::OP_INTERFACE_VERIFIERS_MAP.get(&Self::get_opid_static())
            {
                for (_, verifier) in interface_verifiers {
                    verifier(self, ctx)?;
                }
            }
            Ok(())
        }
    }
    impl ::pliron::common_traits::Verify for IAddOp {
        fn verify(&self, _ctx: &::pliron::context::Context) -> ::pliron::result::Result<()> {
            Ok(())
        }
    }
    impl ::pliron::printable::Printable for ISubOp {
        fn fmt(
            &self,
            ctx: &::pliron::context::Context,
            state: &::pliron::printable::State,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            use ::pliron::op::Op;
            use ::pliron::irfmt::printers::iter_with_sep;
            use ::pliron::common_traits::Named;
            let op = self.get_operation().deref(ctx);
            if op.get_num_results() > 0 {
                let sep = ::pliron::printable::ListSeparator::CharSpace(',');
                let results = iter_with_sep(op.results(), sep);
                fmt.write_fmt(format_args!("{0} = ", results.disp(ctx)))?;
            }
            fmt.write_fmt(format_args!("{0} ", self.get_opid()))?;
            let opd = self.get_operation().deref(ctx).get_operand(0usize);
            ::pliron::printable::Printable::fmt(&opd, ctx, state, fmt)?;
            ::pliron::printable::Printable::fmt(&",", ctx, state, fmt)?;
            let opd = self.get_operation().deref(ctx).get_operand(1usize);
            ::pliron::printable::Printable::fmt(&opd, ctx, state, fmt)?;
            ::pliron::printable::Printable::fmt(&":", ctx, state, fmt)?;
            let res = self.get_operation().deref(ctx).get_type(0usize);
            ::pliron::printable::Printable::fmt(&res, ctx, state, fmt)?;
            Ok(())
        }
    }
    impl ::pliron::parsable::Parsable for ISubOp {
        type Arg = Vec<(
            ::pliron::identifier::Identifier,
            ::pliron::location::Location,
        )>;
        type Parsed = ::pliron::op::OpObj;
        fn parse<'a>(
            state_stream: &mut ::pliron::parsable::StateStream<'a>,
            arg: Self::Arg,
        ) -> ::pliron::parsable::ParseResult<'a, Self::Parsed> {
            use ::pliron::parsable::IntoParseResult;
            use ::combine::Parser;
            use ::pliron::input_err;
            use ::pliron::location::Located;
            let cur_loc = state_stream.loc();
            use ::pliron::op::Op;
            use ::pliron::operation::Operation;
            use ::pliron::irfmt::parsers::{
                process_parsed_ssa_defs, ssa_opd_parser, block_opd_parser, attr_parser,
            };
            let opd_0 = ssa_opd_parser().parse_stream(state_stream).into_result()?.0;
            ::pliron::irfmt::parsers::spaced(::combine::parser::char::string(","))
                .parse_stream(state_stream)
                .into_result()?;
            let opd_1 = ssa_opd_parser().parse_stream(state_stream).into_result()?.0;
            ::pliron::irfmt::parsers::spaced(::combine::parser::char::string(":"))
                .parse_stream(state_stream)
                .into_result()?;
            let res_0 = ::pliron::irfmt::parsers::type_parser()
                .parse_stream(state_stream)
                .into_result()?
                .0;
            if arg.len() != 1usize {
                return Err(::pliron::result::Error {
                    kind: ::pliron::result::ErrorKind::InvalidInput,
                    err: Box::new(::pliron::result::StringError(::alloc::__export::must_use(
                        {
                            let res = ::alloc::fmt::format(format_args!(
                                "expected {0} results as per spec, got {1} during parsing",
                                1usize,
                                arg.len()
                            ));
                            res
                        },
                    ))),
                    loc: cur_loc,
                    backtrace: std::backtrace::Backtrace::capture(),
                })?;
            }
            let op = ::pliron::operation::Operation::new(
                state_stream.state.ctx,
                Self::get_opid_static(),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([res_0]),
                ),
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([opd_0, opd_1]),
                ),
                ::alloc::vec::Vec::new(),
                0,
            );
            for region in ::alloc::vec::Vec::new() {
                ::pliron::region::Region::move_to_op(region, op, state_stream.state.ctx);
            }
            process_parsed_ssa_defs(state_stream, &arg, op)?;
            let final_ret_value = Operation::get_op(op, state_stream.state.ctx);
            Ok(final_ret_value).into_parse_result()
        }
    }
    /// Equivalent to CLIF's standard integer subtraction (with no overflow) opcode.
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
    pub struct ISubOp {
        op: ::pliron::context::Ptr<::pliron::operation::Operation>,
    }
    impl OneResultInterface for ISubOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<ISubOp>(),
                        std::any::TypeId::of::<dyn OneResultInterface>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn OneResultInterface + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn OneResultInterface + 'static)> {
            r.downcast_ref::<ISubOp>()
                .map(|s| s as &dyn OneResultInterface)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    ISubOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn OneResultInterface>(),
                        <ISubOp as OneResultInterface>::verify,
                    ),
                )
            })
        };
    };
    impl SameOperandsType for ISubOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<ISubOp>(),
                        std::any::TypeId::of::<dyn SameOperandsType>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn SameOperandsType + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn SameOperandsType + 'static)> {
            r.downcast_ref::<ISubOp>()
                .map(|s| s as &dyn SameOperandsType)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    ISubOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn SameOperandsType>(),
                        <ISubOp as SameOperandsType>::verify,
                    ),
                )
            })
        };
    };
    impl SameResultsType for ISubOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<ISubOp>(),
                        std::any::TypeId::of::<dyn SameResultsType>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn SameResultsType + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn SameResultsType + 'static)> {
            r.downcast_ref::<ISubOp>()
                .map(|s| s as &dyn SameResultsType)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    ISubOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn SameResultsType>(),
                        <ISubOp as SameResultsType>::verify,
                    ),
                )
            })
        };
    };
    impl SameOperandsAndResultType for ISubOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<ISubOp>(),
                        std::any::TypeId::of::<dyn SameOperandsAndResultType>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn SameOperandsAndResultType + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn SameOperandsAndResultType + 'static)> {
            r.downcast_ref::<ISubOp>()
                .map(|s| s as &dyn SameOperandsAndResultType)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    ISubOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn SameOperandsAndResultType>(),
                        <ISubOp as SameOperandsAndResultType>::verify,
                    ),
                )
            })
        };
    };
    impl BinArithOp for ISubOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<ISubOp>(),
                        std::any::TypeId::of::<dyn BinArithOp>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn BinArithOp + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn BinArithOp + 'static)> {
            r.downcast_ref::<ISubOp>().map(|s| s as &dyn BinArithOp)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    ISubOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn BinArithOp>(),
                        <ISubOp as BinArithOp>::verify,
                    ),
                )
            })
        };
    };
    impl IntBinArithOp for ISubOp {}
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeNzcB60jH,regular,no_dead_strip")]
        static CAST_TO_TRAIT: std::sync::LazyLock<(
            (std::any::TypeId, std::any::TypeId),
            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync + Send>,
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<((std::any::TypeId,
                                            std::any::TypeId),
                                            Box<dyn ::pliron::utils::trait_cast::ClonableAny + Sync +
                                            Send>)> { || &CAST_TO_TRAIT };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::utils::trait_cast::TRAIT_CASTERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    (
                        std::any::TypeId::of::<ISubOp>(),
                        std::any::TypeId::of::<dyn IntBinArithOp>(),
                    ),
                    Box::new(
                        cast_to_trait
                            as for<'a> fn(
                                &'a (dyn std::any::Any + 'static),
                            )
                                -> Option<&'a (dyn IntBinArithOp + 'static)>,
                    ),
                )
            })
        };
        fn cast_to_trait<'a>(
            r: &'a (dyn std::any::Any + 'static),
        ) -> Option<&'a (dyn IntBinArithOp + 'static)> {
            r.downcast_ref::<ISubOp>().map(|s| s as &dyn IntBinArithOp)
        }
    };
    const _: () = {
        #[used]
        #[unsafe(link_section = "__DATA,__linkmeym4ebP38,regular,no_dead_strip")]
        static INTERFACE_VERIFIER: std::sync::LazyLock<(
            ::pliron::op::OpId,
            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier),
        )> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: ::linkme::__private::Void) {
                #[allow(clippy :: ref_option_ref)]
                        let __new =
                            ||
                                ->
                                    fn()
                                        ->
                                            &'static std::sync::LazyLock<(::pliron::op::OpId,
                                            (std::any::TypeId, ::pliron::op::OpInterfaceVerifier))>
                                { || &INTERFACE_VERIFIER };
                unsafe {
                    ::linkme::DistributedSlice::private_typecheck(
                        ::pliron::op::OP_INTERFACE_VERIFIERS,
                        __new(),
                    );
                }
            }
            std::sync::LazyLock::new(|| {
                (
                    ISubOp::get_opid_static(),
                    (
                        std::any::TypeId::of::<dyn IntBinArithOp>(),
                        <ISubOp as IntBinArithOp>::verify,
                    ),
                )
            })
        };
    };
    #[automatically_derived]
    impl ::core::clone::Clone for ISubOp {
        #[inline]
        fn clone(&self) -> ISubOp {
            let _: ::core::clone::AssertParamIsClone<
                ::pliron::context::Ptr<::pliron::operation::Operation>,
            >;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ISubOp {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ISubOp {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ISubOp {
        #[inline]
        fn eq(&self, other: &ISubOp) -> bool {
            self.op == other.op
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ISubOp {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                ::pliron::context::Ptr<::pliron::operation::Operation>,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ISubOp {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.op, state)
        }
    }
    impl ::pliron::op::Op for ISubOp {
        fn get_operation(&self) -> ::pliron::context::Ptr<::pliron::operation::Operation> {
            self.op
        }
        fn wrap_operation(
            op: ::pliron::context::Ptr<::pliron::operation::Operation>,
        ) -> ::pliron::op::OpObj {
            Box::new(ISubOp { op })
        }
        fn get_opid(&self) -> ::pliron::op::OpId {
            Self::get_opid_static()
        }
        fn get_opid_static() -> ::pliron::op::OpId {
            ::pliron::op::OpId {
                name: ::pliron::op::OpName::new("isub"),
                dialect: ::pliron::dialect::DialectName::new("clif"),
            }
        }
        fn verify_interfaces(
            &self,
            ctx: &::pliron::context::Context,
        ) -> ::pliron::result::Result<()> {
            if let Some(interface_verifiers) =
                ::pliron::op::OP_INTERFACE_VERIFIERS_MAP.get(&Self::get_opid_static())
            {
                for (_, verifier) in interface_verifiers {
                    verifier(self, ctx)?;
                }
            }
            Ok(())
        }
    }
    impl ::pliron::common_traits::Verify for ISubOp {
        fn verify(&self, _ctx: &::pliron::context::Context) -> ::pliron::result::Result<()> {
            Ok(())
        }
    }
}
