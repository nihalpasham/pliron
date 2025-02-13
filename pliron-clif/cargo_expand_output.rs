#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod ops {
    //! [Op]s defined in the CLIF dialect
    use pliron::{
        builtin::op_interfaces::IsTerminatorInterface, context::Context, impl_canonical_syntax,
        impl_verify_succ, op::Op, operation::Operation, value::Value,
    };
    use pliron::derive::{def_op, derive_op_interface_impl};
    /// Equivalent to CLIF's return opcode.
    ///
    /// Operands:
    ///
    /// | operand | description |
    /// |-----|-------|
    /// | `arg` | any type |
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
}
