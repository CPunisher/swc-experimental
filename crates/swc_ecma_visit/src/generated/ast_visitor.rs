#![allow(unused)]
use rspack_experimental_swc_ecma_ast::*;
use swc_common::Span;
pub trait Visit {
    #[inline]
    fn visit_program(&mut self, node: Program, ast: &Ast) {
        <Program as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_module(&mut self, node: Module, ast: &Ast) {
        <Module as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_script(&mut self, node: Script, ast: &Ast) {
        <Script as VisitWith<Self>>::visit_children_with(node, self, ast)
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
        <ImportDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_import_specifier(&mut self, node: ImportSpecifier, ast: &Ast) {
        <ImportSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_import_named_specifier(&mut self, node: ImportNamedSpecifier, ast: &Ast) {
        <ImportNamedSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_import_default_specifier(&mut self, node: ImportDefaultSpecifier, ast: &Ast) {
        <ImportDefaultSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_import_star_as_specifier(&mut self, node: ImportStarAsSpecifier, ast: &Ast) {
        <ImportStarAsSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_decl(&mut self, node: ExportDecl, ast: &Ast) {
        <ExportDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_named_export(&mut self, node: NamedExport, ast: &Ast) {
        <NamedExport as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_specifier(&mut self, node: ExportSpecifier, ast: &Ast) {
        <ExportSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_namespace_specifier(&mut self, node: ExportNamespaceSpecifier, ast: &Ast) {
        <ExportNamespaceSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_module_export_name(&mut self, node: ModuleExportName, ast: &Ast) {
        <ModuleExportName as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_default_specifier(&mut self, node: ExportDefaultSpecifier, ast: &Ast) {
        <ExportDefaultSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_named_specifier(&mut self, node: ExportNamedSpecifier, ast: &Ast) {
        <ExportNamedSpecifier as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_default_decl(&mut self, node: ExportDefaultDecl, ast: &Ast) {
        <ExportDefaultDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_default_decl(&mut self, node: DefaultDecl, ast: &Ast) {
        <DefaultDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_default_expr(&mut self, node: ExportDefaultExpr, ast: &Ast) {
        <ExportDefaultExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_export_all(&mut self, node: ExportAll, ast: &Ast) {
        <ExportAll as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_block_stmt(&mut self, node: BlockStmt, ast: &Ast) {
        <BlockStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_stmt(&mut self, node: Stmt, ast: &Ast) {
        <Stmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_expr_stmt(&mut self, node: ExprStmt, ast: &Ast) {
        <ExprStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_empty_stmt(&mut self, node: EmptyStmt, ast: &Ast) {
        <EmptyStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_debugger_stmt(&mut self, node: DebuggerStmt, ast: &Ast) {
        <DebuggerStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_with_stmt(&mut self, node: WithStmt, ast: &Ast) {
        <WithStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_return_stmt(&mut self, node: ReturnStmt, ast: &Ast) {
        <ReturnStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_labeled_stmt(&mut self, node: LabeledStmt, ast: &Ast) {
        <LabeledStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_break_stmt(&mut self, node: BreakStmt, ast: &Ast) {
        <BreakStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_continue_stmt(&mut self, node: ContinueStmt, ast: &Ast) {
        <ContinueStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_if_stmt(&mut self, node: IfStmt, ast: &Ast) {
        <IfStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_switch_stmt(&mut self, node: SwitchStmt, ast: &Ast) {
        <SwitchStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_throw_stmt(&mut self, node: ThrowStmt, ast: &Ast) {
        <ThrowStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_try_stmt(&mut self, node: TryStmt, ast: &Ast) {
        <TryStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_while_stmt(&mut self, node: WhileStmt, ast: &Ast) {
        <WhileStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_do_while_stmt(&mut self, node: DoWhileStmt, ast: &Ast) {
        <DoWhileStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_for_stmt(&mut self, node: ForStmt, ast: &Ast) {
        <ForStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_for_in_stmt(&mut self, node: ForInStmt, ast: &Ast) {
        <ForInStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_for_of_stmt(&mut self, node: ForOfStmt, ast: &Ast) {
        <ForOfStmt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_switch_case(&mut self, node: SwitchCase, ast: &Ast) {
        <SwitchCase as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_catch_clause(&mut self, node: CatchClause, ast: &Ast) {
        <CatchClause as VisitWith<Self>>::visit_children_with(node, self, ast)
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
        <FnDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_class_decl(&mut self, node: ClassDecl, ast: &Ast) {
        <ClassDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_var_decl(&mut self, node: VarDecl, ast: &Ast) {
        <VarDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_var_declarator(&mut self, node: VarDeclarator, ast: &Ast) {
        <VarDeclarator as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_using_decl(&mut self, node: UsingDecl, ast: &Ast) {
        <UsingDecl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_expr(&mut self, node: Expr, ast: &Ast) {
        <Expr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_this_expr(&mut self, node: ThisExpr, ast: &Ast) {
        <ThisExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_array_lit(&mut self, node: ArrayLit, ast: &Ast) {
        <ArrayLit as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_object_lit(&mut self, node: ObjectLit, ast: &Ast) {
        <ObjectLit as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_prop_or_spread(&mut self, node: PropOrSpread, ast: &Ast) {
        <PropOrSpread as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_spread_element(&mut self, node: SpreadElement, ast: &Ast) {
        <SpreadElement as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_unary_expr(&mut self, node: UnaryExpr, ast: &Ast) {
        <UnaryExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_update_expr(&mut self, node: UpdateExpr, ast: &Ast) {
        <UpdateExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_bin_expr(&mut self, node: BinExpr, ast: &Ast) {
        <BinExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_fn_expr(&mut self, node: FnExpr, ast: &Ast) {
        <FnExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_class_expr(&mut self, node: ClassExpr, ast: &Ast) {
        <ClassExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_assign_expr(&mut self, node: AssignExpr, ast: &Ast) {
        <AssignExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_member_expr(&mut self, node: MemberExpr, ast: &Ast) {
        <MemberExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_member_prop(&mut self, node: MemberProp, ast: &Ast) {
        <MemberProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_super_prop_expr(&mut self, node: SuperPropExpr, ast: &Ast) {
        <SuperPropExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_super_prop(&mut self, node: SuperProp, ast: &Ast) {
        <SuperProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_cond_expr(&mut self, node: CondExpr, ast: &Ast) {
        <CondExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_call_expr(&mut self, node: CallExpr, ast: &Ast) {
        <CallExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_new_expr(&mut self, node: NewExpr, ast: &Ast) {
        <NewExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_seq_expr(&mut self, node: SeqExpr, ast: &Ast) {
        <SeqExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_arrow_expr(&mut self, node: ArrowExpr, ast: &Ast) {
        <ArrowExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_yield_expr(&mut self, node: YieldExpr, ast: &Ast) {
        <YieldExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_meta_prop_expr(&mut self, node: MetaPropExpr, ast: &Ast) {
        <MetaPropExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_await_expr(&mut self, node: AwaitExpr, ast: &Ast) {
        <AwaitExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_tpl(&mut self, node: Tpl, ast: &Ast) {
        <Tpl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_tagged_tpl(&mut self, node: TaggedTpl, ast: &Ast) {
        <TaggedTpl as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_tpl_element(&mut self, node: TplElement, ast: &Ast) {
        <TplElement as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_paren_expr(&mut self, node: ParenExpr, ast: &Ast) {
        <ParenExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_callee(&mut self, node: Callee, ast: &Ast) {
        <Callee as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_super(&mut self, node: Super, ast: &Ast) {
        <Super as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_import(&mut self, node: Import, ast: &Ast) {
        <Import as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_expr_or_spread(&mut self, node: ExprOrSpread, ast: &Ast) {
        <ExprOrSpread as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_elision(&mut self, node: Elision, ast: &Ast) {
        <Elision as VisitWith<Self>>::visit_children_with(node, self, ast)
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
        <OptChainExpr as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_chain_base(&mut self, node: OptChainBase, ast: &Ast) {
        <OptChainBase as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_opt_call(&mut self, node: OptCall, ast: &Ast) {
        <OptCall as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_invalid(&mut self, node: Invalid, ast: &Ast) {
        <Invalid as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_function(&mut self, node: Function, ast: &Ast) {
        <Function as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_param(&mut self, node: Param, ast: &Ast) {
        <Param as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_param_or_ts_param_prop(&mut self, node: ParamOrTsParamProp, ast: &Ast) {
        <ParamOrTsParamProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_class(&mut self, node: Class, ast: &Ast) {
        <Class as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_class_member(&mut self, node: ClassMember, ast: &Ast) {
        <ClassMember as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_class_prop(&mut self, node: ClassProp, ast: &Ast) {
        <ClassProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_private_prop(&mut self, node: PrivateProp, ast: &Ast) {
        <PrivateProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_class_method(&mut self, node: ClassMethod, ast: &Ast) {
        <ClassMethod as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_private_method(&mut self, node: PrivateMethod, ast: &Ast) {
        <PrivateMethod as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_constructor(&mut self, node: Constructor, ast: &Ast) {
        <Constructor as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_decorator(&mut self, node: Decorator, ast: &Ast) {
        <Decorator as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_static_block(&mut self, node: StaticBlock, ast: &Ast) {
        <StaticBlock as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_key(&mut self, node: Key, ast: &Ast) {
        <Key as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_auto_accessor(&mut self, node: AutoAccessor, ast: &Ast) {
        <AutoAccessor as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_prop(&mut self, node: Prop, ast: &Ast) {
        <Prop as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_key_value_prop(&mut self, node: KeyValueProp, ast: &Ast) {
        <KeyValueProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_assign_prop(&mut self, node: AssignProp, ast: &Ast) {
        <AssignProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_getter_prop(&mut self, node: GetterProp, ast: &Ast) {
        <GetterProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_setter_prop(&mut self, node: SetterProp, ast: &Ast) {
        <SetterProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_method_prop(&mut self, node: MethodProp, ast: &Ast) {
        <MethodProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_prop_name(&mut self, node: PropName, ast: &Ast) {
        <PropName as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_computed_prop_name(&mut self, node: ComputedPropName, ast: &Ast) {
        <ComputedPropName as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_pat(&mut self, node: Pat, ast: &Ast) {
        <Pat as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_array_pat(&mut self, node: ArrayPat, ast: &Ast) {
        <ArrayPat as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_object_pat(&mut self, node: ObjectPat, ast: &Ast) {
        <ObjectPat as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_assign_pat(&mut self, node: AssignPat, ast: &Ast) {
        <AssignPat as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_rest_pat(&mut self, node: RestPat, ast: &Ast) {
        <RestPat as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_object_pat_prop(&mut self, node: ObjectPatProp, ast: &Ast) {
        <ObjectPatProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_key_value_pat_prop(&mut self, node: KeyValuePatProp, ast: &Ast) {
        <KeyValuePatProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_assign_pat_prop(&mut self, node: AssignPatProp, ast: &Ast) {
        <AssignPatProp as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_ident(&mut self, node: Ident, ast: &Ast) {
        <Ident as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_ident_name(&mut self, node: IdentName, ast: &Ast) {
        <IdentName as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_private_name(&mut self, node: PrivateName, ast: &Ast) {
        <PrivateName as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_binding_ident(&mut self, node: BindingIdent, ast: &Ast) {
        <BindingIdent as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_lit(&mut self, node: Lit, ast: &Ast) {
        <Lit as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_str(&mut self, node: Str, ast: &Ast) {
        <Str as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_bool(&mut self, node: Bool, ast: &Ast) {
        <Bool as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_null(&mut self, node: Null, ast: &Ast) {
        <Null as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_num(&mut self, node: Num, ast: &Ast) {
        <Num as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_big_int(&mut self, node: BigInt, ast: &Ast) {
        <BigInt as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_regex(&mut self, node: Regex, ast: &Ast) {
        <Regex as VisitWith<Self>>::visit_children_with(node, self, ast)
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
    fn visit_expr_or_spreads(&mut self, node: TypedSubRange<ExprOrSpread>, ast: &Ast) {
        <TypedSubRange<ExprOrSpread> as VisitWith<Self>>::visit_children_with(node, self, ast)
    }
    #[inline]
    fn visit_prop_or_spreads(&mut self, node: TypedSubRange<PropOrSpread>, ast: &Ast) {
        <TypedSubRange<PropOrSpread> as VisitWith<Self>>::visit_children_with(node, self, ast)
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
    fn visit_params(&mut self, node: TypedSubRange<Param>, ast: &Ast) {
        <TypedSubRange<Param> as VisitWith<Self>>::visit_children_with(node, self, ast)
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
        <TypedSubRange<ModuleItem> as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
        <OptionalAtomRef as VisitWith<V>>::visit_with(self.shebang(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Script {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_script(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<Stmt> as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
        <OptionalAtomRef as VisitWith<V>>::visit_with(self.shebang(ast), visitor, ast);
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
        <TypedSubRange<ImportSpecifier> as VisitWith<V>>::visit_with(
            self.specifiers(ast),
            visitor,
            ast,
        );
        <Str as VisitWith<V>>::visit_with(self.src(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.type_only(ast), visitor, ast);
        <Option<ObjectLit> as VisitWith<V>>::visit_with(self.with(ast), visitor, ast);
        <ImportPhase as VisitWith<V>>::visit_with(self.phase(ast), visitor, ast);
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
        <Ident as VisitWith<V>>::visit_with(self.local(ast), visitor, ast);
        <Option<ModuleExportName> as VisitWith<V>>::visit_with(self.imported(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_type_only(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ImportDefaultSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import_default_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Ident as VisitWith<V>>::visit_with(self.local(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ImportStarAsSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import_star_as_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Ident as VisitWith<V>>::visit_with(self.local(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Decl as VisitWith<V>>::visit_with(self.decl(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for NamedExport {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_named_export(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<ExportSpecifier> as VisitWith<V>>::visit_with(
            self.specifiers(ast),
            visitor,
            ast,
        );
        <Option<Str> as VisitWith<V>>::visit_with(self.src(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.type_only(ast), visitor, ast);
        <Option<ObjectLit> as VisitWith<V>>::visit_with(self.with(ast), visitor, ast);
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
        <ModuleExportName as VisitWith<V>>::visit_with(self.name(ast), visitor, ast);
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
        <Ident as VisitWith<V>>::visit_with(self.exported(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportNamedSpecifier {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_named_specifier(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <ModuleExportName as VisitWith<V>>::visit_with(self.orig(ast), visitor, ast);
        <Option<ModuleExportName> as VisitWith<V>>::visit_with(self.exported(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_type_only(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportDefaultDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_default_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <DefaultDecl as VisitWith<V>>::visit_with(self.decl(ast), visitor, ast);
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
        <Expr as VisitWith<V>>::visit_with(self.expr(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExportAll {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_export_all(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Str as VisitWith<V>>::visit_with(self.src(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.type_only(ast), visitor, ast);
        <Option<ObjectLit> as VisitWith<V>>::visit_with(self.with(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BlockStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_block_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<Stmt> as VisitWith<V>>::visit_with(self.stmts(ast), visitor, ast);
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
        <Expr as VisitWith<V>>::visit_with(self.expr(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for EmptyStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_empty_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {}
}
impl<V: ?Sized + Visit> VisitWith<V> for DebuggerStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_debugger_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {}
}
impl<V: ?Sized + Visit> VisitWith<V> for WithStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_with_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.obj(ast), visitor, ast);
        <Stmt as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ReturnStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_return_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Option<Expr> as VisitWith<V>>::visit_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for LabeledStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_labeled_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Ident as VisitWith<V>>::visit_with(self.label(ast), visitor, ast);
        <Stmt as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BreakStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_break_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Option<Ident> as VisitWith<V>>::visit_with(self.label(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ContinueStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_continue_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Option<Ident> as VisitWith<V>>::visit_with(self.label(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for IfStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_if_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.test(ast), visitor, ast);
        <Stmt as VisitWith<V>>::visit_with(self.cons(ast), visitor, ast);
        <Option<Stmt> as VisitWith<V>>::visit_with(self.alt(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SwitchStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_switch_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.discriminant(ast), visitor, ast);
        <TypedSubRange<SwitchCase> as VisitWith<V>>::visit_with(self.cases(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ThrowStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_throw_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TryStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_try_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <BlockStmt as VisitWith<V>>::visit_with(self.block(ast), visitor, ast);
        <Option<CatchClause> as VisitWith<V>>::visit_with(self.handler(ast), visitor, ast);
        <Option<BlockStmt> as VisitWith<V>>::visit_with(self.finalizer(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for WhileStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_while_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.test(ast), visitor, ast);
        <Stmt as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for DoWhileStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_do_while_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.test(ast), visitor, ast);
        <Stmt as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ForStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_for_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Option<VarDeclOrExpr> as VisitWith<V>>::visit_with(self.init(ast), visitor, ast);
        <Option<Expr> as VisitWith<V>>::visit_with(self.test(ast), visitor, ast);
        <Option<Expr> as VisitWith<V>>::visit_with(self.update(ast), visitor, ast);
        <Stmt as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ForInStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_for_in_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <ForHead as VisitWith<V>>::visit_with(self.left(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.right(ast), visitor, ast);
        <Stmt as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ForOfStmt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_for_of_stmt(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <bool as VisitWith<V>>::visit_with(self.is_await(ast), visitor, ast);
        <ForHead as VisitWith<V>>::visit_with(self.left(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.right(ast), visitor, ast);
        <Stmt as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SwitchCase {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_switch_case(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Option<Expr> as VisitWith<V>>::visit_with(self.test(ast), visitor, ast);
        <TypedSubRange<Stmt> as VisitWith<V>>::visit_with(self.cons(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for CatchClause {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_catch_clause(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Option<Pat> as VisitWith<V>>::visit_with(self.param(ast), visitor, ast);
        <BlockStmt as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
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
        <Ident as VisitWith<V>>::visit_with(self.ident(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.declare(ast), visitor, ast);
        <Function as VisitWith<V>>::visit_with(self.function(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ClassDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Ident as VisitWith<V>>::visit_with(self.ident(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.declare(ast), visitor, ast);
        <Class as VisitWith<V>>::visit_with(self.class(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for VarDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_var_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <VarDeclKind as VisitWith<V>>::visit_with(self.kind(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.declare(ast), visitor, ast);
        <TypedSubRange<VarDeclarator> as VisitWith<V>>::visit_with(self.decls(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for VarDeclarator {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_var_declarator(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Pat as VisitWith<V>>::visit_with(self.name(ast), visitor, ast);
        <Option<Expr> as VisitWith<V>>::visit_with(self.init(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for UsingDecl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_using_decl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <bool as VisitWith<V>>::visit_with(self.is_await(ast), visitor, ast);
        <TypedSubRange<VarDeclarator> as VisitWith<V>>::visit_with(self.decls(ast), visitor, ast);
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
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {}
}
impl<V: ?Sized + Visit> VisitWith<V> for ArrayLit {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_array_lit(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<ExprOrSpread> as VisitWith<V>>::visit_with(self.elems(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ObjectLit {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_object_lit(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<PropOrSpread> as VisitWith<V>>::visit_with(self.props(ast), visitor, ast);
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
        <Span as VisitWith<V>>::visit_with(self.dot_3_token(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.expr(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for UnaryExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_unary_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <UnaryOp as VisitWith<V>>::visit_with(self.op(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for UpdateExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_update_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <UpdateOp as VisitWith<V>>::visit_with(self.op(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.prefix(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BinExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_bin_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <BinaryOp as VisitWith<V>>::visit_with(self.op(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.left(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.right(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for FnExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_fn_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Option<Ident> as VisitWith<V>>::visit_with(self.ident(ast), visitor, ast);
        <Function as VisitWith<V>>::visit_with(self.function(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ClassExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Option<Ident> as VisitWith<V>>::visit_with(self.ident(ast), visitor, ast);
        <Class as VisitWith<V>>::visit_with(self.class(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <AssignOp as VisitWith<V>>::visit_with(self.op(ast), visitor, ast);
        <AssignTarget as VisitWith<V>>::visit_with(self.left(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.right(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for MemberExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_member_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.obj(ast), visitor, ast);
        <MemberProp as VisitWith<V>>::visit_with(self.prop(ast), visitor, ast);
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
        <Super as VisitWith<V>>::visit_with(self.obj(ast), visitor, ast);
        <SuperProp as VisitWith<V>>::visit_with(self.prop(ast), visitor, ast);
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
        <Expr as VisitWith<V>>::visit_with(self.test(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.cons(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.alt(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for CallExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_call_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Callee as VisitWith<V>>::visit_with(self.callee(ast), visitor, ast);
        <TypedSubRange<ExprOrSpread> as VisitWith<V>>::visit_with(self.args(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for NewExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_new_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.callee(ast), visitor, ast);
        <TypedSubRange<ExprOrSpread> as VisitWith<V>>::visit_with(self.args(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SeqExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_seq_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<Expr> as VisitWith<V>>::visit_with(self.exprs(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ArrowExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_arrow_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<Pat> as VisitWith<V>>::visit_with(self.params(ast), visitor, ast);
        <BlockStmtOrExpr as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_async(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_generator(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for YieldExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_yield_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Option<Expr> as VisitWith<V>>::visit_with(self.arg(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.delegate(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for MetaPropExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_meta_prop_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <MetaPropKind as VisitWith<V>>::visit_with(self.kind(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AwaitExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_await_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Tpl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_tpl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<Expr> as VisitWith<V>>::visit_with(self.exprs(ast), visitor, ast);
        <TypedSubRange<TplElement> as VisitWith<V>>::visit_with(self.quasis(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TaggedTpl {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_tagged_tpl(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.tag(ast), visitor, ast);
        <Tpl as VisitWith<V>>::visit_with(self.tpl(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TplElement {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_tpl_element(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <bool as VisitWith<V>>::visit_with(self.tail(ast), visitor, ast);
        <OptionalWtf8AtomId as VisitWith<V>>::visit_with(self.cooked(ast), visitor, ast);
        <AtomRef as VisitWith<V>>::visit_with(self.raw(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ParenExpr {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_paren_expr(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.expr(ast), visitor, ast);
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
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {}
}
impl<V: ?Sized + Visit> VisitWith<V> for Import {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_import(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <ImportPhase as VisitWith<V>>::visit_with(self.phase(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ExprOrSpread {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_expr_or_spread(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        match self {
            Self::Spread(it) => <SpreadElement as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Expr(it) => <Expr as VisitWith<V>>::visit_with(it, visitor, ast),
            Self::Elision(it) => <Elision as VisitWith<V>>::visit_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Elision {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_elision(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {}
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
        <bool as VisitWith<V>>::visit_with(self.optional(ast), visitor, ast);
        <OptChainBase as VisitWith<V>>::visit_with(self.base(ast), visitor, ast);
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
        <Expr as VisitWith<V>>::visit_with(self.callee(ast), visitor, ast);
        <TypedSubRange<ExprOrSpread> as VisitWith<V>>::visit_with(self.args(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Invalid {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_invalid(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {}
}
impl<V: ?Sized + Visit> VisitWith<V> for Function {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_function(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<Param> as VisitWith<V>>::visit_with(self.params(ast), visitor, ast);
        <Option<BlockStmt> as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_generator(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_async(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Param {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_param(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Pat as VisitWith<V>>::visit_with(self.pat(ast), visitor, ast);
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
        <TypedSubRange<ClassMember> as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
        <Option<Expr> as VisitWith<V>>::visit_with(self.super_class(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_abstract(ast), visitor, ast);
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
        <PropName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Option<Expr> as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_static(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for PrivateProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_private_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <PrivateName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Option<Expr> as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_static(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ClassMethod {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_class_method(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <PropName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Function as VisitWith<V>>::visit_with(self.function(ast), visitor, ast);
        <MethodKind as VisitWith<V>>::visit_with(self.kind(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_static(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for PrivateMethod {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_private_method(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <PrivateName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Function as VisitWith<V>>::visit_with(self.function(ast), visitor, ast);
        <MethodKind as VisitWith<V>>::visit_with(self.kind(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_static(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Constructor {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_constructor(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <PropName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <TypedSubRange<ParamOrTsParamProp> as VisitWith<V>>::visit_with(
            self.params(ast),
            visitor,
            ast,
        );
        <Option<BlockStmt> as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Decorator {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_decorator(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Expr as VisitWith<V>>::visit_with(self.expr(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for StaticBlock {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_static_block(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <BlockStmt as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
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
        <Key as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Option<Expr> as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.is_static(ast), visitor, ast);
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
        <PropName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Ident as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for GetterProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_getter_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <PropName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Option<BlockStmt> as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for SetterProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_setter_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <PropName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Pat as VisitWith<V>>::visit_with(self.param(ast), visitor, ast);
        <Option<BlockStmt> as VisitWith<V>>::visit_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for MethodProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_method_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <PropName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Function as VisitWith<V>>::visit_with(self.function(ast), visitor, ast);
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
            Self::Num(it) => <Num as VisitWith<V>>::visit_with(it, visitor, ast),
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
        <Expr as VisitWith<V>>::visit_with(self.expr(ast), visitor, ast);
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
        <TypedSubRange<Option<Pat>> as VisitWith<V>>::visit_with(self.elems(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.optional(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for ObjectPat {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_object_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <TypedSubRange<ObjectPatProp> as VisitWith<V>>::visit_with(self.props(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.optional(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignPat {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Pat as VisitWith<V>>::visit_with(self.left(ast), visitor, ast);
        <Expr as VisitWith<V>>::visit_with(self.right(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for RestPat {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_rest_pat(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Span as VisitWith<V>>::visit_with(self.dot_3_token(ast), visitor, ast);
        <Pat as VisitWith<V>>::visit_with(self.arg(ast), visitor, ast);
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
        <PropName as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Pat as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for AssignPatProp {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_assign_pat_prop(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <BindingIdent as VisitWith<V>>::visit_with(self.key(ast), visitor, ast);
        <Option<Expr> as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Ident {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_ident(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <AtomRef as VisitWith<V>>::visit_with(self.sym(ast), visitor, ast);
        <bool as VisitWith<V>>::visit_with(self.optional(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for IdentName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_ident_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <AtomRef as VisitWith<V>>::visit_with(self.sym(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for PrivateName {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_private_name(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <AtomRef as VisitWith<V>>::visit_with(self.name(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BindingIdent {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_binding_ident(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <Ident as VisitWith<V>>::visit_with(self.id(ast), visitor, ast);
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
            Self::Num(it) => <Num as VisitWith<V>>::visit_with(it, visitor, ast),
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
        <Wtf8AtomId as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
        <OptionalAtomRef as VisitWith<V>>::visit_with(self.raw(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Bool {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_bool(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <bool as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Null {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_null(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {}
}
impl<V: ?Sized + Visit> VisitWith<V> for Num {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_num(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <f64 as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
        <OptionalAtomRef as VisitWith<V>>::visit_with(self.raw(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for BigInt {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_big_int(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <BigIntId as VisitWith<V>>::visit_with(self.value(ast), visitor, ast);
        <OptionalAtomRef as VisitWith<V>>::visit_with(self.raw(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for Regex {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_regex(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        <AtomRef as VisitWith<V>>::visit_with(self.exp(ast), visitor, ast);
        <AtomRef as VisitWith<V>>::visit_with(self.flags(ast), visitor, ast);
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<ModuleItem> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_module_items(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
            child.visit_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + Visit> VisitWith<V> for TypedSubRange<Param> {
    fn visit_with(self, visitor: &mut V, ast: &Ast) {
        <V as Visit>::visit_params(visitor, self, ast)
    }
    fn visit_children_with(self, visitor: &mut V, ast: &Ast) {
        for child in self.iter() {
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_opt_node(child);
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
            let child = ast.get_node(child);
            child.visit_with(visitor, ast);
        }
    }
}
pub trait VisitMut {
    #[inline]
    fn visit_mut_program(&mut self, node: Program, ast: &mut Ast) {
        <Program as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_module(&mut self, node: Module, ast: &mut Ast) {
        <Module as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_script(&mut self, node: Script, ast: &mut Ast) {
        <Script as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
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
        <ImportDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_import_specifier(&mut self, node: ImportSpecifier, ast: &mut Ast) {
        <ImportSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_import_named_specifier(&mut self, node: ImportNamedSpecifier, ast: &mut Ast) {
        <ImportNamedSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_import_default_specifier(&mut self, node: ImportDefaultSpecifier, ast: &mut Ast) {
        <ImportDefaultSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_import_star_as_specifier(&mut self, node: ImportStarAsSpecifier, ast: &mut Ast) {
        <ImportStarAsSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_decl(&mut self, node: ExportDecl, ast: &mut Ast) {
        <ExportDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_named_export(&mut self, node: NamedExport, ast: &mut Ast) {
        <NamedExport as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
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
        <ExportNamespaceSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_module_export_name(&mut self, node: ModuleExportName, ast: &mut Ast) {
        <ModuleExportName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_default_specifier(&mut self, node: ExportDefaultSpecifier, ast: &mut Ast) {
        <ExportDefaultSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_named_specifier(&mut self, node: ExportNamedSpecifier, ast: &mut Ast) {
        <ExportNamedSpecifier as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_default_decl(&mut self, node: ExportDefaultDecl, ast: &mut Ast) {
        <ExportDefaultDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_default_decl(&mut self, node: DefaultDecl, ast: &mut Ast) {
        <DefaultDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_default_expr(&mut self, node: ExportDefaultExpr, ast: &mut Ast) {
        <ExportDefaultExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_export_all(&mut self, node: ExportAll, ast: &mut Ast) {
        <ExportAll as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_block_stmt(&mut self, node: BlockStmt, ast: &mut Ast) {
        <BlockStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_stmt(&mut self, node: Stmt, ast: &mut Ast) {
        <Stmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_expr_stmt(&mut self, node: ExprStmt, ast: &mut Ast) {
        <ExprStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_empty_stmt(&mut self, node: EmptyStmt, ast: &mut Ast) {
        <EmptyStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_debugger_stmt(&mut self, node: DebuggerStmt, ast: &mut Ast) {
        <DebuggerStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_with_stmt(&mut self, node: WithStmt, ast: &mut Ast) {
        <WithStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_return_stmt(&mut self, node: ReturnStmt, ast: &mut Ast) {
        <ReturnStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_labeled_stmt(&mut self, node: LabeledStmt, ast: &mut Ast) {
        <LabeledStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_break_stmt(&mut self, node: BreakStmt, ast: &mut Ast) {
        <BreakStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_continue_stmt(&mut self, node: ContinueStmt, ast: &mut Ast) {
        <ContinueStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_if_stmt(&mut self, node: IfStmt, ast: &mut Ast) {
        <IfStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_switch_stmt(&mut self, node: SwitchStmt, ast: &mut Ast) {
        <SwitchStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_throw_stmt(&mut self, node: ThrowStmt, ast: &mut Ast) {
        <ThrowStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_try_stmt(&mut self, node: TryStmt, ast: &mut Ast) {
        <TryStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_while_stmt(&mut self, node: WhileStmt, ast: &mut Ast) {
        <WhileStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_do_while_stmt(&mut self, node: DoWhileStmt, ast: &mut Ast) {
        <DoWhileStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_for_stmt(&mut self, node: ForStmt, ast: &mut Ast) {
        <ForStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_for_in_stmt(&mut self, node: ForInStmt, ast: &mut Ast) {
        <ForInStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_for_of_stmt(&mut self, node: ForOfStmt, ast: &mut Ast) {
        <ForOfStmt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_switch_case(&mut self, node: SwitchCase, ast: &mut Ast) {
        <SwitchCase as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_catch_clause(&mut self, node: CatchClause, ast: &mut Ast) {
        <CatchClause as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
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
        <FnDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_class_decl(&mut self, node: ClassDecl, ast: &mut Ast) {
        <ClassDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_var_decl(&mut self, node: VarDecl, ast: &mut Ast) {
        <VarDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_var_declarator(&mut self, node: VarDeclarator, ast: &mut Ast) {
        <VarDeclarator as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_using_decl(&mut self, node: UsingDecl, ast: &mut Ast) {
        <UsingDecl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_expr(&mut self, node: Expr, ast: &mut Ast) {
        <Expr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_this_expr(&mut self, node: ThisExpr, ast: &mut Ast) {
        <ThisExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_array_lit(&mut self, node: ArrayLit, ast: &mut Ast) {
        <ArrayLit as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_object_lit(&mut self, node: ObjectLit, ast: &mut Ast) {
        <ObjectLit as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_prop_or_spread(&mut self, node: PropOrSpread, ast: &mut Ast) {
        <PropOrSpread as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_spread_element(&mut self, node: SpreadElement, ast: &mut Ast) {
        <SpreadElement as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_unary_expr(&mut self, node: UnaryExpr, ast: &mut Ast) {
        <UnaryExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_update_expr(&mut self, node: UpdateExpr, ast: &mut Ast) {
        <UpdateExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_bin_expr(&mut self, node: BinExpr, ast: &mut Ast) {
        <BinExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_fn_expr(&mut self, node: FnExpr, ast: &mut Ast) {
        <FnExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_class_expr(&mut self, node: ClassExpr, ast: &mut Ast) {
        <ClassExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_assign_expr(&mut self, node: AssignExpr, ast: &mut Ast) {
        <AssignExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_member_expr(&mut self, node: MemberExpr, ast: &mut Ast) {
        <MemberExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_member_prop(&mut self, node: MemberProp, ast: &mut Ast) {
        <MemberProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_super_prop_expr(&mut self, node: SuperPropExpr, ast: &mut Ast) {
        <SuperPropExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_super_prop(&mut self, node: SuperProp, ast: &mut Ast) {
        <SuperProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_cond_expr(&mut self, node: CondExpr, ast: &mut Ast) {
        <CondExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_call_expr(&mut self, node: CallExpr, ast: &mut Ast) {
        <CallExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_new_expr(&mut self, node: NewExpr, ast: &mut Ast) {
        <NewExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_seq_expr(&mut self, node: SeqExpr, ast: &mut Ast) {
        <SeqExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_arrow_expr(&mut self, node: ArrowExpr, ast: &mut Ast) {
        <ArrowExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_yield_expr(&mut self, node: YieldExpr, ast: &mut Ast) {
        <YieldExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_meta_prop_expr(&mut self, node: MetaPropExpr, ast: &mut Ast) {
        <MetaPropExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_await_expr(&mut self, node: AwaitExpr, ast: &mut Ast) {
        <AwaitExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_tpl(&mut self, node: Tpl, ast: &mut Ast) {
        <Tpl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_tagged_tpl(&mut self, node: TaggedTpl, ast: &mut Ast) {
        <TaggedTpl as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_tpl_element(&mut self, node: TplElement, ast: &mut Ast) {
        <TplElement as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_paren_expr(&mut self, node: ParenExpr, ast: &mut Ast) {
        <ParenExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_callee(&mut self, node: Callee, ast: &mut Ast) {
        <Callee as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_super(&mut self, node: Super, ast: &mut Ast) {
        <Super as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_import(&mut self, node: Import, ast: &mut Ast) {
        <Import as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_expr_or_spread(&mut self, node: ExprOrSpread, ast: &mut Ast) {
        <ExprOrSpread as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_elision(&mut self, node: Elision, ast: &mut Ast) {
        <Elision as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
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
        <OptChainExpr as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_chain_base(&mut self, node: OptChainBase, ast: &mut Ast) {
        <OptChainBase as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_opt_call(&mut self, node: OptCall, ast: &mut Ast) {
        <OptCall as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_invalid(&mut self, node: Invalid, ast: &mut Ast) {
        <Invalid as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_function(&mut self, node: Function, ast: &mut Ast) {
        <Function as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_param(&mut self, node: Param, ast: &mut Ast) {
        <Param as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_param_or_ts_param_prop(&mut self, node: ParamOrTsParamProp, ast: &mut Ast) {
        <ParamOrTsParamProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_class(&mut self, node: Class, ast: &mut Ast) {
        <Class as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_class_member(&mut self, node: ClassMember, ast: &mut Ast) {
        <ClassMember as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_class_prop(&mut self, node: ClassProp, ast: &mut Ast) {
        <ClassProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_private_prop(&mut self, node: PrivateProp, ast: &mut Ast) {
        <PrivateProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_class_method(&mut self, node: ClassMethod, ast: &mut Ast) {
        <ClassMethod as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_private_method(&mut self, node: PrivateMethod, ast: &mut Ast) {
        <PrivateMethod as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_constructor(&mut self, node: Constructor, ast: &mut Ast) {
        <Constructor as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_decorator(&mut self, node: Decorator, ast: &mut Ast) {
        <Decorator as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_static_block(&mut self, node: StaticBlock, ast: &mut Ast) {
        <StaticBlock as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_key(&mut self, node: Key, ast: &mut Ast) {
        <Key as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_auto_accessor(&mut self, node: AutoAccessor, ast: &mut Ast) {
        <AutoAccessor as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_prop(&mut self, node: Prop, ast: &mut Ast) {
        <Prop as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_key_value_prop(&mut self, node: KeyValueProp, ast: &mut Ast) {
        <KeyValueProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_assign_prop(&mut self, node: AssignProp, ast: &mut Ast) {
        <AssignProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_getter_prop(&mut self, node: GetterProp, ast: &mut Ast) {
        <GetterProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_setter_prop(&mut self, node: SetterProp, ast: &mut Ast) {
        <SetterProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_method_prop(&mut self, node: MethodProp, ast: &mut Ast) {
        <MethodProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_prop_name(&mut self, node: PropName, ast: &mut Ast) {
        <PropName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_computed_prop_name(&mut self, node: ComputedPropName, ast: &mut Ast) {
        <ComputedPropName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_pat(&mut self, node: Pat, ast: &mut Ast) {
        <Pat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_array_pat(&mut self, node: ArrayPat, ast: &mut Ast) {
        <ArrayPat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_object_pat(&mut self, node: ObjectPat, ast: &mut Ast) {
        <ObjectPat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_assign_pat(&mut self, node: AssignPat, ast: &mut Ast) {
        <AssignPat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_rest_pat(&mut self, node: RestPat, ast: &mut Ast) {
        <RestPat as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_object_pat_prop(&mut self, node: ObjectPatProp, ast: &mut Ast) {
        <ObjectPatProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_key_value_pat_prop(&mut self, node: KeyValuePatProp, ast: &mut Ast) {
        <KeyValuePatProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_assign_pat_prop(&mut self, node: AssignPatProp, ast: &mut Ast) {
        <AssignPatProp as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_ident(&mut self, node: Ident, ast: &mut Ast) {
        <Ident as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_ident_name(&mut self, node: IdentName, ast: &mut Ast) {
        <IdentName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_private_name(&mut self, node: PrivateName, ast: &mut Ast) {
        <PrivateName as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_binding_ident(&mut self, node: BindingIdent, ast: &mut Ast) {
        <BindingIdent as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_lit(&mut self, node: Lit, ast: &mut Ast) {
        <Lit as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_str(&mut self, node: Str, ast: &mut Ast) {
        <Str as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_bool(&mut self, node: Bool, ast: &mut Ast) {
        <Bool as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_null(&mut self, node: Null, ast: &mut Ast) {
        <Null as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_num(&mut self, node: Num, ast: &mut Ast) {
        <Num as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_big_int(&mut self, node: BigInt, ast: &mut Ast) {
        <BigInt as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
    }
    #[inline]
    fn visit_mut_regex(&mut self, node: Regex, ast: &mut Ast) {
        <Regex as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
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
    fn visit_mut_expr_or_spreads(&mut self, node: TypedSubRange<ExprOrSpread>, ast: &mut Ast) {
        <TypedSubRange<ExprOrSpread> as VisitMutWith<Self>>::visit_mut_children_with(
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
    fn visit_mut_params(&mut self, node: TypedSubRange<Param>, ast: &mut Ast) {
        <TypedSubRange<Param> as VisitMutWith<Self>>::visit_mut_children_with(node, self, ast)
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
        <TypedSubRange<ModuleItem> as VisitMutWith<V>>::visit_mut_with(
            self.body(ast),
            visitor,
            ast,
        );
        <OptionalAtomRef as VisitMutWith<V>>::visit_mut_with(self.shebang(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Script {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_script(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <TypedSubRange<Stmt> as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
        <OptionalAtomRef as VisitMutWith<V>>::visit_mut_with(self.shebang(ast), visitor, ast);
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
        <TypedSubRange<ImportSpecifier> as VisitMutWith<V>>::visit_mut_with(
            self.specifiers(ast),
            visitor,
            ast,
        );
        <Str as VisitMutWith<V>>::visit_mut_with(self.src(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.type_only(ast), visitor, ast);
        <Option<ObjectLit> as VisitMutWith<V>>::visit_mut_with(self.with(ast), visitor, ast);
        <ImportPhase as VisitMutWith<V>>::visit_mut_with(self.phase(ast), visitor, ast);
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
        <Ident as VisitMutWith<V>>::visit_mut_with(self.local(ast), visitor, ast);
        <Option<ModuleExportName> as VisitMutWith<V>>::visit_mut_with(
            self.imported(ast),
            visitor,
            ast,
        );
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_type_only(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ImportDefaultSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import_default_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Ident as VisitMutWith<V>>::visit_mut_with(self.local(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ImportStarAsSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import_star_as_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Ident as VisitMutWith<V>>::visit_mut_with(self.local(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Decl as VisitMutWith<V>>::visit_mut_with(self.decl(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for NamedExport {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_named_export(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <TypedSubRange<ExportSpecifier> as VisitMutWith<V>>::visit_mut_with(
            self.specifiers(ast),
            visitor,
            ast,
        );
        <Option<Str> as VisitMutWith<V>>::visit_mut_with(self.src(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.type_only(ast), visitor, ast);
        <Option<ObjectLit> as VisitMutWith<V>>::visit_mut_with(self.with(ast), visitor, ast);
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
        <ModuleExportName as VisitMutWith<V>>::visit_mut_with(self.name(ast), visitor, ast);
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
        <Ident as VisitMutWith<V>>::visit_mut_with(self.exported(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportNamedSpecifier {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_named_specifier(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <ModuleExportName as VisitMutWith<V>>::visit_mut_with(self.orig(ast), visitor, ast);
        <Option<ModuleExportName> as VisitMutWith<V>>::visit_mut_with(
            self.exported(ast),
            visitor,
            ast,
        );
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_type_only(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportDefaultDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_default_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <DefaultDecl as VisitMutWith<V>>::visit_mut_with(self.decl(ast), visitor, ast);
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
        <Expr as VisitMutWith<V>>::visit_mut_with(self.expr(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExportAll {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_export_all(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Str as VisitMutWith<V>>::visit_mut_with(self.src(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.type_only(ast), visitor, ast);
        <Option<ObjectLit> as VisitMutWith<V>>::visit_mut_with(self.with(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BlockStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_block_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <TypedSubRange<Stmt> as VisitMutWith<V>>::visit_mut_with(self.stmts(ast), visitor, ast);
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
        <Expr as VisitMutWith<V>>::visit_mut_with(self.expr(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for EmptyStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_empty_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {}
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for DebuggerStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_debugger_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {}
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for WithStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_with_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Expr as VisitMutWith<V>>::visit_mut_with(self.obj(ast), visitor, ast);
        <Stmt as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ReturnStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_return_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for LabeledStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_labeled_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Ident as VisitMutWith<V>>::visit_mut_with(self.label(ast), visitor, ast);
        <Stmt as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BreakStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_break_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Option<Ident> as VisitMutWith<V>>::visit_mut_with(self.label(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ContinueStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_continue_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Option<Ident> as VisitMutWith<V>>::visit_mut_with(self.label(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for IfStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_if_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Expr as VisitMutWith<V>>::visit_mut_with(self.test(ast), visitor, ast);
        <Stmt as VisitMutWith<V>>::visit_mut_with(self.cons(ast), visitor, ast);
        <Option<Stmt> as VisitMutWith<V>>::visit_mut_with(self.alt(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SwitchStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_switch_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Expr as VisitMutWith<V>>::visit_mut_with(self.discriminant(ast), visitor, ast);
        <TypedSubRange<SwitchCase> as VisitMutWith<V>>::visit_mut_with(
            self.cases(ast),
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
        <Expr as VisitMutWith<V>>::visit_mut_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TryStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_try_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <BlockStmt as VisitMutWith<V>>::visit_mut_with(self.block(ast), visitor, ast);
        <Option<CatchClause> as VisitMutWith<V>>::visit_mut_with(self.handler(ast), visitor, ast);
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(self.finalizer(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for WhileStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_while_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Expr as VisitMutWith<V>>::visit_mut_with(self.test(ast), visitor, ast);
        <Stmt as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for DoWhileStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_do_while_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Expr as VisitMutWith<V>>::visit_mut_with(self.test(ast), visitor, ast);
        <Stmt as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ForStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_for_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Option<VarDeclOrExpr> as VisitMutWith<V>>::visit_mut_with(self.init(ast), visitor, ast);
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.test(ast), visitor, ast);
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.update(ast), visitor, ast);
        <Stmt as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ForInStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_for_in_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <ForHead as VisitMutWith<V>>::visit_mut_with(self.left(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.right(ast), visitor, ast);
        <Stmt as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ForOfStmt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_for_of_stmt(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_await(ast), visitor, ast);
        <ForHead as VisitMutWith<V>>::visit_mut_with(self.left(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.right(ast), visitor, ast);
        <Stmt as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SwitchCase {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_switch_case(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.test(ast), visitor, ast);
        <TypedSubRange<Stmt> as VisitMutWith<V>>::visit_mut_with(self.cons(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for CatchClause {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_catch_clause(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Option<Pat> as VisitMutWith<V>>::visit_mut_with(self.param(ast), visitor, ast);
        <BlockStmt as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
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
        <Ident as VisitMutWith<V>>::visit_mut_with(self.ident(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.declare(ast), visitor, ast);
        <Function as VisitMutWith<V>>::visit_mut_with(self.function(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ClassDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Ident as VisitMutWith<V>>::visit_mut_with(self.ident(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.declare(ast), visitor, ast);
        <Class as VisitMutWith<V>>::visit_mut_with(self.class(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for VarDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_var_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <VarDeclKind as VisitMutWith<V>>::visit_mut_with(self.kind(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.declare(ast), visitor, ast);
        <TypedSubRange<VarDeclarator> as VisitMutWith<V>>::visit_mut_with(
            self.decls(ast),
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
        <Pat as VisitMutWith<V>>::visit_mut_with(self.name(ast), visitor, ast);
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.init(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for UsingDecl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_using_decl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_await(ast), visitor, ast);
        <TypedSubRange<VarDeclarator> as VisitMutWith<V>>::visit_mut_with(
            self.decls(ast),
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
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {}
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ArrayLit {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_array_lit(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <TypedSubRange<ExprOrSpread> as VisitMutWith<V>>::visit_mut_with(
            self.elems(ast),
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
        <TypedSubRange<PropOrSpread> as VisitMutWith<V>>::visit_mut_with(
            self.props(ast),
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
        <Span as VisitMutWith<V>>::visit_mut_with(self.dot_3_token(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.expr(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for UnaryExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_unary_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <UnaryOp as VisitMutWith<V>>::visit_mut_with(self.op(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for UpdateExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_update_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <UpdateOp as VisitMutWith<V>>::visit_mut_with(self.op(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.prefix(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BinExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_bin_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <BinaryOp as VisitMutWith<V>>::visit_mut_with(self.op(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.left(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.right(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for FnExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_fn_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Option<Ident> as VisitMutWith<V>>::visit_mut_with(self.ident(ast), visitor, ast);
        <Function as VisitMutWith<V>>::visit_mut_with(self.function(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ClassExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Option<Ident> as VisitMutWith<V>>::visit_mut_with(self.ident(ast), visitor, ast);
        <Class as VisitMutWith<V>>::visit_mut_with(self.class(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <AssignOp as VisitMutWith<V>>::visit_mut_with(self.op(ast), visitor, ast);
        <AssignTarget as VisitMutWith<V>>::visit_mut_with(self.left(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.right(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for MemberExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_member_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Expr as VisitMutWith<V>>::visit_mut_with(self.obj(ast), visitor, ast);
        <MemberProp as VisitMutWith<V>>::visit_mut_with(self.prop(ast), visitor, ast);
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
        <Super as VisitMutWith<V>>::visit_mut_with(self.obj(ast), visitor, ast);
        <SuperProp as VisitMutWith<V>>::visit_mut_with(self.prop(ast), visitor, ast);
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
        <Expr as VisitMutWith<V>>::visit_mut_with(self.test(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.cons(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.alt(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for CallExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_call_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Callee as VisitMutWith<V>>::visit_mut_with(self.callee(ast), visitor, ast);
        <TypedSubRange<ExprOrSpread> as VisitMutWith<V>>::visit_mut_with(
            self.args(ast),
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
        <Expr as VisitMutWith<V>>::visit_mut_with(self.callee(ast), visitor, ast);
        <TypedSubRange<ExprOrSpread> as VisitMutWith<V>>::visit_mut_with(
            self.args(ast),
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
        <TypedSubRange<Expr> as VisitMutWith<V>>::visit_mut_with(self.exprs(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ArrowExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_arrow_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <TypedSubRange<Pat> as VisitMutWith<V>>::visit_mut_with(self.params(ast), visitor, ast);
        <BlockStmtOrExpr as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_async(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_generator(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for YieldExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_yield_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.arg(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.delegate(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for MetaPropExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_meta_prop_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <MetaPropKind as VisitMutWith<V>>::visit_mut_with(self.kind(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AwaitExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_await_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Expr as VisitMutWith<V>>::visit_mut_with(self.arg(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Tpl {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_tpl(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <TypedSubRange<Expr> as VisitMutWith<V>>::visit_mut_with(self.exprs(ast), visitor, ast);
        <TypedSubRange<TplElement> as VisitMutWith<V>>::visit_mut_with(
            self.quasis(ast),
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
        <Expr as VisitMutWith<V>>::visit_mut_with(self.tag(ast), visitor, ast);
        <Tpl as VisitMutWith<V>>::visit_mut_with(self.tpl(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TplElement {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_tpl_element(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <bool as VisitMutWith<V>>::visit_mut_with(self.tail(ast), visitor, ast);
        <OptionalWtf8AtomId as VisitMutWith<V>>::visit_mut_with(self.cooked(ast), visitor, ast);
        <AtomRef as VisitMutWith<V>>::visit_mut_with(self.raw(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ParenExpr {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_paren_expr(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Expr as VisitMutWith<V>>::visit_mut_with(self.expr(ast), visitor, ast);
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
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {}
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Import {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_import(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <ImportPhase as VisitMutWith<V>>::visit_mut_with(self.phase(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ExprOrSpread {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_expr_or_spread(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        match self {
            Self::Spread(it) => {
                <SpreadElement as VisitMutWith<V>>::visit_mut_with(it, visitor, ast)
            }
            Self::Expr(it) => <Expr as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
            Self::Elision(it) => <Elision as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Elision {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_elision(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {}
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
        <bool as VisitMutWith<V>>::visit_mut_with(self.optional(ast), visitor, ast);
        <OptChainBase as VisitMutWith<V>>::visit_mut_with(self.base(ast), visitor, ast);
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
        <Expr as VisitMutWith<V>>::visit_mut_with(self.callee(ast), visitor, ast);
        <TypedSubRange<ExprOrSpread> as VisitMutWith<V>>::visit_mut_with(
            self.args(ast),
            visitor,
            ast,
        );
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Invalid {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_invalid(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {}
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Function {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_function(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <TypedSubRange<Param> as VisitMutWith<V>>::visit_mut_with(self.params(ast), visitor, ast);
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_generator(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_async(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Param {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_param(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Pat as VisitMutWith<V>>::visit_mut_with(self.pat(ast), visitor, ast);
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
        <TypedSubRange<ClassMember> as VisitMutWith<V>>::visit_mut_with(
            self.body(ast),
            visitor,
            ast,
        );
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.super_class(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_abstract(ast), visitor, ast);
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
        <PropName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_static(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for PrivateProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_private_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <PrivateName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_static(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ClassMethod {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_class_method(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <PropName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Function as VisitMutWith<V>>::visit_mut_with(self.function(ast), visitor, ast);
        <MethodKind as VisitMutWith<V>>::visit_mut_with(self.kind(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_static(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for PrivateMethod {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_private_method(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <PrivateName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Function as VisitMutWith<V>>::visit_mut_with(self.function(ast), visitor, ast);
        <MethodKind as VisitMutWith<V>>::visit_mut_with(self.kind(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_static(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Constructor {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_constructor(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <PropName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <TypedSubRange<ParamOrTsParamProp> as VisitMutWith<V>>::visit_mut_with(
            self.params(ast),
            visitor,
            ast,
        );
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Decorator {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_decorator(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Expr as VisitMutWith<V>>::visit_mut_with(self.expr(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for StaticBlock {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_static_block(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <BlockStmt as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
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
        <Key as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.is_static(ast), visitor, ast);
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
        <PropName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Ident as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for GetterProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_getter_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <PropName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for SetterProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_setter_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <PropName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Pat as VisitMutWith<V>>::visit_mut_with(self.param(ast), visitor, ast);
        <Option<BlockStmt> as VisitMutWith<V>>::visit_mut_with(self.body(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for MethodProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_method_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <PropName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Function as VisitMutWith<V>>::visit_mut_with(self.function(ast), visitor, ast);
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
            Self::Num(it) => <Num as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
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
        <Expr as VisitMutWith<V>>::visit_mut_with(self.expr(ast), visitor, ast);
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
        <TypedSubRange<Option<Pat>> as VisitMutWith<V>>::visit_mut_with(
            self.elems(ast),
            visitor,
            ast,
        );
        <bool as VisitMutWith<V>>::visit_mut_with(self.optional(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for ObjectPat {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_object_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <TypedSubRange<ObjectPatProp> as VisitMutWith<V>>::visit_mut_with(
            self.props(ast),
            visitor,
            ast,
        );
        <bool as VisitMutWith<V>>::visit_mut_with(self.optional(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignPat {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Pat as VisitMutWith<V>>::visit_mut_with(self.left(ast), visitor, ast);
        <Expr as VisitMutWith<V>>::visit_mut_with(self.right(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for RestPat {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_rest_pat(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Span as VisitMutWith<V>>::visit_mut_with(self.dot_3_token(ast), visitor, ast);
        <Pat as VisitMutWith<V>>::visit_mut_with(self.arg(ast), visitor, ast);
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
        <PropName as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Pat as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for AssignPatProp {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_assign_pat_prop(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <BindingIdent as VisitMutWith<V>>::visit_mut_with(self.key(ast), visitor, ast);
        <Option<Expr> as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Ident {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_ident(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <AtomRef as VisitMutWith<V>>::visit_mut_with(self.sym(ast), visitor, ast);
        <bool as VisitMutWith<V>>::visit_mut_with(self.optional(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for IdentName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_ident_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <AtomRef as VisitMutWith<V>>::visit_mut_with(self.sym(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for PrivateName {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_private_name(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <AtomRef as VisitMutWith<V>>::visit_mut_with(self.name(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BindingIdent {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_binding_ident(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <Ident as VisitMutWith<V>>::visit_mut_with(self.id(ast), visitor, ast);
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
            Self::Num(it) => <Num as VisitMutWith<V>>::visit_mut_with(it, visitor, ast),
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
        <Wtf8AtomId as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
        <OptionalAtomRef as VisitMutWith<V>>::visit_mut_with(self.raw(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Bool {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_bool(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <bool as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Null {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_null(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {}
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Num {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_num(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <f64 as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
        <OptionalAtomRef as VisitMutWith<V>>::visit_mut_with(self.raw(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for BigInt {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_big_int(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <BigIntId as VisitMutWith<V>>::visit_mut_with(self.value(ast), visitor, ast);
        <OptionalAtomRef as VisitMutWith<V>>::visit_mut_with(self.raw(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for Regex {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_regex(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        <AtomRef as VisitMutWith<V>>::visit_mut_with(self.exp(ast), visitor, ast);
        <AtomRef as VisitMutWith<V>>::visit_mut_with(self.flags(ast), visitor, ast);
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<ModuleItem> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_module_items(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
impl<V: ?Sized + VisitMut> VisitMutWith<V> for TypedSubRange<Param> {
    fn visit_mut_with(self, visitor: &mut V, ast: &mut Ast) {
        <V as VisitMut>::visit_mut_params(visitor, self, ast)
    }
    fn visit_mut_children_with(self, visitor: &mut V, ast: &mut Ast) {
        for child in self.iter() {
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_node(child);
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
            let child = ast.get_opt_node(child);
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
            let child = ast.get_node(child);
            child.visit_mut_with(visitor, ast);
        }
    }
}
