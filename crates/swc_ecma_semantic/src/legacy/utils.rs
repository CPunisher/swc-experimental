use rspack_experimental_swc_ecma_ast::{Ast, Expr, Ident, PropName};
use rspack_experimental_swc_ecma_visit::{Visit, VisitWith};
use swc_atoms::Atom;

pub struct DestructuringFinder {
    pub found: Vec<Atom>,
}

pub fn find_pat_ids<N: VisitWith<DestructuringFinder>>(ast: &Ast, node: N) -> Vec<Atom> {
    let mut v = DestructuringFinder { found: Vec::new() };
    node.visit_with(&mut v, ast);

    v.found
}

impl Visit for DestructuringFinder {
    /// No-op (we don't care about expressions)
    fn visit_expr(&mut self, _: Expr, ast: &Ast) {}

    fn visit_ident(&mut self, i: Ident, ast: &Ast) {
        self.found.push(ast.get_atom(i.sym(ast)).clone());
    }

    // fn visit_jsx_member_expr(&mut self, n: &JSXMemberExpr) {
    //     n.obj.visit_with(self);
    // }

    /// No-op (we don't care about expressions)
    fn visit_prop_name(&mut self, _: PropName, _ast: &Ast) {}

    // fn visit_ts_type(&mut self, _: &TsType) {}
}
