use swc_common::Span;

use crate::{
    Ast,
    ast::module::Module,
    node_id::{AtomRef, OptionalAtomRef, SubRange},
};

pub trait Visit {
    fn visit_module(&mut self, node: Module, ast: &Ast) {
        <Module as VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_span(&mut self, node: Span, ast: &Ast) {
        <Span as VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_sub_range(&mut self, node: SubRange, ast: &Ast) {
        <SubRange as VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_opt_atom_ref(&mut self, node: OptionalAtomRef, ast: &Ast) {
        <OptionalAtomRef as VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_atom_ref(&mut self, node: AtomRef, ast: &Ast) {
        <AtomRef as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
}

pub trait VisitWith<V: ?Sized + Visit> {
    fn visit_with(self, visitor: &mut V, ast: &Ast);
    fn visit_children_with(self, visitor: &mut V, ast: &Ast);
}

impl<V: ?Sized + Visit> VisitWith<V> for Module {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_module(visitor, self, ast)
    }

    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <swc_common::Span as VisitWith<V>>::visit_with(self.span(ast), visitor, ast);
        <SubRange as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
        <OptionalAtomRef as VisitWith<V>>::visit_with(self.shebang(ast), visitor, ast);
    }
}

impl<V: ?Sized + Visit> VisitWith<V> for Span {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_span(visitor, self, ast)
    }

    fn visit_children_with(self, _visitor: &mut V, _ast: &Ast) {}
}

impl<V: ?Sized + Visit> VisitWith<V> for SubRange {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_sub_range(visitor, self, ast)
    }

    fn visit_children_with(self, _visitor: &mut V, _ast: &Ast) {}
}

impl<V: ?Sized + Visit> VisitWith<V> for OptionalAtomRef {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_atom_ref(visitor, self, ast)
    }

    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        if let Some(atom_ref) = self.unwrap() {
            <AtomRef as VisitWith<V>>::visit_with(atom_ref, visitor, ast);
        }
    }
}

impl<V: ?Sized + Visit> VisitWith<V> for AtomRef {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_atom_ref(visitor, self, ast)
    }

    fn visit_children_with(self, _visitor: &mut V, _ast: &Ast) {}
}
