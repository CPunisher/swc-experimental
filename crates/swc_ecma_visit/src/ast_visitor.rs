use rspack_experimental_swc_ecma_ast::{
    AssignOp, AtomRef, BigIntId, BinaryOp, ImportPhase, MetaPropKind, MethodKind, OptionalAtomRef,
    OptionalWtf8AtomId, UnaryOp, UpdateOp, VarDeclKind, Wtf8AtomId,
};
use swc_common::Span;

use crate::{Visit, VisitMut, VisitMutWith, VisitWith};

macro_rules! dummy_visit_mut_impl {
    ($ident:ident) => {
        impl<V: ?Sized + Visit> VisitWith<V> for $ident {
            #[inline]
            fn visit_with(self, _visitor: &mut V, _ast: &rspack_experimental_swc_ecma_ast::Ast) {}

            #[inline]
            fn visit_children_with(
                self,
                _visitor: &mut V,
                _ast: &rspack_experimental_swc_ecma_ast::Ast,
            ) {
            }
        }

        impl<V: ?Sized + VisitMut> VisitMutWith<V> for $ident {
            #[inline]
            fn visit_mut_with(
                self,
                _visitor: &mut V,
                _ast: &mut rspack_experimental_swc_ecma_ast::Ast,
            ) {
            }

            #[inline]
            fn visit_mut_children_with(
                self,
                _visitor: &mut V,
                _ast: &mut rspack_experimental_swc_ecma_ast::Ast,
            ) {
            }
        }
    };
}

dummy_visit_mut_impl!(bool);
dummy_visit_mut_impl!(f64);
dummy_visit_mut_impl!(Span);
dummy_visit_mut_impl!(BigIntId);
dummy_visit_mut_impl!(AtomRef);
dummy_visit_mut_impl!(OptionalAtomRef);
dummy_visit_mut_impl!(Wtf8AtomId);
dummy_visit_mut_impl!(OptionalWtf8AtomId);
dummy_visit_mut_impl!(ImportPhase);
dummy_visit_mut_impl!(VarDeclKind);
dummy_visit_mut_impl!(UnaryOp);
dummy_visit_mut_impl!(UpdateOp);
dummy_visit_mut_impl!(BinaryOp);
dummy_visit_mut_impl!(AssignOp);
dummy_visit_mut_impl!(MetaPropKind);
dummy_visit_mut_impl!(MethodKind);
