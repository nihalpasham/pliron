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
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use pliron::{
    context::Context,
    dialect::{Dialect, DialectName},
};

pub mod attributes {}
pub mod from_clif_ir {

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
    use rustc_hash::FxHashMap;
    use crate::{
        op_interfaces::BinArithOp,
        ops::{IAddOp, ReturnOp},
    };
    /// Converts a slice of [ClifValue]s to Pliron's [PlironValue]s.
    ///
    /// This function processes each operand, determining if it is defined by an instruction or
    /// a block parameter, and converts it into the corresponding `PlironValue`.
    ///
    /// # Arguments
    /// - `ctx`: The Pliron context for creating IR entities.
    /// - `dfg`: The data flow graph containing the original operand definitions.
    /// - `cctx`: The conversion context for caching converted entities.
    /// - `operands`: The slice of `ClifValue` to convert.
    ///
    /// # Returns
    /// A `Result` containing a vector of converted `PlironValue` or an error if conversion fails.
    ///
    /// # Panics
    /// Panics if a `Union` value definition is encountered, as it is not yet implemented.
    fn convert_operands(
        ctx: &mut Context,
        dfg: &DataFlowGraph,
        cctx: &mut ConversionCtx,
        operands: &[ClifValue],
    ) -> Result<Vec<PlironValue>> {
        let mut pliron_operands = Vec::with_capacity(operands.len());
        for operand in operands {
            let operand_def = dfg.value_def(*operand);
            match operand_def {
                ValueDef::Result(inst, idx) => {
                    let op = convert_instruction(ctx, dfg, cctx, inst)?;
                    let pliron_value = PlironValue::OpResult { op, res_idx: idx };
                    pliron_operands.push(pliron_value);
                }
                ValueDef::Param(block, idx) => {
                    let block = convert_block(ctx, dfg, cctx, block)?;
                    let pliron_value = PlironValue::BlockArgument {
                        block,
                        arg_idx: idx,
                    };
                    pliron_operands.push(pliron_value);
                }
                ValueDef::Union(_value, _value1) => ::core::panicking::panic("not yet implemented"),
            }
        }
        Ok(pliron_operands)
    }
    /// Converts a Cranelift [Inst] to Pliron's [Ptr<Operation>].
    ///
    /// This function checks if the instruction has already been converted (using `cctx` for caching).
    /// If not, it converts the instruction based on its opcode (like `Iadd` and `Return`).
    ///
    /// # Arguments
    /// - `ctx`: The Pliron context for creating IR entities.
    /// - `dfg`: The data flow graph containing the original instruction's information.
    /// - `cctx`: The conversion context for caching converted entities.
    /// - `inst`: The Cranelift `Inst` to convert.
    ///
    /// # Returns
    /// A `Result` containing the converted `Operation` or an error if conversion fails.
    ///
    /// # Panics
    /// Panics if the opcode is not yet implemented
    fn convert_instruction(
        ctx: &mut Context,
        dfg: &DataFlowGraph,
        cctx: &mut ConversionCtx,
        inst: Inst,
    ) -> Result<Ptr<Operation>> {
        if let Some(op) = cctx.ops.get(&inst) {
            return Ok(*op);
        };
        let clif_opcode = dfg.insts[inst].opcode();
        let inst_args = dfg.inst_args(inst);
        match clif_opcode {
            Opcode::Iadd => {
                let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
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
                    let operands = convert_operands(ctx, dfg, cctx, &inst_args)?;
                    let return_op = ReturnOp::new(ctx, Some(operands[0]));
                    let op = return_op.get_operation();
                    return Ok(op);
                }
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "not implemented: {0}",
                        format_args!("Multiple return values are not supported")
                    ));
                }
            },
            _ => {
                ::core::panicking::panic_fmt(format_args!(
                    "not implemented: {0}",
                    format_args!("Opcode {0} is not implemented", clif_opcode)
                ));
            }
        }
    }
    /// Converts a [ClifBasicBlock] to Pliron's [BasicBlock].
    ///
    /// This function checks if the block has already been converted (using `cctx` for caching).
    /// If not, it creates a new `BasicBlock` in Pliron's IR, deriving its label and argument types
    /// from the provided `ClifBasicBlock`.
    ///
    /// # Arguments
    /// - `ctx`: The Pliron context for creating IR entities.
    /// - `dfg`: The data flow graph containing the original block's information.
    /// - `cctx`: The conversion context for caching converted entities.
    /// - `block`: The `ClifBasicBlock` to convert.
    ///
    /// # Returns
    /// A `Result` containing the converted `BasicBlock` or an error if conversion fails.
    fn convert_block(
        ctx: &mut Context,
        dfg: &DataFlowGraph,
        cctx: &mut ConversionCtx,
        block: ClifBasicBlock,
    ) -> Result<Ptr<BasicBlock>> {
        if let Some(bb) = cctx.bbs.get(&block) {
            return Ok(*bb);
        };
        let label = Identifier::try_new(::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}", block));
            res
        }))?;
        let block_params = dfg.block_params(block);
        let mut arg_types = ::alloc::vec::Vec::new();
        for param in block_params {
            let param_type = dfg.value_type(*param);
            let pliron_type = convert_type(ctx, param_type)?;
            arg_types.push(pliron_type);
        }
        let pliron_block = BasicBlock::new(ctx, Some(label), arg_types);
        Ok(pliron_block)
    }
    /// Converts a [ClifType] to Pliron's [TypeObj].
    ///
    /// This function maps Cranelift types to their corresponding Pliron types. Currently, only `i32`
    /// is supported.
    ///
    /// # Arguments
    /// - `ctx`: The Pliron context for creating IR entities.
    /// - `ty`: The `ClifType` to convert.
    ///
    /// # Returns
    /// A `Result` containing the converted `TypeObj` or an error if conversion fails.
    ///
    /// # Panics
    /// Panics if the provided type is not yet implemented.
    fn convert_type(ctx: &mut Context, ty: ClifType) -> Result<Ptr<TypeObj>> {
        match ty.to_string().as_str() {
            "i32" => {
                let pliron_int_type_ptr = IntegerType::get(ctx, 32, Signedness::Signed);
                return Ok(pliron_int_type_ptr.to_ptr());
            }
            _ => {
                ::core::panicking::panic_fmt(format_args!(
                    "not implemented: {0}",
                    format_args!("Type {0:?} is not implemented", ty)
                ));
            }
        }
    }
    /// Converts a Cranelift [Function] to a Pliron [FuncOp].
    ///
    /// This function translates all Cranelift entities (instructions, blocks, operands, types, etc.)
    /// into their Pliron equivalents and stores them in the conversion context (`cctx`).
    ///
    /// # Arguments
    /// - `ctx`: The Pliron context for creating IR entities.
    /// - `cctx`: The conversion context for caching converted entities.
    /// - `func`: The Cranelift `Function` to convert.
    ///
    /// # Returns
    /// A `Result` containing the converted `FuncOp` or an error if conversion fails.
    ///
    /// # Panics
    /// Panics if the function's entry block or types cannot be converted.
    fn convert_function(
        ctx: &mut Context,
        cctx: &mut ConversionCtx,
        func: Function,
    ) -> Result<FuncOp> {
        fn convert_and_link(ctx: &mut Context, cctx: &mut ConversionCtx, func: Function) {
            let dfg = &func.dfg;
            let mut prev_bb = match cctx.entry_block {
                Some(entry) => entry,
                None => return,
            };
            for (idx, block) in func.layout.blocks().enumerate() {
                let bb = convert_block(ctx, &dfg, cctx, block).unwrap();
                match idx {
                    0 => {
                        cctx.bbs.insert(block, bb);
                    }
                    _ => {
                        bb.insert_after(ctx, prev_bb);
                        cctx.bbs.insert(block, bb);
                        prev_bb = bb;
                    }
                }
                let mut prev_inst = None;
                for (idx, inst) in func.layout.block_insts(block).enumerate() {
                    let op = convert_instruction(ctx, &dfg, cctx, inst).unwrap();
                    match idx {
                        0 => {
                            op.insert_at_front(bb, ctx);
                            cctx.ops.insert(inst, op);
                            prev_inst = Some(inst);
                        }
                        _ => match prev_inst {
                            Some(inst) => {
                                let prev_op = cctx.ops.get(&inst).unwrap();
                                op.insert_after(ctx, *prev_op);
                                cctx.ops.insert(inst, op);
                                prev_inst = Some(inst);
                            }
                            None => ::core::panicking::panic("not yet implemented"),
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
                convert_type(ctx, param.value_type)
                    .expect("Failed to convert Clif Function Parameter Type")
            })
            .collect();
        let func_return_types: Vec<_> = func_type
            .returns
            .iter()
            .map(|ret| {
                convert_type(ctx, ret.value_type)
                    .expect("Failed to convert Clif Function Return Type")
            })
            .collect();
        let pliron_func_type = FunctionType::get(ctx, func_params_types, func_return_types);
        let func_op = FuncOp::new(ctx, &Identifier::try_new(func_name)?, pliron_func_type);
        let pliron_entry_blk = func_op.get_entry_block(ctx);
        if let Some(blk) = func.layout.entry_block() {
            cctx.bbs.insert(blk, pliron_entry_blk);
        } else {
            {
                ::std::io::_print(format_args!("Function has no entry block\n"));
            };
        }
        cctx.entry_block = Some(pliron_entry_blk);
        cctx.regs.push(func_op.get_region(ctx));
        cctx.func_op = Some(func_op.get_operation());
        convert_and_link(ctx, cctx, func);
        Ok(func_op)
    }
    /// Tracks converted Pliron entities during IR transformation.
    ///
    /// This struct ensures efficient lookup and prevents redundant conversions of operations,
    /// regions, and basic blocks during the transformation process.
    ///
    /// # Fields
    /// - `ops`: Maps original instructions to their converted `Operation` entities.
    /// - `regs`: Stores pointers to converted `Region` entities.
    /// - `bbs`: Maps `ClifBasicBlock`s to their corresponding Pliron `BasicBlock`s.
    /// - `entry_block`: The entry `BasicBlock` of the function being processed, if any.
    /// - `func_op`: The root `Operation` representing the function being processed, if any.
    struct ConversionCtx {
        ops: FxHashMap<Inst, Ptr<Operation>>,
        regs: Vec<Ptr<Region>>,
        bbs: FxHashMap<ClifBasicBlock, Ptr<BasicBlock>>,
        entry_block: Option<Ptr<BasicBlock>>,
        func_op: Option<Ptr<Operation>>,
    }
    #[automatically_derived]
    impl ::core::default::Default for ConversionCtx {
        #[inline]
        fn default() -> ConversionCtx {
            ConversionCtx {
                ops: ::core::default::Default::default(),
                regs: ::core::default::Default::default(),
                bbs: ::core::default::Default::default(),
                entry_block: ::core::default::Default::default(),
                func_op: ::core::default::Default::default(),
            }
        }
    }
}
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
    use pliron::derive::{def_op, derive_op_interface_impl};
    use pliron::parsable::Parsable;
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
            ///{}
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
    ///{}
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
    ///{}
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
    pub fn register(ctx: &mut Context) {
        ReturnOp::register(ctx, ReturnOp::parser_fn);
        IAddOp::register(ctx, IAddOp::parser_fn);
        ISubOp::register(ctx, ISubOp::parser_fn);
    }
}
pub fn register(ctx: &mut Context) {
    let dialect = Dialect::new(DialectName::new("clif"));
    dialect.register(ctx);
    ops::register(ctx);
}
