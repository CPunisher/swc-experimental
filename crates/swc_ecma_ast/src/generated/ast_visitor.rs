#![allow(unused, clippy::useless_conversion, clippy::single_match)]
use crate::{Ast, ast::*, node_id::*};
use swc_core::common::Span;
pub trait Visit {
    #[inline]
    fn enter_node(&mut self, node_id: NodeId, ast: &Ast) {}
    #[inline]
    fn leave_node(&mut self, node_id: NodeId, ast: &Ast) {}
    #[inline]
    fn visit_program(&mut self, node: Program, ast: &Ast) {
        <Program as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_module(&mut self, node: Module, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Module as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_script(&mut self, node: Script, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Script as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_module_item(&mut self, node: ModuleItem, ast: &Ast) {
        <ModuleItem as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_module_decl(&mut self, node: ModuleDecl, ast: &Ast) {
        <ModuleDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_import_decl(&mut self, node: ImportDecl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ImportDecl as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_import_specifier(&mut self, node: ImportSpecifier, ast: &Ast) {
        <ImportSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_import_named_specifier(&mut self, node: ImportNamedSpecifier, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ImportNamedSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_import_default_specifier(&mut self, node: ImportDefaultSpecifier, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ImportDefaultSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_import_star_as_specifier(&mut self, node: ImportStarAsSpecifier, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ImportStarAsSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_export_decl(&mut self, node: ExportDecl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportDecl as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_named_export(&mut self, node: NamedExport, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <NamedExport as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_export_specifier(&mut self, node: ExportSpecifier, ast: &Ast) {
        <ExportSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_namespace_specifier(&mut self, node: ExportNamespaceSpecifier, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportNamespaceSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_module_export_name(&mut self, node: ModuleExportName, ast: &Ast) {
        <ModuleExportName as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_default_specifier(&mut self, node: ExportDefaultSpecifier, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportDefaultSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_export_named_specifier(&mut self, node: ExportNamedSpecifier, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportNamedSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_export_default_decl(&mut self, node: ExportDefaultDecl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportDefaultDecl as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_default_decl(&mut self, node: DefaultDecl, ast: &Ast) {
        <DefaultDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_default_expr(&mut self, node: ExportDefaultExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportDefaultExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_export_all(&mut self, node: ExportAll, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportAll as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_block_stmt(&mut self, node: BlockStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <BlockStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_stmt(&mut self, node: Stmt, ast: &Ast) {
        <Stmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_expr_stmt(&mut self, node: ExprStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ExprStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_empty_stmt(&mut self, node: EmptyStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <EmptyStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_debugger_stmt(&mut self, node: DebuggerStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <DebuggerStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_with_stmt(&mut self, node: WithStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <WithStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_return_stmt(&mut self, node: ReturnStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ReturnStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_labeled_stmt(&mut self, node: LabeledStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <LabeledStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_break_stmt(&mut self, node: BreakStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <BreakStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_continue_stmt(&mut self, node: ContinueStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ContinueStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_if_stmt(&mut self, node: IfStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <IfStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_switch_stmt(&mut self, node: SwitchStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <SwitchStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_throw_stmt(&mut self, node: ThrowStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ThrowStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_try_stmt(&mut self, node: TryStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <TryStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_while_stmt(&mut self, node: WhileStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <WhileStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_do_while_stmt(&mut self, node: DoWhileStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <DoWhileStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_for_stmt(&mut self, node: ForStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ForStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_for_in_stmt(&mut self, node: ForInStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ForInStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_for_of_stmt(&mut self, node: ForOfStmt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ForOfStmt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_switch_case(&mut self, node: SwitchCase, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <SwitchCase as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_catch_clause(&mut self, node: CatchClause, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <CatchClause as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_for_head(&mut self, node: ForHead, ast: &Ast) {
        <ForHead as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_var_decl_or_expr(&mut self, node: VarDeclOrExpr, ast: &Ast) {
        <VarDeclOrExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_decl(&mut self, node: Decl, ast: &Ast) {
        <Decl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_fn_decl(&mut self, node: FnDecl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <FnDecl as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_class_decl(&mut self, node: ClassDecl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ClassDecl as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_var_decl(&mut self, node: VarDecl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <VarDecl as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_var_declarator(&mut self, node: VarDeclarator, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <VarDeclarator as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_using_decl(&mut self, node: UsingDecl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <UsingDecl as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_expr(&mut self, node: Expr, ast: &Ast) {
        <Expr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_this_expr(&mut self, node: ThisExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ThisExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_array_lit(&mut self, node: ArrayLit, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ArrayLit as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_object_lit(&mut self, node: ObjectLit, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ObjectLit as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_prop_or_spread(&mut self, node: PropOrSpread, ast: &Ast) {
        <PropOrSpread as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_spread_element(&mut self, node: SpreadElement, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <SpreadElement as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_unary_expr(&mut self, node: UnaryExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <UnaryExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_update_expr(&mut self, node: UpdateExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <UpdateExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_bin_expr(&mut self, node: BinExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <BinExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_fn_expr(&mut self, node: FnExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <FnExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_class_expr(&mut self, node: ClassExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ClassExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_assign_expr(&mut self, node: AssignExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <AssignExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_member_expr(&mut self, node: MemberExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <MemberExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_member_prop(&mut self, node: MemberProp, ast: &Ast) {
        <MemberProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_super_prop_expr(&mut self, node: SuperPropExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <SuperPropExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_super_prop(&mut self, node: SuperProp, ast: &Ast) {
        <SuperProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_cond_expr(&mut self, node: CondExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <CondExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_call_expr(&mut self, node: CallExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <CallExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_new_expr(&mut self, node: NewExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <NewExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_seq_expr(&mut self, node: SeqExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <SeqExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_arrow_expr(&mut self, node: ArrowExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ArrowExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_yield_expr(&mut self, node: YieldExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <YieldExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_meta_prop_expr(&mut self, node: MetaPropExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <MetaPropExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_await_expr(&mut self, node: AwaitExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <AwaitExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_tpl(&mut self, node: Tpl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Tpl as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_tagged_tpl(&mut self, node: TaggedTpl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <TaggedTpl as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_tpl_element(&mut self, node: TplElement, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <TplElement as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_paren_expr(&mut self, node: ParenExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ParenExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_callee(&mut self, node: Callee, ast: &Ast) {
        <Callee as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_super(&mut self, node: Super, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Super as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_import(&mut self, node: Import, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Import as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_expr_or_spread(&mut self, node: ExprOrSpread, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ExprOrSpread as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_spread_dot_3_token(&mut self, node: SpreadDot3Token, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <SpreadDot3Token as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_block_stmt_or_expr(&mut self, node: BlockStmtOrExpr, ast: &Ast) {
        <BlockStmtOrExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_assign_target(&mut self, node: AssignTarget, ast: &Ast) {
        <AssignTarget as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_assign_target_pat(&mut self, node: AssignTargetPat, ast: &Ast) {
        <AssignTargetPat as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_simple_assign_target(&mut self, node: SimpleAssignTarget, ast: &Ast) {
        <SimpleAssignTarget as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_chain_expr(&mut self, node: OptChainExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <OptChainExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_opt_chain_base(&mut self, node: OptChainBase, ast: &Ast) {
        <OptChainBase as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_call(&mut self, node: OptCall, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <OptCall as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_invalid(&mut self, node: Invalid, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Invalid as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_function(&mut self, node: Function, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Function as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_param(&mut self, node: Param, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Param as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_param_or_ts_param_prop(&mut self, node: ParamOrTsParamProp, ast: &Ast) {
        <ParamOrTsParamProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_class(&mut self, node: Class, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Class as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_class_member(&mut self, node: ClassMember, ast: &Ast) {
        <ClassMember as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_class_prop(&mut self, node: ClassProp, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ClassProp as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_private_prop(&mut self, node: PrivateProp, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <PrivateProp as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_class_method(&mut self, node: ClassMethod, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ClassMethod as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_private_method(&mut self, node: PrivateMethod, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <PrivateMethod as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_constructor(&mut self, node: Constructor, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Constructor as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_decorator(&mut self, node: Decorator, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Decorator as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_static_block(&mut self, node: StaticBlock, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <StaticBlock as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_key(&mut self, node: Key, ast: &Ast) {
        <Key as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_auto_accessor(&mut self, node: AutoAccessor, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <AutoAccessor as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_prop(&mut self, node: Prop, ast: &Ast) {
        <Prop as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_key_value_prop(&mut self, node: KeyValueProp, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <KeyValueProp as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_assign_prop(&mut self, node: AssignProp, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <AssignProp as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_getter_prop(&mut self, node: GetterProp, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <GetterProp as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_setter_prop(&mut self, node: SetterProp, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <SetterProp as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_method_prop(&mut self, node: MethodProp, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <MethodProp as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_prop_name(&mut self, node: PropName, ast: &Ast) {
        <PropName as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_computed_prop_name(&mut self, node: ComputedPropName, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ComputedPropName as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_pat(&mut self, node: Pat, ast: &Ast) {
        <Pat as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_array_pat(&mut self, node: ArrayPat, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ArrayPat as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_object_pat(&mut self, node: ObjectPat, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <ObjectPat as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_assign_pat(&mut self, node: AssignPat, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <AssignPat as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_rest_pat(&mut self, node: RestPat, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <RestPat as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_object_pat_prop(&mut self, node: ObjectPatProp, ast: &Ast) {
        <ObjectPatProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_key_value_pat_prop(&mut self, node: KeyValuePatProp, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <KeyValuePatProp as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_assign_pat_prop(&mut self, node: AssignPatProp, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <AssignPatProp as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_ident(&mut self, node: Ident, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Ident as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_ident_name(&mut self, node: IdentName, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <IdentName as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_private_name(&mut self, node: PrivateName, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <PrivateName as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_binding_ident(&mut self, node: BindingIdent, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <BindingIdent as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_lit(&mut self, node: Lit, ast: &Ast) {
        <Lit as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_str(&mut self, node: Str, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Str as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_bool(&mut self, node: Bool, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Bool as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_null(&mut self, node: Null, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Null as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_number(&mut self, node: Number, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Number as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_big_int(&mut self, node: BigInt, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <BigInt as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_regex(&mut self, node: Regex, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <Regex as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_object(&mut self, node: JSXObject, ast: &Ast) {
        <JSXObject as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_jsx_member_expr(&mut self, node: JSXMemberExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXMemberExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_namespaced_name(&mut self, node: JSXNamespacedName, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXNamespacedName as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_empty_expr(&mut self, node: JSXEmptyExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXEmptyExpr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_expr_container(&mut self, node: JSXExprContainer, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXExprContainer as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_expr(&mut self, node: JSXExpr, ast: &Ast) {
        <JSXExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_jsx_spread_child(&mut self, node: JSXSpreadChild, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXSpreadChild as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_element_name(&mut self, node: JSXElementName, ast: &Ast) {
        <JSXElementName as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_jsx_opening_element(&mut self, node: JSXOpeningElement, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXOpeningElement as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_attr_or_spread(&mut self, node: JSXAttrOrSpread, ast: &Ast) {
        <JSXAttrOrSpread as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_jsx_closing_element(&mut self, node: JSXClosingElement, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXClosingElement as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_attr(&mut self, node: JSXAttr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXAttr as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_attr_name(&mut self, node: JSXAttrName, ast: &Ast) {
        <JSXAttrName as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_jsx_attr_value(&mut self, node: JSXAttrValue, ast: &Ast) {
        <JSXAttrValue as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_jsx_text(&mut self, node: JSXText, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXText as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_element(&mut self, node: JSXElement, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXElement as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_element_child(&mut self, node: JSXElementChild, ast: &Ast) {
        <JSXElementChild as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_jsx_fragment(&mut self, node: JSXFragment, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXFragment as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_opening_fragment(&mut self, node: JSXOpeningFragment, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXOpeningFragment as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_jsx_closing_fragment(&mut self, node: JSXClosingFragment, ast: &Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXClosingFragment as VisitWith<Self>>::visit_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_module_items(&mut self, node: TypedSubRange<ModuleItem>, ast: &Ast) {
        <TypedSubRange<ModuleItem> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_stmts(&mut self, node: TypedSubRange<Stmt>, ast: &Ast) {
        <TypedSubRange<Stmt> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_import_specifiers(&mut self, node: TypedSubRange<ImportSpecifier>, ast: &Ast) {
        <TypedSubRange<ImportSpecifier> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_object_lit(&mut self, node: Option<ObjectLit>, ast: &Ast) {
        <Option<ObjectLit> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_module_export_name(&mut self, node: Option<ModuleExportName>, ast: &Ast) {
        <Option<ModuleExportName> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_specifiers(&mut self, node: TypedSubRange<ExportSpecifier>, ast: &Ast) {
        <TypedSubRange<ExportSpecifier> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_str(&mut self, node: Option<Str>, ast: &Ast) {
        <Option<Str> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_expr(&mut self, node: Option<Expr>, ast: &Ast) {
        <Option<Expr> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_ident(&mut self, node: Option<Ident>, ast: &Ast) {
        <Option<Ident> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_stmt(&mut self, node: Option<Stmt>, ast: &Ast) {
        <Option<Stmt> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_switch_cases(&mut self, node: TypedSubRange<SwitchCase>, ast: &Ast) {
        <TypedSubRange<SwitchCase> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_catch_clause(&mut self, node: Option<CatchClause>, ast: &Ast) {
        <Option<CatchClause> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_block_stmt(&mut self, node: Option<BlockStmt>, ast: &Ast) {
        <Option<BlockStmt> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_var_decl_or_expr(&mut self, node: Option<VarDeclOrExpr>, ast: &Ast) {
        <Option<VarDeclOrExpr> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_pat(&mut self, node: Option<Pat>, ast: &Ast) {
        <Option<Pat> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_var_declarators(&mut self, node: TypedSubRange<VarDeclarator>, ast: &Ast) {
        <TypedSubRange<VarDeclarator> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_expr_or_spread(&mut self, node: Option<ExprOrSpread>, ast: &Ast) {
        <Option<ExprOrSpread> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_expr_or_spreads(&mut self, node: TypedSubRange<Option<ExprOrSpread>>, ast: &Ast) {
        <TypedSubRange<Option<ExprOrSpread>> as VisitWith<Self>>::visit_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_prop_or_spreads(&mut self, node: TypedSubRange<PropOrSpread>, ast: &Ast) {
        <TypedSubRange<PropOrSpread> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_expr_or_spreads(&mut self, node: TypedSubRange<ExprOrSpread>, ast: &Ast) {
        <TypedSubRange<ExprOrSpread> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_exprs(&mut self, node: TypedSubRange<Expr>, ast: &Ast) {
        <TypedSubRange<Expr> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_pats(&mut self, node: TypedSubRange<Pat>, ast: &Ast) {
        <TypedSubRange<Pat> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_tpl_elements(&mut self, node: TypedSubRange<TplElement>, ast: &Ast) {
        <TypedSubRange<TplElement> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_spread_dot_3_token(&mut self, node: Option<SpreadDot3Token>, ast: &Ast) {
        <Option<SpreadDot3Token> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_params(&mut self, node: TypedSubRange<Param>, ast: &Ast) {
        <TypedSubRange<Param> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_decorators(&mut self, node: TypedSubRange<Decorator>, ast: &Ast) {
        <TypedSubRange<Decorator> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_class_members(&mut self, node: TypedSubRange<ClassMember>, ast: &Ast) {
        <TypedSubRange<ClassMember> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_param_or_ts_param_props(
        &mut self,
        node: TypedSubRange<ParamOrTsParamProp>,
        ast: &Ast,
    ) {
        <TypedSubRange<ParamOrTsParamProp> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_pats(&mut self, node: TypedSubRange<Option<Pat>>, ast: &Ast) {
        <TypedSubRange<Option<Pat>> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_object_pat_props(&mut self, node: TypedSubRange<ObjectPatProp>, ast: &Ast) {
        <TypedSubRange<ObjectPatProp> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_jsx_attr_or_spreads(&mut self, node: TypedSubRange<JSXAttrOrSpread>, ast: &Ast) {
        <TypedSubRange<JSXAttrOrSpread> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_jsx_attr_value(&mut self, node: Option<JSXAttrValue>, ast: &Ast) {
        <Option<JSXAttrValue> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_jsx_element_childs(&mut self, node: TypedSubRange<JSXElementChild>, ast: &Ast) {
        <TypedSubRange<JSXElementChild> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_jsx_closing_element(&mut self, node: Option<JSXClosingElement>, ast: &Ast) {
        <Option<JSXClosingElement> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
}
pub trait VisitWith<V: ?Sized + Visit> {
    fn visit_with(self, visitor: &mut V, ast: &Ast);
    fn visit_children_with(self, visitor: &mut V, ast: &Ast);
}
impl<V: ?Sized + Visit> VisitWith<V> for Program {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_program(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Module(it) => <Module as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Script(it) => <Script as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Module {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_module(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<ModuleItem> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Script {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_script(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Stmt> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ModuleItem {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_module_item(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::ModuleDecl(it) => <ModuleDecl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Stmt(it) => <Stmt as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ModuleDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_module_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Import(it) => <ImportDecl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::ExportDecl(it) => <ExportDecl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::ExportNamed(it) => <NamedExport as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::ExportDefaultDecl(it) => {
                <ExportDefaultDecl as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::ExportDefaultExpr(it) => {
                <ExportDefaultExpr as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::ExportAll(it) => <ExportAll as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ImportDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<ImportSpecifier> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Str as VisitWith<V>>::visit_with(
            unsafe { Str::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .optional_node
        };
        <Option<ObjectLit> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { ObjectLit::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 4usize).index())
                .other
        };
        <ImportPhase as VisitWith<V>>::visit_with(ImportPhase::from_extra_data(ret), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ImportSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Named(it) => <ImportNamedSpecifier as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Default(it) => {
                <ImportDefaultSpecifier as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::Namespace(it) => {
                <ImportStarAsSpecifier as VisitWith<V>>::visit_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ImportNamedSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import_named_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitWith<V>>::visit_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<ModuleExportName> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { ModuleExportName::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ImportDefaultSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import_default_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitWith<V>>::visit_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ImportStarAsSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import_star_as_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitWith<V>>::visit_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Decl as VisitWith<V>>::visit_with(
            unsafe { Decl::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for NamedExport {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_named_export(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<ExportSpecifier> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Str> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Str::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .optional_node
        };
        <Option<ObjectLit> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { ObjectLit::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Namespace(it) => {
                <ExportNamespaceSpecifier as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::Default(it) => {
                <ExportDefaultSpecifier as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::Named(it) => <ExportNamedSpecifier as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportNamespaceSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_namespace_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <ModuleExportName as VisitWith<V>>::visit_with(
            unsafe { ModuleExportName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ModuleExportName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_module_export_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Ident(it) => <Ident as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Str(it) => <Str as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportDefaultSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_default_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitWith<V>>::visit_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportNamedSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_named_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <ModuleExportName as VisitWith<V>>::visit_with(
            unsafe { ModuleExportName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<ModuleExportName> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { ModuleExportName::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportDefaultDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_default_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <DefaultDecl as VisitWith<V>>::visit_with(
            unsafe { DefaultDecl::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for DefaultDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_default_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Class(it) => <ClassExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Fn(it) => <FnExpr as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportDefaultExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_default_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportAll {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_all(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Str as VisitWith<V>>::visit_with(
            unsafe { Str::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<ObjectLit> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { ObjectLit::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BlockStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_block_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Stmt> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Stmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Block(it) => <BlockStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Empty(it) => <EmptyStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Debugger(it) => <DebuggerStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::With(it) => <WithStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Return(it) => <ReturnStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Labeled(it) => <LabeledStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Break(it) => <BreakStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Continue(it) => <ContinueStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::If(it) => <IfStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Switch(it) => <SwitchStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Throw(it) => <ThrowStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Try(it) => <TryStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::While(it) => <WhileStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::DoWhile(it) => <DoWhileStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::For(it) => <ForStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::ForIn(it) => <ForInStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::ForOf(it) => <ForOfStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Decl(it) => <Decl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Expr(it) => <ExprStmt as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExprStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_expr_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for EmptyStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_empty_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for DebuggerStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_debugger_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for WithStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_with_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitWith<V>>::visit_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ReturnStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_return_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for LabeledStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_labeled_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitWith<V>>::visit_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitWith<V>>::visit_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BreakStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_break_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Ident> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Ident::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ContinueStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_continue_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Ident> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Ident::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for IfStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_if_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitWith<V>>::visit_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<Stmt> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Stmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SwitchStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_switch_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<SwitchCase> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ThrowStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_throw_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TryStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_try_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <BlockStmt as VisitWith<V>>::visit_with(
            unsafe { BlockStmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<CatchClause> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { CatchClause::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for WhileStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_while_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitWith<V>>::visit_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for DoWhileStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_do_while_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitWith<V>>::visit_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ForStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_for_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<VarDeclOrExpr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { VarDeclOrExpr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .node
        };
        <Stmt as VisitWith<V>>::visit_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ForInStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_for_in_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <ForHead as VisitWith<V>>::visit_with(
            unsafe { ForHead::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Stmt as VisitWith<V>>::visit_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ForOfStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_for_of_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <ForHead as VisitWith<V>>::visit_with(
            unsafe { ForHead::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .node
        };
        <Stmt as VisitWith<V>>::visit_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SwitchCase {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_switch_case(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<Stmt> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for CatchClause {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_catch_clause(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Pat> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Pat::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <BlockStmt as VisitWith<V>>::visit_with(
            unsafe { BlockStmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ForHead {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_for_head(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::VarDecl(it) => <VarDecl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::UsingDecl(it) => <UsingDecl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Pat(it) => <Pat as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for VarDeclOrExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_var_decl_or_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::VarDecl(it) => <VarDecl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Decl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Class(it) => <ClassDecl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Fn(it) => <FnDecl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Var(it) => <VarDecl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Using(it) => <UsingDecl as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for FnDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_fn_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitWith<V>>::visit_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Function as VisitWith<V>>::visit_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ClassDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitWith<V>>::visit_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Class as VisitWith<V>>::visit_with(
            unsafe { Class::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for VarDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_var_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <VarDeclKind as VisitWith<V>>::visit_with(VarDeclKind::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .sub_range
        };
        <TypedSubRange<VarDeclarator> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for VarDeclarator {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_var_declarator(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Pat as VisitWith<V>>::visit_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for UsingDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_using_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<VarDeclarator> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Expr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::This(it) => <ThisExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Array(it) => <ArrayLit as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Object(it) => <ObjectLit as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Fn(it) => <FnExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Unary(it) => <UnaryExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Update(it) => <UpdateExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Bin(it) => <BinExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Assign(it) => <AssignExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Member(it) => <MemberExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::SuperProp(it) => <SuperPropExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Cond(it) => <CondExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Call(it) => <CallExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::New(it) => <NewExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Seq(it) => <SeqExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Ident(it) => <Ident as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Lit(it) => <Lit as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Tpl(it) => <Tpl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::TaggedTpl(it) => <TaggedTpl as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Arrow(it) => <ArrowExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Class(it) => <ClassExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Yield(it) => <YieldExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::MetaProp(it) => <MetaPropExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Await(it) => <AwaitExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Paren(it) => <ParenExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXMember(it) => <JSXMemberExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXNamespacedName(it) => {
                <JSXNamespacedName as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::JSXEmpty(it) => <JSXEmptyExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXElement(it) => <JSXElement as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXFragment(it) => <JSXFragment as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::PrivateName(it) => <PrivateName as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::OptChain(it) => <OptChainExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Invalid(it) => <Invalid as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ThisExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_this_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ArrayLit {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_array_lit(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Option<ExprOrSpread>> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ObjectLit {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_object_lit(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<PropOrSpread> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for PropOrSpread {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_prop_or_spread(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::SpreadElement(it) => {
                <SpreadElement as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::Prop(it) => <Prop as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SpreadElement {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_spread_element(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .span
        };
        <Span as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for UnaryExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_unary_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <UnaryOp as VisitWith<V>>::visit_with(UnaryOp::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for UpdateExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_update_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <UpdateOp as VisitWith<V>>::visit_with(UpdateOp::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BinExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_bin_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <BinaryOp as VisitWith<V>>::visit_with(BinaryOp::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for FnExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_fn_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Ident> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Ident::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Function as VisitWith<V>>::visit_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ClassExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Ident> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Ident::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Class as VisitWith<V>>::visit_with(
            unsafe { Class::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <AssignOp as VisitWith<V>>::visit_with(AssignOp::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <AssignTarget as VisitWith<V>>::visit_with(
            unsafe { AssignTarget::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for MemberExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_member_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <MemberProp as VisitWith<V>>::visit_with(
            unsafe { MemberProp::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for MemberProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_member_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Ident(it) => <IdentName as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::PrivateName(it) => <PrivateName as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Computed(it) => <ComputedPropName as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SuperPropExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_super_prop_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Super as VisitWith<V>>::visit_with(
            unsafe { Super::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <SuperProp as VisitWith<V>>::visit_with(
            unsafe { SuperProp::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SuperProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_super_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Ident(it) => <IdentName as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Computed(it) => <ComputedPropName as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for CondExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_cond_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for CallExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_call_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Callee as VisitWith<V>>::visit_with(
            unsafe { Callee::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ExprOrSpread> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for NewExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_new_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ExprOrSpread> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SeqExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_seq_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Expr> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ArrowExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_arrow_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Pat> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <BlockStmtOrExpr as VisitWith<V>>::visit_with(
            unsafe { BlockStmtOrExpr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for YieldExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_yield_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for MetaPropExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_meta_prop_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <MetaPropKind as VisitWith<V>>::visit_with(
            MetaPropKind::from_extra_data(ret),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AwaitExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_await_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Tpl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_tpl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Expr> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<TplElement> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TaggedTpl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_tagged_tpl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Tpl as VisitWith<V>>::visit_with(
            unsafe { Tpl::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TplElement {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_tpl_element(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_wtf8
        };
        <OptionalWtf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .utf8
        };
        <Utf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ParenExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_paren_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Callee {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_callee(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Super(it) => <Super as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Import(it) => <Import as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Super {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_super(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Import {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <ImportPhase as VisitWith<V>>::visit_with(ImportPhase::from_extra_data(ret), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExprOrSpread {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_expr_or_spread(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<SpreadDot3Token> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { SpreadDot3Token::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SpreadDot3Token {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_spread_dot_3_token(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BlockStmtOrExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_block_stmt_or_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::BlockStmt(it) => <BlockStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignTarget {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_target(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Simple(it) => <SimpleAssignTarget as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Pat(it) => <AssignTargetPat as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignTargetPat {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_target_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Array(it) => <ArrayPat as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Object(it) => <ObjectPat as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Invalid(it) => <Invalid as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SimpleAssignTarget {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_simple_assign_target(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Ident(it) => <BindingIdent as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Member(it) => <MemberExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::SuperProp(it) => <SuperPropExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Paren(it) => <ParenExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::OptChain(it) => <OptChainExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Invalid(it) => <Invalid as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for OptChainExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_chain_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <OptChainBase as VisitWith<V>>::visit_with(
            unsafe { OptChainBase::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for OptChainBase {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_chain_base(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Member(it) => <MemberExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Call(it) => <OptCall as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for OptCall {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_call(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ExprOrSpread> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Invalid {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_invalid(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Function {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_function(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Param> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 4usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Param {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_param(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Pat as VisitWith<V>>::visit_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ParamOrTsParamProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_param_or_ts_param_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Param(it) => <Param as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Class {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ClassMember> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ClassMember {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class_member(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Constructor(it) => <Constructor as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Method(it) => <ClassMethod as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::PrivateMethod(it) => {
                <PrivateMethod as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::ClassProp(it) => <ClassProp as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::PrivateProp(it) => <PrivateProp as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Empty(it) => <EmptyStmt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::StaticBlock(it) => <StaticBlock as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::AutoAccessor(it) => <AutoAccessor as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ClassProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitWith<V>>::visit_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for PrivateProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_private_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PrivateName as VisitWith<V>>::visit_with(
            unsafe { PrivateName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ClassMethod {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class_method(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitWith<V>>::visit_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Function as VisitWith<V>>::visit_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .other
        };
        <MethodKind as VisitWith<V>>::visit_with(MethodKind::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for PrivateMethod {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_private_method(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PrivateName as VisitWith<V>>::visit_with(
            unsafe { PrivateName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Function as VisitWith<V>>::visit_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .other
        };
        <MethodKind as VisitWith<V>>::visit_with(MethodKind::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Constructor {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_constructor(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitWith<V>>::visit_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ParamOrTsParamProp> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Decorator {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_decorator(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for StaticBlock {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_static_block(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <BlockStmt as VisitWith<V>>::visit_with(
            unsafe { BlockStmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Key {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_key(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Private(it) => <PrivateName as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Public(it) => <PropName as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AutoAccessor {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_auto_accessor(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Key as VisitWith<V>>::visit_with(
            unsafe { Key::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Prop {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Shorthand(it) => <Ident as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::KeyValue(it) => <KeyValueProp as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Assign(it) => <AssignProp as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Getter(it) => <GetterProp as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Setter(it) => <SetterProp as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Method(it) => <MethodProp as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for KeyValueProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_key_value_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitWith<V>>::visit_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitWith<V>>::visit_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for GetterProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_getter_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitWith<V>>::visit_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SetterProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_setter_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitWith<V>>::visit_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Pat> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Pat::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Pat as VisitWith<V>>::visit_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for MethodProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_method_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitWith<V>>::visit_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Function as VisitWith<V>>::visit_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for PropName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_prop_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Ident(it) => <IdentName as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Str(it) => <Str as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Num(it) => <Number as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Computed(it) => <ComputedPropName as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::BigInt(it) => <BigInt as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ComputedPropName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_computed_prop_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Pat {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Ident(it) => <BindingIdent as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Array(it) => <ArrayPat as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Rest(it) => <RestPat as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Object(it) => <ObjectPat as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Assign(it) => <AssignPat as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Invalid(it) => <Invalid as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ArrayPat {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_array_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Option<Pat>> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ObjectPat {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_object_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<ObjectPatProp> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignPat {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Pat as VisitWith<V>>::visit_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for RestPat {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_rest_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .span
        };
        <Span as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Pat as VisitWith<V>>::visit_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ObjectPatProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_object_pat_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::KeyValue(it) => <KeyValuePatProp as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Assign(it) => <AssignPatProp as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Rest(it) => <RestPat as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for KeyValuePatProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_key_value_pat_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitWith<V>>::visit_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Pat as VisitWith<V>>::visit_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignPatProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_pat_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <BindingIdent as VisitWith<V>>::visit_with(
            unsafe { BindingIdent::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Ident {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_ident(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for IdentName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_ident_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for PrivateName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_private_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BindingIdent {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_binding_ident(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitWith<V>>::visit_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Lit {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_lit(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Str(it) => <Str as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Bool(it) => <Bool as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Null(it) => <Null as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Num(it) => <Number as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::BigInt(it) => <BigInt as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Regex(it) => <Regex as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Str {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_str(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .wtf8
        };
        <Wtf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Bool {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_bool(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Null {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_null(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Number {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_number(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .number
        };
        <f64 as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BigInt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_big_int(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bigint
        };
        <BigIntId as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Regex {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_regex(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .utf8
        };
        <Utf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXObject {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_object(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::JSXMemberExpr(it) => {
                <JSXMemberExpr as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::Ident(it) => <Ident as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXMemberExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_member_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXObject as VisitWith<V>>::visit_with(
            unsafe { JSXObject::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <IdentName as VisitWith<V>>::visit_with(
            unsafe { IdentName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXNamespacedName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_namespaced_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <IdentName as VisitWith<V>>::visit_with(
            unsafe { IdentName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <IdentName as VisitWith<V>>::visit_with(
            unsafe { IdentName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXEmptyExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_empty_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXExprContainer {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_expr_container(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXExpr as VisitWith<V>>::visit_with(
            unsafe { JSXExpr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::JSXEmptyExpr(it) => <JSXEmptyExpr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXSpreadChild {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_spread_child(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitWith<V>>::visit_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXElementName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_element_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Ident(it) => <Ident as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXMemberExpr(it) => {
                <JSXMemberExpr as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::JSXNamespacedName(it) => {
                <JSXNamespacedName as VisitWith<V>>::visit_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXOpeningElement {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_opening_element(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXElementName as VisitWith<V>>::visit_with(
            unsafe { JSXElementName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<JSXAttrOrSpread> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXAttrOrSpread {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_attr_or_spread(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::JSXAttr(it) => <JSXAttr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::SpreadElement(it) => {
                <SpreadElement as VisitWith<V>>::visit_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXClosingElement {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_closing_element(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXElementName as VisitWith<V>>::visit_with(
            unsafe { JSXElementName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXAttr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_attr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXAttrName as VisitWith<V>>::visit_with(
            unsafe { JSXAttrName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<JSXAttrValue> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { JSXAttrValue::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXAttrName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_attr_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Ident(it) => <IdentName as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXNamespacedName(it) => {
                <JSXNamespacedName as VisitWith<V>>::visit_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXAttrValue {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_attr_value(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Str(it) => <Str as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXExprContainer(it) => {
                <JSXExprContainer as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::JSXElement(it) => <JSXElement as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXFragment(it) => <JSXFragment as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXText {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_text(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .utf8
        };
        <Utf8Ref as VisitWith<V>>::visit_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXElement {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_element(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXOpeningElement as VisitWith<V>>::visit_with(
            unsafe { JSXOpeningElement::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<JSXElementChild> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<JSXClosingElement> as VisitWith<V>>::visit_with(
            ret.map(|id| unsafe { JSXClosingElement::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXElementChild {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_element_child(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::JSXText(it) => <JSXText as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXExprContainer(it) => {
                <JSXExprContainer as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::JSXSpreadChild(it) => {
                <JSXSpreadChild as VisitWith<V>>::visit_with(it, visitor, ast)
            }
            Self::JSXElement(it) => <JSXElement as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::JSXFragment(it) => <JSXFragment as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXFragment {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_fragment(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXOpeningFragment as VisitWith<V>>::visit_with(
            unsafe { JSXOpeningFragment::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<JSXElementChild> as VisitWith<V>>::visit_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <JSXClosingFragment as VisitWith<V>>::visit_with(
            unsafe { JSXClosingFragment::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXOpeningFragment {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_opening_fragment(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for JSXClosingFragment {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_closing_fragment(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<ModuleItem> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_module_items(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<Stmt> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_stmts(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<ImportSpecifier> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import_specifiers(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<ObjectLit> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_object_lit(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<ModuleExportName> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_module_export_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<ExportSpecifier> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_specifiers(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<Str> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_str(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<Expr> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<Ident> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_ident(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<Stmt> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<SwitchCase> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_switch_cases(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<CatchClause> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_catch_clause(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<BlockStmt> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_block_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<VarDeclOrExpr> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_var_decl_or_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<Pat> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<VarDeclarator> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_var_declarators(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<ExprOrSpread> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_expr_or_spread(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<Option<ExprOrSpread>> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_expr_or_spreads(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_opt_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<PropOrSpread> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_prop_or_spreads(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<ExprOrSpread> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_expr_or_spreads(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<Expr> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_exprs(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<Pat> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_pats(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<TplElement> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_tpl_elements(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<SpreadDot3Token> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_spread_dot_3_token(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<Param> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_params(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<Decorator> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_decorators(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<ClassMember> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class_members(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<ParamOrTsParamProp> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_param_or_ts_param_props(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<Option<Pat>> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_pats(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_opt_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<ObjectPatProp> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_object_pat_props(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<JSXAttrOrSpread> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_attr_or_spreads(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<JSXAttrValue> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_jsx_attr_value(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<JSXElementChild> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_jsx_element_childs(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Option<JSXClosingElement> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_opt_jsx_closing_element(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Some(it) => it.visit_with(visitor, ast),
            None => {}
        }
    }
}
pub trait VisitMut {
    #[inline]
    fn enter_node(&mut self, node_id: NodeId, ast: &mut Ast) {}
    #[inline]
    fn leave_node(&mut self, node_id: NodeId, ast: &mut Ast) {}
    #[inline]
    fn visit_mut_program(&mut self, node: Program, ast: &mut Ast) {
        <Program as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_module(&mut self, node: Module, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Module as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_script(&mut self, node: Script, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Script as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_module_item(&mut self, node: ModuleItem, ast: &mut Ast) {
        <ModuleItem as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_module_decl(&mut self, node: ModuleDecl, ast: &mut Ast) {
        <ModuleDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_import_decl(&mut self, node: ImportDecl, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ImportDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_import_specifier(&mut self, node: ImportSpecifier, ast: &mut Ast) {
        <ImportSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_import_named_specifier(&mut self, node: ImportNamedSpecifier, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ImportNamedSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_import_default_specifier(&mut self, node: ImportDefaultSpecifier, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ImportDefaultSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_import_star_as_specifier(&mut self, node: ImportStarAsSpecifier, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ImportStarAsSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_export_decl(&mut self, node: ExportDecl, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_named_export(&mut self, node: NamedExport, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <NamedExport as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_export_specifier(&mut self, node: ExportSpecifier, ast: &mut Ast) {
        <ExportSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_namespace_specifier(
        &mut self,
        node: ExportNamespaceSpecifier,
        ast: &mut Ast,
    ) {
        self.enter_node(node.node_id(), ast);
        <ExportNamespaceSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_module_export_name(&mut self, node: ModuleExportName, ast: &mut Ast) {
        <ModuleExportName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_default_specifier(&mut self, node: ExportDefaultSpecifier, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportDefaultSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_export_named_specifier(&mut self, node: ExportNamedSpecifier, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportNamedSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_export_default_decl(&mut self, node: ExportDefaultDecl, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportDefaultDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_default_decl(&mut self, node: DefaultDecl, ast: &mut Ast) {
        <DefaultDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_default_expr(&mut self, node: ExportDefaultExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportDefaultExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_export_all(&mut self, node: ExportAll, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ExportAll as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_block_stmt(&mut self, node: BlockStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <BlockStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_stmt(&mut self, node: Stmt, ast: &mut Ast) {
        <Stmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_expr_stmt(&mut self, node: ExprStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ExprStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_empty_stmt(&mut self, node: EmptyStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <EmptyStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_debugger_stmt(&mut self, node: DebuggerStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <DebuggerStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_with_stmt(&mut self, node: WithStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <WithStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_return_stmt(&mut self, node: ReturnStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ReturnStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_labeled_stmt(&mut self, node: LabeledStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <LabeledStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_break_stmt(&mut self, node: BreakStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <BreakStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_continue_stmt(&mut self, node: ContinueStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ContinueStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_if_stmt(&mut self, node: IfStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <IfStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_switch_stmt(&mut self, node: SwitchStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <SwitchStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_throw_stmt(&mut self, node: ThrowStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ThrowStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_try_stmt(&mut self, node: TryStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <TryStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_while_stmt(&mut self, node: WhileStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <WhileStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_do_while_stmt(&mut self, node: DoWhileStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <DoWhileStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_for_stmt(&mut self, node: ForStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ForStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_for_in_stmt(&mut self, node: ForInStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ForInStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_for_of_stmt(&mut self, node: ForOfStmt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ForOfStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_switch_case(&mut self, node: SwitchCase, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <SwitchCase as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_catch_clause(&mut self, node: CatchClause, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <CatchClause as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_for_head(&mut self, node: ForHead, ast: &mut Ast) {
        <ForHead as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_var_decl_or_expr(&mut self, node: VarDeclOrExpr, ast: &mut Ast) {
        <VarDeclOrExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_decl(&mut self, node: Decl, ast: &mut Ast) {
        <Decl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_fn_decl(&mut self, node: FnDecl, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <FnDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_class_decl(&mut self, node: ClassDecl, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ClassDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_var_decl(&mut self, node: VarDecl, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <VarDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_var_declarator(&mut self, node: VarDeclarator, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <VarDeclarator as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_using_decl(&mut self, node: UsingDecl, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <UsingDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_expr(&mut self, node: Expr, ast: &mut Ast) {
        <Expr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_this_expr(&mut self, node: ThisExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ThisExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_array_lit(&mut self, node: ArrayLit, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ArrayLit as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_object_lit(&mut self, node: ObjectLit, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ObjectLit as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_prop_or_spread(&mut self, node: PropOrSpread, ast: &mut Ast) {
        <PropOrSpread as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_spread_element(&mut self, node: SpreadElement, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <SpreadElement as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_unary_expr(&mut self, node: UnaryExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <UnaryExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_update_expr(&mut self, node: UpdateExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <UpdateExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_bin_expr(&mut self, node: BinExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <BinExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_fn_expr(&mut self, node: FnExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <FnExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_class_expr(&mut self, node: ClassExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ClassExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_assign_expr(&mut self, node: AssignExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <AssignExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_member_expr(&mut self, node: MemberExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <MemberExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_member_prop(&mut self, node: MemberProp, ast: &mut Ast) {
        <MemberProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_super_prop_expr(&mut self, node: SuperPropExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <SuperPropExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_super_prop(&mut self, node: SuperProp, ast: &mut Ast) {
        <SuperProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_cond_expr(&mut self, node: CondExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <CondExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_call_expr(&mut self, node: CallExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <CallExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_new_expr(&mut self, node: NewExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <NewExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_seq_expr(&mut self, node: SeqExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <SeqExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_arrow_expr(&mut self, node: ArrowExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ArrowExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_yield_expr(&mut self, node: YieldExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <YieldExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_meta_prop_expr(&mut self, node: MetaPropExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <MetaPropExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_await_expr(&mut self, node: AwaitExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <AwaitExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_tpl(&mut self, node: Tpl, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Tpl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_tagged_tpl(&mut self, node: TaggedTpl, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <TaggedTpl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_tpl_element(&mut self, node: TplElement, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <TplElement as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_paren_expr(&mut self, node: ParenExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ParenExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_callee(&mut self, node: Callee, ast: &mut Ast) {
        <Callee as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_super(&mut self, node: Super, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Super as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_import(&mut self, node: Import, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Import as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_expr_or_spread(&mut self, node: ExprOrSpread, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ExprOrSpread as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_spread_dot_3_token(&mut self, node: SpreadDot3Token, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <SpreadDot3Token as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_block_stmt_or_expr(&mut self, node: BlockStmtOrExpr, ast: &mut Ast) {
        <BlockStmtOrExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_assign_target(&mut self, node: AssignTarget, ast: &mut Ast) {
        <AssignTarget as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_assign_target_pat(&mut self, node: AssignTargetPat, ast: &mut Ast) {
        <AssignTargetPat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_simple_assign_target(&mut self, node: SimpleAssignTarget, ast: &mut Ast) {
        <SimpleAssignTarget as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_chain_expr(&mut self, node: OptChainExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <OptChainExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_opt_chain_base(&mut self, node: OptChainBase, ast: &mut Ast) {
        <OptChainBase as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_call(&mut self, node: OptCall, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <OptCall as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_invalid(&mut self, node: Invalid, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Invalid as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_function(&mut self, node: Function, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Function as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_param(&mut self, node: Param, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Param as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_param_or_ts_param_prop(&mut self, node: ParamOrTsParamProp, ast: &mut Ast) {
        <ParamOrTsParamProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_class(&mut self, node: Class, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Class as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_class_member(&mut self, node: ClassMember, ast: &mut Ast) {
        <ClassMember as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_class_prop(&mut self, node: ClassProp, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ClassProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_private_prop(&mut self, node: PrivateProp, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <PrivateProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_class_method(&mut self, node: ClassMethod, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ClassMethod as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_private_method(&mut self, node: PrivateMethod, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <PrivateMethod as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_constructor(&mut self, node: Constructor, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Constructor as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_decorator(&mut self, node: Decorator, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Decorator as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_static_block(&mut self, node: StaticBlock, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <StaticBlock as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_key(&mut self, node: Key, ast: &mut Ast) {
        <Key as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_auto_accessor(&mut self, node: AutoAccessor, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <AutoAccessor as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_prop(&mut self, node: Prop, ast: &mut Ast) {
        <Prop as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_key_value_prop(&mut self, node: KeyValueProp, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <KeyValueProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_assign_prop(&mut self, node: AssignProp, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <AssignProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_getter_prop(&mut self, node: GetterProp, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <GetterProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_setter_prop(&mut self, node: SetterProp, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <SetterProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_method_prop(&mut self, node: MethodProp, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <MethodProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_prop_name(&mut self, node: PropName, ast: &mut Ast) {
        <PropName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_computed_prop_name(&mut self, node: ComputedPropName, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ComputedPropName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_pat(&mut self, node: Pat, ast: &mut Ast) {
        <Pat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_array_pat(&mut self, node: ArrayPat, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ArrayPat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_object_pat(&mut self, node: ObjectPat, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <ObjectPat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_assign_pat(&mut self, node: AssignPat, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <AssignPat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_rest_pat(&mut self, node: RestPat, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <RestPat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_object_pat_prop(&mut self, node: ObjectPatProp, ast: &mut Ast) {
        <ObjectPatProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_key_value_pat_prop(&mut self, node: KeyValuePatProp, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <KeyValuePatProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_assign_pat_prop(&mut self, node: AssignPatProp, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <AssignPatProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_ident(&mut self, node: Ident, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Ident as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_ident_name(&mut self, node: IdentName, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <IdentName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_private_name(&mut self, node: PrivateName, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <PrivateName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_binding_ident(&mut self, node: BindingIdent, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <BindingIdent as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_lit(&mut self, node: Lit, ast: &mut Ast) {
        <Lit as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_str(&mut self, node: Str, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Str as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_bool(&mut self, node: Bool, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Bool as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_null(&mut self, node: Null, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Null as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_number(&mut self, node: Number, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Number as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_big_int(&mut self, node: BigInt, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <BigInt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_regex(&mut self, node: Regex, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <Regex as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_object(&mut self, node: JSXObject, ast: &mut Ast) {
        <JSXObject as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_jsx_member_expr(&mut self, node: JSXMemberExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXMemberExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_namespaced_name(&mut self, node: JSXNamespacedName, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXNamespacedName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_empty_expr(&mut self, node: JSXEmptyExpr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXEmptyExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_expr_container(&mut self, node: JSXExprContainer, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXExprContainer as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_expr(&mut self, node: JSXExpr, ast: &mut Ast) {
        <JSXExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_jsx_spread_child(&mut self, node: JSXSpreadChild, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXSpreadChild as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_element_name(&mut self, node: JSXElementName, ast: &mut Ast) {
        <JSXElementName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_jsx_opening_element(&mut self, node: JSXOpeningElement, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXOpeningElement as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_attr_or_spread(&mut self, node: JSXAttrOrSpread, ast: &mut Ast) {
        <JSXAttrOrSpread as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_jsx_closing_element(&mut self, node: JSXClosingElement, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXClosingElement as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_attr(&mut self, node: JSXAttr, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXAttr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_attr_name(&mut self, node: JSXAttrName, ast: &mut Ast) {
        <JSXAttrName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_jsx_attr_value(&mut self, node: JSXAttrValue, ast: &mut Ast) {
        <JSXAttrValue as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_jsx_text(&mut self, node: JSXText, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXText as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_element(&mut self, node: JSXElement, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXElement as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_element_child(&mut self, node: JSXElementChild, ast: &mut Ast) {
        <JSXElementChild as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_jsx_fragment(&mut self, node: JSXFragment, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXFragment as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_opening_fragment(&mut self, node: JSXOpeningFragment, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXOpeningFragment as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_jsx_closing_fragment(&mut self, node: JSXClosingFragment, ast: &mut Ast) {
        self.enter_node(node.node_id(), ast);
        <JSXClosingFragment as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast);
        self.leave_node(node.node_id(), ast);
    }
    #[inline]
    fn visit_mut_module_items(&mut self, node: TypedSubRange<ModuleItem>, ast: &mut Ast) {
        <TypedSubRange<ModuleItem> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_stmts(&mut self, node: TypedSubRange<Stmt>, ast: &mut Ast) {
        <TypedSubRange<Stmt> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_import_specifiers(&mut self, node: TypedSubRange<ImportSpecifier>, ast: &mut Ast) {
        <TypedSubRange<ImportSpecifier> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_opt_object_lit(&mut self, node: Option<ObjectLit>, ast: &mut Ast) {
        <Option<ObjectLit> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_module_export_name(&mut self, node: Option<ModuleExportName>, ast: &mut Ast) {
        <Option<ModuleExportName> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_specifiers(&mut self, node: TypedSubRange<ExportSpecifier>, ast: &mut Ast) {
        <TypedSubRange<ExportSpecifier> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_opt_str(&mut self, node: Option<Str>, ast: &mut Ast) {
        <Option<Str> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_expr(&mut self, node: Option<Expr>, ast: &mut Ast) {
        <Option<Expr> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_ident(&mut self, node: Option<Ident>, ast: &mut Ast) {
        <Option<Ident> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_stmt(&mut self, node: Option<Stmt>, ast: &mut Ast) {
        <Option<Stmt> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_switch_cases(&mut self, node: TypedSubRange<SwitchCase>, ast: &mut Ast) {
        <TypedSubRange<SwitchCase> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_catch_clause(&mut self, node: Option<CatchClause>, ast: &mut Ast) {
        <Option<CatchClause> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_block_stmt(&mut self, node: Option<BlockStmt>, ast: &mut Ast) {
        <Option<BlockStmt> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_var_decl_or_expr(&mut self, node: Option<VarDeclOrExpr>, ast: &mut Ast) {
        <Option<VarDeclOrExpr> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_pat(&mut self, node: Option<Pat>, ast: &mut Ast) {
        <Option<Pat> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_var_declarators(&mut self, node: TypedSubRange<VarDeclarator>, ast: &mut Ast) {
        <TypedSubRange<VarDeclarator> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_opt_expr_or_spread(&mut self, node: Option<ExprOrSpread>, ast: &mut Ast) {
        <Option<ExprOrSpread> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_expr_or_spreads(
        &mut self,
        node: TypedSubRange<Option<ExprOrSpread>>,
        ast: &mut Ast,
    ) {
        <TypedSubRange<Option<ExprOrSpread>> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_prop_or_spreads(&mut self, node: TypedSubRange<PropOrSpread>, ast: &mut Ast) {
        <TypedSubRange<PropOrSpread> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_expr_or_spreads(&mut self, node: TypedSubRange<ExprOrSpread>, ast: &mut Ast) {
        <TypedSubRange<ExprOrSpread> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_exprs(&mut self, node: TypedSubRange<Expr>, ast: &mut Ast) {
        <TypedSubRange<Expr> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_pats(&mut self, node: TypedSubRange<Pat>, ast: &mut Ast) {
        <TypedSubRange<Pat> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_tpl_elements(&mut self, node: TypedSubRange<TplElement>, ast: &mut Ast) {
        <TypedSubRange<TplElement> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_spread_dot_3_token(&mut self, node: Option<SpreadDot3Token>, ast: &mut Ast) {
        <Option<SpreadDot3Token> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_params(&mut self, node: TypedSubRange<Param>, ast: &mut Ast) {
        <TypedSubRange<Param> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_decorators(&mut self, node: TypedSubRange<Decorator>, ast: &mut Ast) {
        <TypedSubRange<Decorator> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_class_members(&mut self, node: TypedSubRange<ClassMember>, ast: &mut Ast) {
        <TypedSubRange<ClassMember> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_param_or_ts_param_props(
        &mut self,
        node: TypedSubRange<ParamOrTsParamProp>,
        ast: &mut Ast,
    ) {
        <TypedSubRange<ParamOrTsParamProp> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_opt_pats(&mut self, node: TypedSubRange<Option<Pat>>, ast: &mut Ast) {
        <TypedSubRange<Option<Pat>> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_object_pat_props(&mut self, node: TypedSubRange<ObjectPatProp>, ast: &mut Ast) {
        <TypedSubRange<ObjectPatProp> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_jsx_attr_or_spreads(
        &mut self,
        node: TypedSubRange<JSXAttrOrSpread>,
        ast: &mut Ast,
    ) {
        <TypedSubRange<JSXAttrOrSpread> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_opt_jsx_attr_value(&mut self, node: Option<JSXAttrValue>, ast: &mut Ast) {
        <Option<JSXAttrValue> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_jsx_element_childs(
        &mut self,
        node: TypedSubRange<JSXElementChild>,
        ast: &mut Ast,
    ) {
        <TypedSubRange<JSXElementChild> as VisitMutWith<Self>>::visit_mut_children_with(
            node, self, ast,
        )
    }
    #[inline]
    fn visit_mut_opt_jsx_closing_element(
        &mut self,
        node: Option<JSXClosingElement>,
        ast: &mut Ast,
    ) {
        <Option<JSXClosingElement> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
}
pub trait VisitMutWith<V: ?Sized + VisitMut> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast);
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast);
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Program {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_program(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Module(it) => <Module as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Script(it) => <Script as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Module {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_module(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<ModuleItem> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Script {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_script(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Stmt> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ModuleItem {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_module_item(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::ModuleDecl(it) => {
                <ModuleDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Stmt(it) => <Stmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ModuleDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_module_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Import(it) => <ImportDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::ExportDecl(it) => {
                <ExportDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::ExportNamed(it) => {
                <NamedExport as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::ExportDefaultDecl(it) => {
                <ExportDefaultDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::ExportDefaultExpr(it) => {
                <ExportDefaultExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::ExportAll(it) => <ExportAll as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ImportDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<ImportSpecifier> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Str as VisitMutWith<V>>::visit_mut_with(
            unsafe { Str::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .optional_node
        };
        <Option<ObjectLit> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { ObjectLit::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 4usize).index())
                .other
        };
        <ImportPhase as VisitMutWith<V>>::visit_mut_with(
            ImportPhase::from_extra_data(ret),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ImportSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Named(it) => {
                <ImportNamedSpecifier as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Default(it) => {
                <ImportDefaultSpecifier as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Namespace(it) => {
                <ImportStarAsSpecifier as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ImportNamedSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import_named_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitMutWith<V>>::visit_mut_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<ModuleExportName> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { ModuleExportName::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ImportDefaultSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import_default_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitMutWith<V>>::visit_mut_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ImportStarAsSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import_star_as_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitMutWith<V>>::visit_mut_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Decl as VisitMutWith<V>>::visit_mut_with(
            unsafe { Decl::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for NamedExport {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_named_export(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<ExportSpecifier> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Str> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Str::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .optional_node
        };
        <Option<ObjectLit> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { ObjectLit::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Namespace(it) => {
                <ExportNamespaceSpecifier as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Default(it) => {
                <ExportDefaultSpecifier as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Named(it) => {
                <ExportNamedSpecifier as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportNamespaceSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_namespace_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <ModuleExportName as VisitMutWith<V>>::visit_mut_with(
            unsafe { ModuleExportName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ModuleExportName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_module_export_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Ident(it) => <Ident as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Str(it) => <Str as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportDefaultSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_default_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitMutWith<V>>::visit_mut_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportNamedSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_named_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <ModuleExportName as VisitMutWith<V>>::visit_mut_with(
            unsafe { ModuleExportName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<ModuleExportName> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { ModuleExportName::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportDefaultDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_default_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <DefaultDecl as VisitMutWith<V>>::visit_mut_with(
            unsafe { DefaultDecl::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for DefaultDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_default_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Class(it) => <ClassExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Fn(it) => <FnExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportDefaultExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_default_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportAll {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_all(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Str as VisitMutWith<V>>::visit_mut_with(
            unsafe { Str::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<ObjectLit> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { ObjectLit::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BlockStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_block_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Stmt> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Stmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Block(it) => <BlockStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Empty(it) => <EmptyStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Debugger(it) => {
                <DebuggerStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::With(it) => <WithStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Return(it) => <ReturnStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Labeled(it) => <LabeledStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Break(it) => <BreakStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Continue(it) => {
                <ContinueStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::If(it) => <IfStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Switch(it) => <SwitchStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Throw(it) => <ThrowStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Try(it) => <TryStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::While(it) => <WhileStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::DoWhile(it) => <DoWhileStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::For(it) => <ForStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::ForIn(it) => <ForInStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::ForOf(it) => <ForOfStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Decl(it) => <Decl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Expr(it) => <ExprStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExprStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_expr_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for EmptyStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_empty_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for DebuggerStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_debugger_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for WithStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_with_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ReturnStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_return_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for LabeledStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_labeled_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitMutWith<V>>::visit_mut_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BreakStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_break_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Ident> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Ident::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ContinueStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_continue_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Ident> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Ident::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for IfStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_if_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<Stmt> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Stmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SwitchStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_switch_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<SwitchCase> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ThrowStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_throw_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TryStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_try_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <BlockStmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { BlockStmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<CatchClause> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { CatchClause::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for WhileStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_while_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for DoWhileStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_do_while_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Stmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ForStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_for_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<VarDeclOrExpr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { VarDeclOrExpr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .node
        };
        <Stmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ForInStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_for_in_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <ForHead as VisitMutWith<V>>::visit_mut_with(
            unsafe { ForHead::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Stmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ForOfStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_for_of_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <ForHead as VisitMutWith<V>>::visit_mut_with(
            unsafe { ForHead::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .node
        };
        <Stmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { Stmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SwitchCase {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_switch_case(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<Stmt> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for CatchClause {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_catch_clause(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Pat> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Pat::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <BlockStmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { BlockStmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ForHead {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_for_head(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::VarDecl(it) => <VarDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::UsingDecl(it) => <UsingDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Pat(it) => <Pat as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for VarDeclOrExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_var_decl_or_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::VarDecl(it) => <VarDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Decl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Class(it) => <ClassDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Fn(it) => <FnDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Var(it) => <VarDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Using(it) => <UsingDecl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for FnDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_fn_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitMutWith<V>>::visit_mut_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Function as VisitMutWith<V>>::visit_mut_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ClassDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitMutWith<V>>::visit_mut_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Class as VisitMutWith<V>>::visit_mut_with(
            unsafe { Class::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for VarDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_var_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <VarDeclKind as VisitMutWith<V>>::visit_mut_with(
            VarDeclKind::from_extra_data(ret),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .sub_range
        };
        <TypedSubRange<VarDeclarator> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for VarDeclarator {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_var_declarator(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Pat as VisitMutWith<V>>::visit_mut_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for UsingDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_using_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<VarDeclarator> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Expr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::This(it) => <ThisExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Array(it) => <ArrayLit as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Object(it) => <ObjectLit as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Fn(it) => <FnExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Unary(it) => <UnaryExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Update(it) => <UpdateExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Bin(it) => <BinExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Assign(it) => <AssignExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Member(it) => <MemberExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::SuperProp(it) => {
                <SuperPropExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Cond(it) => <CondExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Call(it) => <CallExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::New(it) => <NewExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Seq(it) => <SeqExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Ident(it) => <Ident as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Lit(it) => <Lit as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Tpl(it) => <Tpl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::TaggedTpl(it) => <TaggedTpl as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Arrow(it) => <ArrowExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Class(it) => <ClassExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Yield(it) => <YieldExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::MetaProp(it) => {
                <MetaPropExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Await(it) => <AwaitExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Paren(it) => <ParenExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::JSXMember(it) => {
                <JSXMemberExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXNamespacedName(it) => {
                <JSXNamespacedName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXEmpty(it) => {
                <JSXEmptyExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXElement(it) => {
                <JSXElement as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXFragment(it) => {
                <JSXFragment as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::PrivateName(it) => {
                <PrivateName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::OptChain(it) => {
                <OptChainExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Invalid(it) => <Invalid as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ThisExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_this_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ArrayLit {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_array_lit(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Option<ExprOrSpread>> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ObjectLit {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_object_lit(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<PropOrSpread> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for PropOrSpread {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_prop_or_spread(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::SpreadElement(it) => {
                <SpreadElement as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Prop(it) => <Prop as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SpreadElement {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_spread_element(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .span
        };
        <Span as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for UnaryExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_unary_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <UnaryOp as VisitMutWith<V>>::visit_mut_with(UnaryOp::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for UpdateExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_update_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <UpdateOp as VisitMutWith<V>>::visit_mut_with(UpdateOp::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BinExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_bin_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <BinaryOp as VisitMutWith<V>>::visit_mut_with(BinaryOp::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for FnExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_fn_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Ident> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Ident::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Function as VisitMutWith<V>>::visit_mut_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ClassExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Ident> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Ident::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Class as VisitMutWith<V>>::visit_mut_with(
            unsafe { Class::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <AssignOp as VisitMutWith<V>>::visit_mut_with(AssignOp::from_extra_data(ret), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <AssignTarget as VisitMutWith<V>>::visit_mut_with(
            unsafe { AssignTarget::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for MemberExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_member_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <MemberProp as VisitMutWith<V>>::visit_mut_with(
            unsafe { MemberProp::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for MemberProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_member_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Ident(it) => <IdentName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::PrivateName(it) => {
                <PrivateName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Computed(it) => {
                <ComputedPropName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SuperPropExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_super_prop_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Super as VisitMutWith<V>>::visit_mut_with(
            unsafe { Super::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <SuperProp as VisitMutWith<V>>::visit_mut_with(
            unsafe { SuperProp::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SuperProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_super_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Ident(it) => <IdentName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Computed(it) => {
                <ComputedPropName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for CondExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_cond_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for CallExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_call_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Callee as VisitMutWith<V>>::visit_mut_with(
            unsafe { Callee::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ExprOrSpread> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for NewExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_new_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ExprOrSpread> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SeqExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_seq_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Expr> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ArrowExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_arrow_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Pat> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <BlockStmtOrExpr as VisitMutWith<V>>::visit_mut_with(
            unsafe { BlockStmtOrExpr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for YieldExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_yield_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for MetaPropExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_meta_prop_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <MetaPropKind as VisitMutWith<V>>::visit_mut_with(
            MetaPropKind::from_extra_data(ret),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AwaitExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_await_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Tpl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_tpl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Expr> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<TplElement> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TaggedTpl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_tagged_tpl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Tpl as VisitMutWith<V>>::visit_mut_with(
            unsafe { Tpl::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TplElement {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_tpl_element(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_wtf8
        };
        <OptionalWtf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .utf8
        };
        <Utf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ParenExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_paren_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Callee {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_callee(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Super(it) => <Super as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Import(it) => <Import as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Super {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_super(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Import {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .other
        };
        <ImportPhase as VisitMutWith<V>>::visit_mut_with(
            ImportPhase::from_extra_data(ret),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExprOrSpread {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_expr_or_spread(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .optional_node
        };
        <Option<SpreadDot3Token> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { SpreadDot3Token::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SpreadDot3Token {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_spread_dot_3_token(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BlockStmtOrExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_block_stmt_or_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::BlockStmt(it) => <BlockStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignTarget {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_target(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Simple(it) => {
                <SimpleAssignTarget as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Pat(it) => <AssignTargetPat as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignTargetPat {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_target_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Array(it) => <ArrayPat as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Object(it) => <ObjectPat as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Invalid(it) => <Invalid as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SimpleAssignTarget {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_simple_assign_target(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Ident(it) => <BindingIdent as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Member(it) => <MemberExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::SuperProp(it) => {
                <SuperPropExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Paren(it) => <ParenExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::OptChain(it) => {
                <OptChainExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Invalid(it) => <Invalid as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for OptChainExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_chain_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <OptChainBase as VisitMutWith<V>>::visit_mut_with(
            unsafe { OptChainBase::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for OptChainBase {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_chain_base(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Member(it) => <MemberExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Call(it) => <OptCall as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for OptCall {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_call(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ExprOrSpread> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Invalid {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_invalid(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Function {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_function(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Param> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 4usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Param {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_param(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Pat as VisitMutWith<V>>::visit_mut_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ParamOrTsParamProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_param_or_ts_param_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Param(it) => <Param as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Class {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ClassMember> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ClassMember {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class_member(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Constructor(it) => {
                <Constructor as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Method(it) => <ClassMethod as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::PrivateMethod(it) => {
                <PrivateMethod as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::ClassProp(it) => <ClassProp as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::PrivateProp(it) => {
                <PrivateProp as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Empty(it) => <EmptyStmt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::StaticBlock(it) => {
                <StaticBlock as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::AutoAccessor(it) => {
                <AutoAccessor as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ClassProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for PrivateProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_private_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PrivateName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PrivateName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ClassMethod {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class_method(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Function as VisitMutWith<V>>::visit_mut_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .other
        };
        <MethodKind as VisitMutWith<V>>::visit_mut_with(
            MethodKind::from_extra_data(ret),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for PrivateMethod {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_private_method(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PrivateName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PrivateName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Function as VisitMutWith<V>>::visit_mut_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .other
        };
        <MethodKind as VisitMutWith<V>>::visit_mut_with(
            MethodKind::from_extra_data(ret),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Constructor {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_constructor(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<ParamOrTsParamProp> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Decorator {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_decorator(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for StaticBlock {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_static_block(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <BlockStmt as VisitMutWith<V>>::visit_mut_with(
            unsafe { BlockStmt::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Key {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_key(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Private(it) => <PrivateName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Public(it) => <PropName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AutoAccessor {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_auto_accessor(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Key as VisitMutWith<V>>::visit_mut_with(
            unsafe { Key::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .sub_range
        };
        <TypedSubRange<Decorator> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Prop {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Shorthand(it) => <Ident as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::KeyValue(it) => {
                <KeyValueProp as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Assign(it) => <AssignProp as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Getter(it) => <GetterProp as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Setter(it) => <SetterProp as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Method(it) => <MethodProp as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for KeyValueProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_key_value_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitMutWith<V>>::visit_mut_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for GetterProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_getter_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SetterProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_setter_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Pat> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Pat::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <Pat as VisitMutWith<V>>::visit_mut_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 3usize).index())
                .optional_node
        };
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { BlockStmt::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for MethodProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_method_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Function as VisitMutWith<V>>::visit_mut_with(
            unsafe { Function::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for PropName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_prop_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Ident(it) => <IdentName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Str(it) => <Str as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Num(it) => <Number as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Computed(it) => {
                <ComputedPropName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::BigInt(it) => <BigInt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ComputedPropName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_computed_prop_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Pat {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Ident(it) => <BindingIdent as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Array(it) => <ArrayPat as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Rest(it) => <RestPat as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Object(it) => <ObjectPat as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Assign(it) => <AssignPat as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Invalid(it) => <Invalid as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ArrayPat {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_array_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<Option<Pat>> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ObjectPat {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_object_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .sub_range
        };
        <TypedSubRange<ObjectPatProp> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignPat {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Pat as VisitMutWith<V>>::visit_mut_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for RestPat {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_rest_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .span
        };
        <Span as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Pat as VisitMutWith<V>>::visit_mut_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ObjectPatProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_object_pat_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::KeyValue(it) => {
                <KeyValuePatProp as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Assign(it) => {
                <AssignPatProp as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Rest(it) => <RestPat as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for KeyValuePatProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_key_value_pat_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <PropName as VisitMutWith<V>>::visit_mut_with(
            unsafe { PropName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <Pat as VisitMutWith<V>>::visit_mut_with(
            unsafe { Pat::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignPatProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_pat_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <BindingIdent as VisitMutWith<V>>::visit_mut_with(
            unsafe { BindingIdent::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { Expr::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Ident {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_ident(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for IdentName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_ident_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for PrivateName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_private_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BindingIdent {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_binding_ident(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Ident as VisitMutWith<V>>::visit_mut_with(
            unsafe { Ident::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Lit {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_lit(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Str(it) => <Str as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Bool(it) => <Bool as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Null(it) => <Null as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Num(it) => <Number as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::BigInt(it) => <BigInt as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Regex(it) => <Regex as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Str {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_str(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .wtf8
        };
        <Wtf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Bool {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_bool(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Null {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_null(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Number {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_number(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .number
        };
        <f64 as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BigInt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_big_int(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .bigint
        };
        <BigIntId as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_utf8
        };
        <OptionalUtf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Regex {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_regex(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .utf8
        };
        <Utf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXObject {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_object(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::JSXMemberExpr(it) => {
                <JSXMemberExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Ident(it) => <Ident as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXMemberExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_member_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXObject as VisitMutWith<V>>::visit_mut_with(
            unsafe { JSXObject::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <IdentName as VisitMutWith<V>>::visit_mut_with(
            unsafe { IdentName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXNamespacedName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_namespaced_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <IdentName as VisitMutWith<V>>::visit_mut_with(
            unsafe { IdentName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .node
        };
        <IdentName as VisitMutWith<V>>::visit_mut_with(
            unsafe { IdentName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXEmptyExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_empty_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXExprContainer {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_expr_container(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXExpr as VisitMutWith<V>>::visit_mut_with(
            unsafe { JSXExpr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::JSXEmptyExpr(it) => {
                <JSXEmptyExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Expr(it) => <Expr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXSpreadChild {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_spread_child(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <Expr as VisitMutWith<V>>::visit_mut_with(
            unsafe { Expr::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXElementName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_element_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Ident(it) => <Ident as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::JSXMemberExpr(it) => {
                <JSXMemberExpr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXNamespacedName(it) => {
                <JSXNamespacedName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXOpeningElement {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_opening_element(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXElementName as VisitMutWith<V>>::visit_mut_with(
            unsafe { JSXElementName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<JSXAttrOrSpread> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .bool
        };
        <bool as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXAttrOrSpread {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_attr_or_spread(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::JSXAttr(it) => <JSXAttr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::SpreadElement(it) => {
                <SpreadElement as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXClosingElement {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_closing_element(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXElementName as VisitMutWith<V>>::visit_mut_with(
            unsafe { JSXElementName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXAttr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_attr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXAttrName as VisitMutWith<V>>::visit_mut_with(
            unsafe { JSXAttrName::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .optional_node
        };
        <Option<JSXAttrValue> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { JSXAttrValue::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXAttrName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_attr_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Ident(it) => <IdentName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::JSXNamespacedName(it) => {
                <JSXNamespacedName as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXAttrValue {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_attr_value(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Str(it) => <Str as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::JSXExprContainer(it) => {
                <JSXExprContainer as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXElement(it) => {
                <JSXElement as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXFragment(it) => {
                <JSXFragment as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXText {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_text(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .utf8
        };
        <Utf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .utf8
        };
        <Utf8Ref as VisitMutWith<V>>::visit_mut_with(ret.into(), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXElement {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_element(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXOpeningElement as VisitMutWith<V>>::visit_mut_with(
            unsafe { JSXOpeningElement::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<JSXElementChild> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .optional_node
        };
        <Option<JSXClosingElement> as VisitMutWith<V>>::visit_mut_with(
            ret.map(|id| unsafe { JSXClosingElement::from_node_id_unchecked(id, ast) }),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXElementChild {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_element_child(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::JSXText(it) => <JSXText as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::JSXExprContainer(it) => {
                <JSXExprContainer as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXSpreadChild(it) => {
                <JSXSpreadChild as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXElement(it) => {
                <JSXElement as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::JSXFragment(it) => {
                <JSXFragment as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXFragment {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_fragment(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 0usize).index())
                .node
        };
        <JSXOpeningFragment as VisitMutWith<V>>::visit_mut_with(
            unsafe { JSXOpeningFragment::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 1usize).index())
                .sub_range
        };
        <TypedSubRange<JSXElementChild> as VisitMutWith<V>>::visit_mut_with(
            unsafe { ret.cast_to_typed() },
            visitor,
            ast,
        );
        let ret = unsafe {
            ast.extra_data
                .as_raw_slice()
                .get_unchecked((offset + 2usize).index())
                .node
        };
        <JSXClosingFragment as VisitMutWith<V>>::visit_mut_with(
            unsafe { JSXClosingFragment::from_node_id_unchecked(ret, ast) },
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXOpeningFragment {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_opening_fragment(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for JSXClosingFragment {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_closing_fragment(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        let offset = unsafe { ast.nodes.get_unchecked(self.0).data.extra_data_start };
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<ModuleItem> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_module_items(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<Stmt> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_stmts(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<ImportSpecifier> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import_specifiers(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<ObjectLit> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_object_lit(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<ModuleExportName> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_module_export_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<ExportSpecifier> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_specifiers(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<Str> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_str(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<Expr> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<Ident> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_ident(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<Stmt> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<SwitchCase> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_switch_cases(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<CatchClause> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_catch_clause(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<BlockStmt> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_block_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<VarDeclOrExpr> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_var_decl_or_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<Pat> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<VarDeclarator> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_var_declarators(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<ExprOrSpread> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_expr_or_spread(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<Option<ExprOrSpread>> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_expr_or_spreads(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_opt_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<PropOrSpread> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_prop_or_spreads(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<ExprOrSpread> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_expr_or_spreads(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<Expr> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_exprs(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<Pat> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_pats(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<TplElement> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_tpl_elements(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<SpreadDot3Token> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_spread_dot_3_token(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<Param> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_params(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<Decorator> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_decorators(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<ClassMember> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class_members(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<ParamOrTsParamProp> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_param_or_ts_param_props(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<Option<Pat>> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_pats(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_opt_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<ObjectPatProp> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_object_pat_props(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<JSXAttrOrSpread> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_attr_or_spreads(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<JSXAttrValue> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_jsx_attr_value(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<JSXElementChild> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_jsx_element_childs(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node_in_sub_range(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Option<JSXClosingElement> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_opt_jsx_closing_element(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Some(it) => it.visit_mut_with(visitor, ast),
            None => {}
        }
    }
}
