use std::collections::HashSet;

use swc_experimental_ecma_ast::{GetNodeId, NodeId};
use swc_experimental_ecma_parser::{EsSyntax, Parser, StringAllocator, StringSource, Syntax};
use swc_experimental_ecma_visit::{Visit, VisitWith};

fn main() {
    let source = include_str!("../files/typescript.js");
    let syntax = Syntax::Es(EsSyntax::default());
    let input = StringSource::new(source);
    let string_allocator = StringAllocator::new();

    let parser = Parser::new(syntax, input, None, string_allocator);
    let ret = parser.parse_program().unwrap();

    let mut used = Use {
        used: Default::default(),
    };
    ret.root.visit_with(&mut used, &ret.ast);

    for (node_id, node) in ret.ast.nodes() {
        if !used.used.contains(&node_id) {
            let source = &source[node.span.lo.0 as usize..node.span.hi.0 as usize];
            println!("Unused node: {:?}, source: {}", node.kind, source);
        }
    }

    println!(
        "Total: {}, Used: {}",
        ret.ast.nodes().count(),
        used.used.len()
    )
}

struct Use {
    used: HashSet<NodeId>,
}

impl Visit for Use {
    fn visit_program(
        &mut self,
        node: swc_experimental_ecma_ast::Program,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Program as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_module(
        &mut self,
        node: swc_experimental_ecma_ast::Module,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Module as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_script(
        &mut self,
        node: swc_experimental_ecma_ast::Script,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Script as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_module_item(
        &mut self,
        node: swc_experimental_ecma_ast::ModuleItem,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ModuleItem as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_module_decl(
        &mut self,
        node: swc_experimental_ecma_ast::ModuleDecl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ModuleDecl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_import_decl(
        &mut self,
        node: swc_experimental_ecma_ast::ImportDecl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ImportDecl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_import_specifier(
        &mut self,
        node: swc_experimental_ecma_ast::ImportSpecifier,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ImportSpecifier as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_import_named_specifier(
        &mut self,
        node: swc_experimental_ecma_ast::ImportNamedSpecifier,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ImportNamedSpecifier as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_import_default_specifier(
        &mut self,
        node: swc_experimental_ecma_ast::ImportDefaultSpecifier,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ImportDefaultSpecifier as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_import_star_as_specifier(
        &mut self,
        node: swc_experimental_ecma_ast::ImportStarAsSpecifier,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ImportStarAsSpecifier as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_export_decl(
        &mut self,
        node: swc_experimental_ecma_ast::ExportDecl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExportDecl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_named_export(
        &mut self,
        node: swc_experimental_ecma_ast::NamedExport,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::NamedExport as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_export_specifier(
        &mut self,
        node: swc_experimental_ecma_ast::ExportSpecifier,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExportSpecifier as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_export_namespace_specifier(
        &mut self,
        node: swc_experimental_ecma_ast::ExportNamespaceSpecifier,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExportNamespaceSpecifier as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_module_export_name(
        &mut self,
        node: swc_experimental_ecma_ast::ModuleExportName,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ModuleExportName as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_export_default_specifier(
        &mut self,
        node: swc_experimental_ecma_ast::ExportDefaultSpecifier,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExportDefaultSpecifier as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_export_named_specifier(
        &mut self,
        node: swc_experimental_ecma_ast::ExportNamedSpecifier,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExportNamedSpecifier as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_export_default_decl(
        &mut self,
        node: swc_experimental_ecma_ast::ExportDefaultDecl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExportDefaultDecl as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_default_decl(
        &mut self,
        node: swc_experimental_ecma_ast::DefaultDecl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::DefaultDecl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_export_default_expr(
        &mut self,
        node: swc_experimental_ecma_ast::ExportDefaultExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExportDefaultExpr as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_export_all(
        &mut self,
        node: swc_experimental_ecma_ast::ExportAll,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExportAll as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_block_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::BlockStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::BlockStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::Stmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Stmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_expr_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::ExprStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExprStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_empty_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::EmptyStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::EmptyStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_debugger_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::DebuggerStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::DebuggerStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_with_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::WithStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::WithStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_return_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::ReturnStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ReturnStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_labeled_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::LabeledStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::LabeledStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_break_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::BreakStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::BreakStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_continue_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::ContinueStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ContinueStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_if_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::IfStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::IfStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_switch_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::SwitchStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::SwitchStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_throw_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::ThrowStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ThrowStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_try_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::TryStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::TryStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_while_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::WhileStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::WhileStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_do_while_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::DoWhileStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::DoWhileStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_for_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::ForStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ForStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_for_in_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::ForInStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ForInStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_for_of_stmt(
        &mut self,
        node: swc_experimental_ecma_ast::ForOfStmt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ForOfStmt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_switch_case(
        &mut self,
        node: swc_experimental_ecma_ast::SwitchCase,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::SwitchCase as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_catch_clause(
        &mut self,
        node: swc_experimental_ecma_ast::CatchClause,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::CatchClause as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_for_head(
        &mut self,
        node: swc_experimental_ecma_ast::ForHead,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ForHead as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_var_decl_or_expr(
        &mut self,
        node: swc_experimental_ecma_ast::VarDeclOrExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::VarDeclOrExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_decl(
        &mut self,
        node: swc_experimental_ecma_ast::Decl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Decl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_fn_decl(
        &mut self,
        node: swc_experimental_ecma_ast::FnDecl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::FnDecl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_class_decl(
        &mut self,
        node: swc_experimental_ecma_ast::ClassDecl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ClassDecl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_var_decl(
        &mut self,
        node: swc_experimental_ecma_ast::VarDecl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::VarDecl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_var_declarator(
        &mut self,
        node: swc_experimental_ecma_ast::VarDeclarator,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::VarDeclarator as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_using_decl(
        &mut self,
        node: swc_experimental_ecma_ast::UsingDecl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::UsingDecl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_expr(
        &mut self,
        node: swc_experimental_ecma_ast::Expr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Expr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_this_expr(
        &mut self,
        node: swc_experimental_ecma_ast::ThisExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ThisExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_array_lit(
        &mut self,
        node: swc_experimental_ecma_ast::ArrayLit,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ArrayLit as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_object_lit(
        &mut self,
        node: swc_experimental_ecma_ast::ObjectLit,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ObjectLit as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_prop_or_spread(
        &mut self,
        node: swc_experimental_ecma_ast::PropOrSpread,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::PropOrSpread as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_spread_element(
        &mut self,
        node: swc_experimental_ecma_ast::SpreadElement,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::SpreadElement as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_unary_expr(
        &mut self,
        node: swc_experimental_ecma_ast::UnaryExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::UnaryExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_update_expr(
        &mut self,
        node: swc_experimental_ecma_ast::UpdateExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::UpdateExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_bin_expr(
        &mut self,
        node: swc_experimental_ecma_ast::BinExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::BinExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_fn_expr(
        &mut self,
        node: swc_experimental_ecma_ast::FnExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::FnExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_class_expr(
        &mut self,
        node: swc_experimental_ecma_ast::ClassExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ClassExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_assign_expr(
        &mut self,
        node: swc_experimental_ecma_ast::AssignExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::AssignExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_member_expr(
        &mut self,
        node: swc_experimental_ecma_ast::MemberExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::MemberExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_member_prop(
        &mut self,
        node: swc_experimental_ecma_ast::MemberProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::MemberProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_super_prop_expr(
        &mut self,
        node: swc_experimental_ecma_ast::SuperPropExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::SuperPropExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_super_prop(
        &mut self,
        node: swc_experimental_ecma_ast::SuperProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::SuperProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_cond_expr(
        &mut self,
        node: swc_experimental_ecma_ast::CondExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::CondExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_call_expr(
        &mut self,
        node: swc_experimental_ecma_ast::CallExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::CallExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_new_expr(
        &mut self,
        node: swc_experimental_ecma_ast::NewExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::NewExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_seq_expr(
        &mut self,
        node: swc_experimental_ecma_ast::SeqExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::SeqExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_arrow_expr(
        &mut self,
        node: swc_experimental_ecma_ast::ArrowExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ArrowExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_yield_expr(
        &mut self,
        node: swc_experimental_ecma_ast::YieldExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::YieldExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_meta_prop_expr(
        &mut self,
        node: swc_experimental_ecma_ast::MetaPropExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::MetaPropExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_await_expr(
        &mut self,
        node: swc_experimental_ecma_ast::AwaitExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::AwaitExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_tpl(
        &mut self,
        node: swc_experimental_ecma_ast::Tpl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Tpl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_tagged_tpl(
        &mut self,
        node: swc_experimental_ecma_ast::TaggedTpl,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::TaggedTpl as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_tpl_element(
        &mut self,
        node: swc_experimental_ecma_ast::TplElement,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::TplElement as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_paren_expr(
        &mut self,
        node: swc_experimental_ecma_ast::ParenExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ParenExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_callee(
        &mut self,
        node: swc_experimental_ecma_ast::Callee,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Callee as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_super(
        &mut self,
        node: swc_experimental_ecma_ast::Super,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Super as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_import(
        &mut self,
        node: swc_experimental_ecma_ast::Import,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Import as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_expr_or_spread(
        &mut self,
        node: swc_experimental_ecma_ast::ExprOrSpread,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ExprOrSpread as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_spread_dot_3_token(
        &mut self,
        node: swc_experimental_ecma_ast::SpreadDot3Token,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::SpreadDot3Token as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_block_stmt_or_expr(
        &mut self,
        node: swc_experimental_ecma_ast::BlockStmtOrExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::BlockStmtOrExpr as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_assign_target(
        &mut self,
        node: swc_experimental_ecma_ast::AssignTarget,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::AssignTarget as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_assign_target_pat(
        &mut self,
        node: swc_experimental_ecma_ast::AssignTargetPat,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::AssignTargetPat as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_simple_assign_target(
        &mut self,
        node: swc_experimental_ecma_ast::SimpleAssignTarget,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::SimpleAssignTarget as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_opt_chain_expr(
        &mut self,
        node: swc_experimental_ecma_ast::OptChainExpr,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::OptChainExpr as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_opt_chain_base(
        &mut self,
        node: swc_experimental_ecma_ast::OptChainBase,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::OptChainBase as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_opt_call(
        &mut self,
        node: swc_experimental_ecma_ast::OptCall,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::OptCall as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_invalid(
        &mut self,
        node: swc_experimental_ecma_ast::Invalid,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Invalid as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_function(
        &mut self,
        node: swc_experimental_ecma_ast::Function,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Function as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_param(
        &mut self,
        node: swc_experimental_ecma_ast::Param,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Param as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_param_or_ts_param_prop(
        &mut self,
        node: swc_experimental_ecma_ast::ParamOrTsParamProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ParamOrTsParamProp as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_class(
        &mut self,
        node: swc_experimental_ecma_ast::Class,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Class as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_class_member(
        &mut self,
        node: swc_experimental_ecma_ast::ClassMember,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ClassMember as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_class_prop(
        &mut self,
        node: swc_experimental_ecma_ast::ClassProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ClassProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_private_prop(
        &mut self,
        node: swc_experimental_ecma_ast::PrivateProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::PrivateProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_class_method(
        &mut self,
        node: swc_experimental_ecma_ast::ClassMethod,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ClassMethod as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_private_method(
        &mut self,
        node: swc_experimental_ecma_ast::PrivateMethod,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::PrivateMethod as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_constructor(
        &mut self,
        node: swc_experimental_ecma_ast::Constructor,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Constructor as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_decorator(
        &mut self,
        node: swc_experimental_ecma_ast::Decorator,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Decorator as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_static_block(
        &mut self,
        node: swc_experimental_ecma_ast::StaticBlock,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::StaticBlock as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_key(
        &mut self,
        node: swc_experimental_ecma_ast::Key,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Key as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_auto_accessor(
        &mut self,
        node: swc_experimental_ecma_ast::AutoAccessor,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::AutoAccessor as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_prop(
        &mut self,
        node: swc_experimental_ecma_ast::Prop,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Prop as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_key_value_prop(
        &mut self,
        node: swc_experimental_ecma_ast::KeyValueProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::KeyValueProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_assign_prop(
        &mut self,
        node: swc_experimental_ecma_ast::AssignProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::AssignProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_getter_prop(
        &mut self,
        node: swc_experimental_ecma_ast::GetterProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::GetterProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_setter_prop(
        &mut self,
        node: swc_experimental_ecma_ast::SetterProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::SetterProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_method_prop(
        &mut self,
        node: swc_experimental_ecma_ast::MethodProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::MethodProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_prop_name(
        &mut self,
        node: swc_experimental_ecma_ast::PropName,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::PropName as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_computed_prop_name(
        &mut self,
        node: swc_experimental_ecma_ast::ComputedPropName,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ComputedPropName as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_pat(
        &mut self,
        node: swc_experimental_ecma_ast::Pat,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Pat as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_array_pat(
        &mut self,
        node: swc_experimental_ecma_ast::ArrayPat,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ArrayPat as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_object_pat(
        &mut self,
        node: swc_experimental_ecma_ast::ObjectPat,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ObjectPat as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_assign_pat(
        &mut self,
        node: swc_experimental_ecma_ast::AssignPat,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::AssignPat as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_rest_pat(
        &mut self,
        node: swc_experimental_ecma_ast::RestPat,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::RestPat as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_object_pat_prop(
        &mut self,
        node: swc_experimental_ecma_ast::ObjectPatProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::ObjectPatProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_key_value_pat_prop(
        &mut self,
        node: swc_experimental_ecma_ast::KeyValuePatProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::KeyValuePatProp as swc_experimental_ecma_visit::VisitWith<
            Self,
        >>::visit_children_with(node, self, ast)
    }

    fn visit_assign_pat_prop(
        &mut self,
        node: swc_experimental_ecma_ast::AssignPatProp,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::AssignPatProp as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_ident(
        &mut self,
        node: swc_experimental_ecma_ast::Ident,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Ident as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_ident_name(
        &mut self,
        node: swc_experimental_ecma_ast::IdentName,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::IdentName as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_private_name(
        &mut self,
        node: swc_experimental_ecma_ast::PrivateName,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::PrivateName as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_binding_ident(
        &mut self,
        node: swc_experimental_ecma_ast::BindingIdent,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::BindingIdent as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_lit(
        &mut self,
        node: swc_experimental_ecma_ast::Lit,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Lit as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_str(
        &mut self,
        node: swc_experimental_ecma_ast::Str,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Str as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_bool(
        &mut self,
        node: swc_experimental_ecma_ast::Bool,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Bool as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_null(
        &mut self,
        node: swc_experimental_ecma_ast::Null,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Null as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_number(
        &mut self,
        node: swc_experimental_ecma_ast::Number,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Number as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_big_int(
        &mut self,
        node: swc_experimental_ecma_ast::BigInt,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::BigInt as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }

    fn visit_regex(
        &mut self,
        node: swc_experimental_ecma_ast::Regex,
        ast: &swc_experimental_ecma_ast::Ast,
    ) {
        self.used.insert(node.node_id());
        <swc_experimental_ecma_ast::Regex as swc_experimental_ecma_visit::VisitWith<Self>>::visit_children_with(node, self, ast)
    }
}
