use swc_experimental_ecma_ast::{Ast, Expr, Ident, PropName};
use swc_experimental_ecma_visit::{Visit, VisitWith};

pub struct DestructuringFinder<'ast> {
    ast: &'ast Ast,
    found: Vec<&'ast str>,
}

pub fn find_pat_ids<'ast, N: VisitWith<DestructuringFinder<'ast>>>(
    ast: &'ast Ast,
    node: N,
) -> Vec<&'ast str> {
    let mut v = DestructuringFinder {
        ast,
        found: Vec::new(),
    };
    node.visit_with(&mut v, ast);

    v.found
}

impl<'ast> Visit for DestructuringFinder<'ast> {
    /// No-op (we don't care about expressions)
    fn visit_expr(&mut self, _: Expr, _ast: &Ast) {}

    fn visit_ident(&mut self, i: Ident, _ast: &Ast) {
        self.found.push(self.ast.get_utf8(i.sym(self.ast)));
    }

    // fn visit_jsx_member_expr(&mut self, n: &JSXMemberExpr) {
    //     n.obj.visit_with(self);
    // }

    /// No-op (we don't care about expressions)
    fn visit_prop_name(&mut self, _: PropName, _ast: &Ast) {}

    // fn visit_ts_type(&mut self, _: &TsType) {}
}
