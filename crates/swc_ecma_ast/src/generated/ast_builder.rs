#![allow(unused)]
use crate::{ast::*, node_id::*, Ast, AstNode, ExtraData, NodeData, NodeKind};
use swc_common::Span;
impl Ast {
    #[inline]
    pub fn program_module(
        &mut self,
        span: Span,
        body: TypedSubRange<ModuleItem>,
        shebang: OptionalAtomRef,
    ) -> Program {
        Program::Module(self.module(span, body, shebang).into())
    }
    #[inline]
    pub fn program_script(
        &mut self,
        span: Span,
        body: TypedSubRange<Stmt>,
        shebang: OptionalAtomRef,
    ) -> Program {
        Program::Script(self.script(span, body, shebang).into())
    }
    #[inline]
    pub fn module(
        &mut self,
        span: Span,
        body: TypedSubRange<ModuleItem>,
        shebang: OptionalAtomRef,
    ) -> Module {
        let _f0 = self.add_extra(ExtraData {
            sub_range: body.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: shebang.into(),
        });
        Module(self.add_node(AstNode {
            span,
            kind: NodeKind::Module,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn script(
        &mut self,
        span: Span,
        body: TypedSubRange<Stmt>,
        shebang: OptionalAtomRef,
    ) -> Script {
        let _f0 = self.add_extra(ExtraData {
            sub_range: body.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: shebang.into(),
        });
        Script(self.add_node(AstNode {
            span,
            kind: NodeKind::Script,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn module_item_module_decl_import_decl(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ImportSpecifier>,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
        phase: ImportPhase,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::Import(
            self.import_decl(span, specifiers, src, type_only, with, phase)
                .into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_decl(&mut self, span: Span, decl: Decl) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(self.export_decl(span, decl).into()))
    }
    #[inline]
    pub fn module_item_module_decl_named_export(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ExportSpecifier>,
        src: Option<Str>,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(
            self.named_export(span, specifiers, src, type_only, with)
                .into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_default_decl(
        &mut self,
        span: Span,
        decl: DefaultDecl,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(
            self.export_default_decl(span, decl).into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_default_expr(
        &mut self,
        span: Span,
        expr: Expr,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(
            self.export_default_expr(span, expr).into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_all(
        &mut self,
        span: Span,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportAll(
            self.export_all(span, src, type_only, with).into(),
        ))
    }
    #[inline]
    pub fn module_item_stmt_block_stmt(
        &mut self,
        span: Span,
        stmts: TypedSubRange<Stmt>,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Block(self.block_stmt(span, stmts).into()))
    }
    #[inline]
    pub fn module_item_stmt_empty_stmt(&mut self, span: Span) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Empty(self.empty_stmt(span).into()))
    }
    #[inline]
    pub fn module_item_stmt_debugger_stmt(&mut self, span: Span) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Debugger(self.debugger_stmt(span).into()))
    }
    #[inline]
    pub fn module_item_stmt_with_stmt(&mut self, span: Span, obj: Expr, body: Stmt) -> ModuleItem {
        ModuleItem::Stmt(Stmt::With(self.with_stmt(span, obj, body).into()))
    }
    #[inline]
    pub fn module_item_stmt_return_stmt(&mut self, span: Span, arg: Option<Expr>) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Return(self.return_stmt(span, arg).into()))
    }
    #[inline]
    pub fn module_item_stmt_labeled_stmt(
        &mut self,
        span: Span,
        label: Ident,
        body: Stmt,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Labeled(self.labeled_stmt(span, label, body).into()))
    }
    #[inline]
    pub fn module_item_stmt_break_stmt(&mut self, span: Span, label: Option<Ident>) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Break(self.break_stmt(span, label).into()))
    }
    #[inline]
    pub fn module_item_stmt_continue_stmt(
        &mut self,
        span: Span,
        label: Option<Ident>,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Continue(self.continue_stmt(span, label).into()))
    }
    #[inline]
    pub fn module_item_stmt_if_stmt(
        &mut self,
        span: Span,
        test: Expr,
        cons: Stmt,
        alt: Option<Stmt>,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::If(self.if_stmt(span, test, cons, alt).into()))
    }
    #[inline]
    pub fn module_item_stmt_switch_stmt(
        &mut self,
        span: Span,
        discriminant: Expr,
        cases: TypedSubRange<SwitchCase>,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Switch(
            self.switch_stmt(span, discriminant, cases).into(),
        ))
    }
    #[inline]
    pub fn module_item_stmt_throw_stmt(&mut self, span: Span, arg: Expr) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Throw(self.throw_stmt(span, arg).into()))
    }
    #[inline]
    pub fn module_item_stmt_try_stmt(
        &mut self,
        span: Span,
        block: BlockStmt,
        handler: Option<CatchClause>,
        finalizer: Option<BlockStmt>,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Try(
            self.try_stmt(span, block, handler, finalizer).into(),
        ))
    }
    #[inline]
    pub fn module_item_stmt_while_stmt(
        &mut self,
        span: Span,
        test: Expr,
        body: Stmt,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::While(self.while_stmt(span, test, body).into()))
    }
    #[inline]
    pub fn module_item_stmt_do_while_stmt(
        &mut self,
        span: Span,
        test: Expr,
        body: Stmt,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::DoWhile(self.do_while_stmt(span, test, body).into()))
    }
    #[inline]
    pub fn module_item_stmt_for_stmt(
        &mut self,
        span: Span,
        init: Option<VarDeclOrExpr>,
        test: Option<Expr>,
        update: Option<Expr>,
        body: Stmt,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::For(
            self.for_stmt(span, init, test, update, body).into(),
        ))
    }
    #[inline]
    pub fn module_item_stmt_for_in_stmt(
        &mut self,
        span: Span,
        left: ForHead,
        right: Expr,
        body: Stmt,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::ForIn(
            self.for_in_stmt(span, left, right, body).into(),
        ))
    }
    #[inline]
    pub fn module_item_stmt_for_of_stmt(
        &mut self,
        span: Span,
        is_await: bool,
        left: ForHead,
        right: Expr,
        body: Stmt,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::ForOf(
            self.for_of_stmt(span, is_await, left, right, body).into(),
        ))
    }
    #[inline]
    pub fn module_item_stmt_decl_class_decl(
        &mut self,
        span: Span,
        ident: Ident,
        declare: bool,
        class: Class,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Decl(Decl::Class(
            self.class_decl(span, ident, declare, class).into(),
        )))
    }
    #[inline]
    pub fn module_item_stmt_decl_fn_decl(
        &mut self,
        span: Span,
        ident: Ident,
        declare: bool,
        function: Function,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Decl(Decl::Fn(
            self.fn_decl(span, ident, declare, function).into(),
        )))
    }
    #[inline]
    pub fn module_item_stmt_decl_var_decl(
        &mut self,
        span: Span,
        kind: VarDeclKind,
        declare: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Decl(Decl::Var(
            self.var_decl(span, kind, declare, decls).into(),
        )))
    }
    #[inline]
    pub fn module_item_stmt_decl_using_decl(
        &mut self,
        span: Span,
        is_await: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Decl(Decl::Using(
            self.using_decl(span, is_await, decls).into(),
        )))
    }
    #[inline]
    pub fn module_item_stmt_expr_stmt(&mut self, span: Span, expr: Expr) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Expr(self.expr_stmt(span, expr).into()))
    }
    #[inline]
    pub fn module_decl_import_decl(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ImportSpecifier>,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
        phase: ImportPhase,
    ) -> ModuleDecl {
        ModuleDecl::Import(
            self.import_decl(span, specifiers, src, type_only, with, phase)
                .into(),
        )
    }
    #[inline]
    pub fn module_decl_export_decl(&mut self, span: Span, decl: Decl) -> ModuleDecl {
        ModuleDecl::ExportDecl(self.export_decl(span, decl).into())
    }
    #[inline]
    pub fn module_decl_named_export(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ExportSpecifier>,
        src: Option<Str>,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleDecl {
        ModuleDecl::ExportNamed(
            self.named_export(span, specifiers, src, type_only, with)
                .into(),
        )
    }
    #[inline]
    pub fn module_decl_export_default_decl(&mut self, span: Span, decl: DefaultDecl) -> ModuleDecl {
        ModuleDecl::ExportDefaultDecl(self.export_default_decl(span, decl).into())
    }
    #[inline]
    pub fn module_decl_export_default_expr(&mut self, span: Span, expr: Expr) -> ModuleDecl {
        ModuleDecl::ExportDefaultExpr(self.export_default_expr(span, expr).into())
    }
    #[inline]
    pub fn module_decl_export_all(
        &mut self,
        span: Span,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleDecl {
        ModuleDecl::ExportAll(self.export_all(span, src, type_only, with).into())
    }
    #[inline]
    pub fn import_decl(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ImportSpecifier>,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
        phase: ImportPhase,
    ) -> ImportDecl {
        let _f0 = self.add_extra(ExtraData {
            sub_range: specifiers.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: src.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: type_only.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            optional_node: with.optional_node_id(),
        });
        let _f4 = self.add_extra(ExtraData {
            other: phase.to_extra_data(),
        });
        ImportDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::ImportDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn import_specifier_import_named_specifier(
        &mut self,
        span: Span,
        local: Ident,
        imported: Option<ModuleExportName>,
        is_type_only: bool,
    ) -> ImportSpecifier {
        ImportSpecifier::Named(
            self.import_named_specifier(span, local, imported, is_type_only)
                .into(),
        )
    }
    #[inline]
    pub fn import_specifier_import_default_specifier(
        &mut self,
        span: Span,
        local: Ident,
    ) -> ImportSpecifier {
        ImportSpecifier::Default(self.import_default_specifier(span, local).into())
    }
    #[inline]
    pub fn import_specifier_import_star_as_specifier(
        &mut self,
        span: Span,
        local: Ident,
    ) -> ImportSpecifier {
        ImportSpecifier::Namespace(self.import_star_as_specifier(span, local).into())
    }
    #[inline]
    pub fn import_named_specifier(
        &mut self,
        span: Span,
        local: Ident,
        imported: Option<ModuleExportName>,
        is_type_only: bool,
    ) -> ImportNamedSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: local.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: imported.optional_node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_type_only.into(),
        });
        ImportNamedSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ImportNamedSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn import_default_specifier(&mut self, span: Span, local: Ident) -> ImportDefaultSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: local.node_id(),
        });
        ImportDefaultSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ImportDefaultSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn import_star_as_specifier(&mut self, span: Span, local: Ident) -> ImportStarAsSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: local.node_id(),
        });
        ImportStarAsSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ImportStarAsSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_decl(&mut self, span: Span, decl: Decl) -> ExportDecl {
        let _f0 = self.add_extra(ExtraData {
            node: decl.node_id(),
        });
        ExportDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn named_export(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ExportSpecifier>,
        src: Option<Str>,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> NamedExport {
        let _f0 = self.add_extra(ExtraData {
            sub_range: specifiers.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: src.optional_node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: type_only.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            optional_node: with.optional_node_id(),
        });
        NamedExport(self.add_node(AstNode {
            span,
            kind: NodeKind::NamedExport,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_specifier_export_namespace_specifier(
        &mut self,
        span: Span,
        name: ModuleExportName,
    ) -> ExportSpecifier {
        ExportSpecifier::Namespace(self.export_namespace_specifier(span, name).into())
    }
    #[inline]
    pub fn export_specifier_export_default_specifier(
        &mut self,
        span: Span,
        exported: Ident,
    ) -> ExportSpecifier {
        ExportSpecifier::Default(self.export_default_specifier(span, exported).into())
    }
    #[inline]
    pub fn export_specifier_export_named_specifier(
        &mut self,
        span: Span,
        orig: ModuleExportName,
        exported: Option<ModuleExportName>,
        is_type_only: bool,
    ) -> ExportSpecifier {
        ExportSpecifier::Named(
            self.export_named_specifier(span, orig, exported, is_type_only)
                .into(),
        )
    }
    #[inline]
    pub fn export_namespace_specifier(
        &mut self,
        span: Span,
        name: ModuleExportName,
    ) -> ExportNamespaceSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: name.node_id(),
        });
        ExportNamespaceSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportNamespaceSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn module_export_name_ident(
        &mut self,
        span: Span,
        sym: AtomRef,
        optional: bool,
    ) -> ModuleExportName {
        ModuleExportName::Ident(self.ident(span, sym, optional).into())
    }
    #[inline]
    pub fn module_export_name_str(
        &mut self,
        span: Span,
        value: Wtf8AtomRef,
        raw: OptionalAtomRef,
    ) -> ModuleExportName {
        ModuleExportName::Str(self.str(span, value, raw).into())
    }
    #[inline]
    pub fn export_default_specifier(
        &mut self,
        span: Span,
        exported: Ident,
    ) -> ExportDefaultSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: exported.node_id(),
        });
        ExportDefaultSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportDefaultSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_named_specifier(
        &mut self,
        span: Span,
        orig: ModuleExportName,
        exported: Option<ModuleExportName>,
        is_type_only: bool,
    ) -> ExportNamedSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: orig.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: exported.optional_node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_type_only.into(),
        });
        ExportNamedSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportNamedSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_default_decl(&mut self, span: Span, decl: DefaultDecl) -> ExportDefaultDecl {
        let _f0 = self.add_extra(ExtraData {
            node: decl.node_id(),
        });
        ExportDefaultDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportDefaultDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn default_decl_class_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        class: Class,
    ) -> DefaultDecl {
        DefaultDecl::Class(self.class_expr(span, ident, class).into())
    }
    #[inline]
    pub fn default_decl_fn_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        function: Function,
    ) -> DefaultDecl {
        DefaultDecl::Fn(self.fn_expr(span, ident, function).into())
    }
    #[inline]
    pub fn export_default_expr(&mut self, span: Span, expr: Expr) -> ExportDefaultExpr {
        let _f0 = self.add_extra(ExtraData {
            node: expr.node_id(),
        });
        ExportDefaultExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportDefaultExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_all(
        &mut self,
        span: Span,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ExportAll {
        let _f0 = self.add_extra(ExtraData {
            node: src.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: type_only.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            optional_node: with.optional_node_id(),
        });
        ExportAll(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportAll,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn block_stmt(&mut self, span: Span, stmts: TypedSubRange<Stmt>) -> BlockStmt {
        let _f0 = self.add_extra(ExtraData {
            sub_range: stmts.into(),
        });
        BlockStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::BlockStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn stmt_block_stmt(&mut self, span: Span, stmts: TypedSubRange<Stmt>) -> Stmt {
        Stmt::Block(self.block_stmt(span, stmts).into())
    }
    #[inline]
    pub fn stmt_empty_stmt(&mut self, span: Span) -> Stmt {
        Stmt::Empty(self.empty_stmt(span).into())
    }
    #[inline]
    pub fn stmt_debugger_stmt(&mut self, span: Span) -> Stmt {
        Stmt::Debugger(self.debugger_stmt(span).into())
    }
    #[inline]
    pub fn stmt_with_stmt(&mut self, span: Span, obj: Expr, body: Stmt) -> Stmt {
        Stmt::With(self.with_stmt(span, obj, body).into())
    }
    #[inline]
    pub fn stmt_return_stmt(&mut self, span: Span, arg: Option<Expr>) -> Stmt {
        Stmt::Return(self.return_stmt(span, arg).into())
    }
    #[inline]
    pub fn stmt_labeled_stmt(&mut self, span: Span, label: Ident, body: Stmt) -> Stmt {
        Stmt::Labeled(self.labeled_stmt(span, label, body).into())
    }
    #[inline]
    pub fn stmt_break_stmt(&mut self, span: Span, label: Option<Ident>) -> Stmt {
        Stmt::Break(self.break_stmt(span, label).into())
    }
    #[inline]
    pub fn stmt_continue_stmt(&mut self, span: Span, label: Option<Ident>) -> Stmt {
        Stmt::Continue(self.continue_stmt(span, label).into())
    }
    #[inline]
    pub fn stmt_if_stmt(&mut self, span: Span, test: Expr, cons: Stmt, alt: Option<Stmt>) -> Stmt {
        Stmt::If(self.if_stmt(span, test, cons, alt).into())
    }
    #[inline]
    pub fn stmt_switch_stmt(
        &mut self,
        span: Span,
        discriminant: Expr,
        cases: TypedSubRange<SwitchCase>,
    ) -> Stmt {
        Stmt::Switch(self.switch_stmt(span, discriminant, cases).into())
    }
    #[inline]
    pub fn stmt_throw_stmt(&mut self, span: Span, arg: Expr) -> Stmt {
        Stmt::Throw(self.throw_stmt(span, arg).into())
    }
    #[inline]
    pub fn stmt_try_stmt(
        &mut self,
        span: Span,
        block: BlockStmt,
        handler: Option<CatchClause>,
        finalizer: Option<BlockStmt>,
    ) -> Stmt {
        Stmt::Try(self.try_stmt(span, block, handler, finalizer).into())
    }
    #[inline]
    pub fn stmt_while_stmt(&mut self, span: Span, test: Expr, body: Stmt) -> Stmt {
        Stmt::While(self.while_stmt(span, test, body).into())
    }
    #[inline]
    pub fn stmt_do_while_stmt(&mut self, span: Span, test: Expr, body: Stmt) -> Stmt {
        Stmt::DoWhile(self.do_while_stmt(span, test, body).into())
    }
    #[inline]
    pub fn stmt_for_stmt(
        &mut self,
        span: Span,
        init: Option<VarDeclOrExpr>,
        test: Option<Expr>,
        update: Option<Expr>,
        body: Stmt,
    ) -> Stmt {
        Stmt::For(self.for_stmt(span, init, test, update, body).into())
    }
    #[inline]
    pub fn stmt_for_in_stmt(&mut self, span: Span, left: ForHead, right: Expr, body: Stmt) -> Stmt {
        Stmt::ForIn(self.for_in_stmt(span, left, right, body).into())
    }
    #[inline]
    pub fn stmt_for_of_stmt(
        &mut self,
        span: Span,
        is_await: bool,
        left: ForHead,
        right: Expr,
        body: Stmt,
    ) -> Stmt {
        Stmt::ForOf(self.for_of_stmt(span, is_await, left, right, body).into())
    }
    #[inline]
    pub fn stmt_decl_class_decl(
        &mut self,
        span: Span,
        ident: Ident,
        declare: bool,
        class: Class,
    ) -> Stmt {
        Stmt::Decl(Decl::Class(
            self.class_decl(span, ident, declare, class).into(),
        ))
    }
    #[inline]
    pub fn stmt_decl_fn_decl(
        &mut self,
        span: Span,
        ident: Ident,
        declare: bool,
        function: Function,
    ) -> Stmt {
        Stmt::Decl(Decl::Fn(
            self.fn_decl(span, ident, declare, function).into(),
        ))
    }
    #[inline]
    pub fn stmt_decl_var_decl(
        &mut self,
        span: Span,
        kind: VarDeclKind,
        declare: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> Stmt {
        Stmt::Decl(Decl::Var(self.var_decl(span, kind, declare, decls).into()))
    }
    #[inline]
    pub fn stmt_decl_using_decl(
        &mut self,
        span: Span,
        is_await: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> Stmt {
        Stmt::Decl(Decl::Using(self.using_decl(span, is_await, decls).into()))
    }
    #[inline]
    pub fn stmt_expr_stmt(&mut self, span: Span, expr: Expr) -> Stmt {
        Stmt::Expr(self.expr_stmt(span, expr).into())
    }
    #[inline]
    pub fn expr_stmt(&mut self, span: Span, expr: Expr) -> ExprStmt {
        let _f0 = self.add_extra(ExtraData {
            node: expr.node_id(),
        });
        ExprStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::ExprStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn empty_stmt(&mut self, span: Span) -> EmptyStmt {
        EmptyStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::EmptyStmt,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn debugger_stmt(&mut self, span: Span) -> DebuggerStmt {
        DebuggerStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::DebuggerStmt,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn with_stmt(&mut self, span: Span, obj: Expr, body: Stmt) -> WithStmt {
        let _f0 = self.add_extra(ExtraData {
            node: obj.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        WithStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::WithStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn return_stmt(&mut self, span: Span, arg: Option<Expr>) -> ReturnStmt {
        let _f0 = self.add_extra(ExtraData {
            optional_node: arg.optional_node_id(),
        });
        ReturnStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::ReturnStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn labeled_stmt(&mut self, span: Span, label: Ident, body: Stmt) -> LabeledStmt {
        let _f0 = self.add_extra(ExtraData {
            node: label.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        LabeledStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::LabeledStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn break_stmt(&mut self, span: Span, label: Option<Ident>) -> BreakStmt {
        let _f0 = self.add_extra(ExtraData {
            optional_node: label.optional_node_id(),
        });
        BreakStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::BreakStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn continue_stmt(&mut self, span: Span, label: Option<Ident>) -> ContinueStmt {
        let _f0 = self.add_extra(ExtraData {
            optional_node: label.optional_node_id(),
        });
        ContinueStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::ContinueStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn if_stmt(&mut self, span: Span, test: Expr, cons: Stmt, alt: Option<Stmt>) -> IfStmt {
        let _f0 = self.add_extra(ExtraData {
            node: test.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: cons.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            optional_node: alt.optional_node_id(),
        });
        IfStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::IfStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn switch_stmt(
        &mut self,
        span: Span,
        discriminant: Expr,
        cases: TypedSubRange<SwitchCase>,
    ) -> SwitchStmt {
        let _f0 = self.add_extra(ExtraData {
            node: discriminant.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: cases.into(),
        });
        SwitchStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::SwitchStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn throw_stmt(&mut self, span: Span, arg: Expr) -> ThrowStmt {
        let _f0 = self.add_extra(ExtraData {
            node: arg.node_id(),
        });
        ThrowStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::ThrowStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn try_stmt(
        &mut self,
        span: Span,
        block: BlockStmt,
        handler: Option<CatchClause>,
        finalizer: Option<BlockStmt>,
    ) -> TryStmt {
        let _f0 = self.add_extra(ExtraData {
            node: block.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: handler.optional_node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            optional_node: finalizer.optional_node_id(),
        });
        TryStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::TryStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn while_stmt(&mut self, span: Span, test: Expr, body: Stmt) -> WhileStmt {
        let _f0 = self.add_extra(ExtraData {
            node: test.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        WhileStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::WhileStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn do_while_stmt(&mut self, span: Span, test: Expr, body: Stmt) -> DoWhileStmt {
        let _f0 = self.add_extra(ExtraData {
            node: test.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        DoWhileStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::DoWhileStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn for_stmt(
        &mut self,
        span: Span,
        init: Option<VarDeclOrExpr>,
        test: Option<Expr>,
        update: Option<Expr>,
        body: Stmt,
    ) -> ForStmt {
        let _f0 = self.add_extra(ExtraData {
            optional_node: init.optional_node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: test.optional_node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            optional_node: update.optional_node_id(),
        });
        let _f3 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        ForStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::ForStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn for_in_stmt(&mut self, span: Span, left: ForHead, right: Expr, body: Stmt) -> ForInStmt {
        let _f0 = self.add_extra(ExtraData {
            node: left.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: right.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        ForInStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::ForInStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn for_of_stmt(
        &mut self,
        span: Span,
        is_await: bool,
        left: ForHead,
        right: Expr,
        body: Stmt,
    ) -> ForOfStmt {
        let _f0 = self.add_extra(ExtraData {
            bool: is_await.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: left.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            node: right.node_id(),
        });
        let _f3 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        ForOfStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::ForOfStmt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn switch_case(
        &mut self,
        span: Span,
        test: Option<Expr>,
        cons: TypedSubRange<Stmt>,
    ) -> SwitchCase {
        let _f0 = self.add_extra(ExtraData {
            optional_node: test.optional_node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: cons.into(),
        });
        SwitchCase(self.add_node(AstNode {
            span,
            kind: NodeKind::SwitchCase,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn catch_clause(&mut self, span: Span, param: Option<Pat>, body: BlockStmt) -> CatchClause {
        let _f0 = self.add_extra(ExtraData {
            optional_node: param.optional_node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        CatchClause(self.add_node(AstNode {
            span,
            kind: NodeKind::CatchClause,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn for_head_var_decl(
        &mut self,
        span: Span,
        kind: VarDeclKind,
        declare: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> ForHead {
        ForHead::VarDecl(self.var_decl(span, kind, declare, decls).into())
    }
    #[inline]
    pub fn for_head_using_decl(
        &mut self,
        span: Span,
        is_await: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> ForHead {
        ForHead::UsingDecl(self.using_decl(span, is_await, decls).into())
    }
    #[inline]
    pub fn for_head_pat_binding_ident(&mut self, span: Span, id: Ident) -> ForHead {
        ForHead::Pat(Pat::Ident(self.binding_ident(span, id).into()))
    }
    #[inline]
    pub fn for_head_pat_array_pat(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<Pat>>,
        optional: bool,
    ) -> ForHead {
        ForHead::Pat(Pat::Array(self.array_pat(span, elems, optional).into()))
    }
    #[inline]
    pub fn for_head_pat_rest_pat(&mut self, span: Span, dot3_token: Span, arg: Pat) -> ForHead {
        ForHead::Pat(Pat::Rest(self.rest_pat(span, dot3_token, arg).into()))
    }
    #[inline]
    pub fn for_head_pat_object_pat(
        &mut self,
        span: Span,
        props: TypedSubRange<ObjectPatProp>,
        optional: bool,
    ) -> ForHead {
        ForHead::Pat(Pat::Object(self.object_pat(span, props, optional).into()))
    }
    #[inline]
    pub fn for_head_pat_assign_pat(&mut self, span: Span, left: Pat, right: Expr) -> ForHead {
        ForHead::Pat(Pat::Assign(self.assign_pat(span, left, right).into()))
    }
    #[inline]
    pub fn for_head_pat_invalid(&mut self, span: Span) -> ForHead {
        ForHead::Pat(Pat::Invalid(self.invalid(span).into()))
    }
    #[inline]
    pub fn for_head_pat_expr_this_expr(&mut self, span: Span) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::This(self.this_expr(span).into())))
    }
    #[inline]
    pub fn for_head_pat_expr_array_lit(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<ExprOrSpread>>,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Array(self.array_lit(span, elems).into())))
    }
    #[inline]
    pub fn for_head_pat_expr_object_lit(
        &mut self,
        span: Span,
        props: TypedSubRange<PropOrSpread>,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Object(self.object_lit(span, props).into())))
    }
    #[inline]
    pub fn for_head_pat_expr_fn_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        function: Function,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Fn(
            self.fn_expr(span, ident, function).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_unary_expr(&mut self, span: Span, op: UnaryOp, arg: Expr) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Unary(
            self.unary_expr(span, op, arg).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_update_expr(
        &mut self,
        span: Span,
        op: UpdateOp,
        prefix: bool,
        arg: Expr,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Update(
            self.update_expr(span, op, prefix, arg).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_bin_expr(
        &mut self,
        span: Span,
        op: BinaryOp,
        left: Expr,
        right: Expr,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Bin(
            self.bin_expr(span, op, left, right).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_assign_expr(
        &mut self,
        span: Span,
        op: AssignOp,
        left: AssignTarget,
        right: Expr,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Assign(
            self.assign_expr(span, op, left, right).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_member_expr(
        &mut self,
        span: Span,
        obj: Expr,
        prop: MemberProp,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Member(
            self.member_expr(span, obj, prop).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_super_prop_expr(
        &mut self,
        span: Span,
        obj: Super,
        prop: SuperProp,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::SuperProp(
            self.super_prop_expr(span, obj, prop).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_cond_expr(
        &mut self,
        span: Span,
        test: Expr,
        cons: Expr,
        alt: Expr,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Cond(
            self.cond_expr(span, test, cons, alt).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_call_expr(
        &mut self,
        span: Span,
        callee: Callee,
        args: TypedSubRange<ExprOrSpread>,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Call(
            self.call_expr(span, callee, args).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_new_expr(
        &mut self,
        span: Span,
        callee: Expr,
        args: TypedSubRange<ExprOrSpread>,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::New(
            self.new_expr(span, callee, args).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_seq_expr(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Seq(self.seq_expr(span, exprs).into())))
    }
    #[inline]
    pub fn for_head_pat_expr_ident(&mut self, span: Span, sym: AtomRef, optional: bool) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Ident(
            self.ident(span, sym, optional).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_lit_str(
        &mut self,
        span: Span,
        value: Wtf8AtomRef,
        raw: OptionalAtomRef,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Lit(Lit::Str(
            self.str(span, value, raw).into(),
        ))))
    }
    #[inline]
    pub fn for_head_pat_expr_lit_bool(&mut self, span: Span, value: bool) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Lit(Lit::Bool(
            self.bool(span, value).into(),
        ))))
    }
    #[inline]
    pub fn for_head_pat_expr_lit_null(&mut self, span: Span) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Lit(Lit::Null(self.null(span).into()))))
    }
    #[inline]
    pub fn for_head_pat_expr_lit_number(
        &mut self,
        span: Span,
        value: f64,
        raw: OptionalAtomRef,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Lit(Lit::Num(
            self.number(span, value, raw).into(),
        ))))
    }
    #[inline]
    pub fn for_head_pat_expr_lit_big_int(
        &mut self,
        span: Span,
        value: BigIntId,
        raw: OptionalAtomRef,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Lit(Lit::BigInt(
            self.big_int(span, value, raw).into(),
        ))))
    }
    #[inline]
    pub fn for_head_pat_expr_lit_regex(
        &mut self,
        span: Span,
        exp: AtomRef,
        flags: AtomRef,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Lit(Lit::Regex(
            self.regex(span, exp, flags).into(),
        ))))
    }
    #[inline]
    pub fn for_head_pat_expr_tpl(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
        quasis: TypedSubRange<TplElement>,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Tpl(self.tpl(span, exprs, quasis).into())))
    }
    #[inline]
    pub fn for_head_pat_expr_tagged_tpl(&mut self, span: Span, tag: Expr, tpl: Tpl) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::TaggedTpl(
            self.tagged_tpl(span, tag, tpl).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_arrow_expr(
        &mut self,
        span: Span,
        params: TypedSubRange<Pat>,
        body: BlockStmtOrExpr,
        is_async: bool,
        is_generator: bool,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Arrow(
            self.arrow_expr(span, params, body, is_async, is_generator)
                .into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_class_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        class: Class,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Class(
            self.class_expr(span, ident, class).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_yield_expr(
        &mut self,
        span: Span,
        arg: Option<Expr>,
        delegate: bool,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Yield(
            self.yield_expr(span, arg, delegate).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_meta_prop_expr(&mut self, span: Span, kind: MetaPropKind) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::MetaProp(
            self.meta_prop_expr(span, kind).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_await_expr(&mut self, span: Span, arg: Expr) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Await(self.await_expr(span, arg).into())))
    }
    #[inline]
    pub fn for_head_pat_expr_paren_expr(&mut self, span: Span, expr: Expr) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Paren(self.paren_expr(span, expr).into())))
    }
    #[inline]
    pub fn for_head_pat_expr_private_name(&mut self, span: Span, name: AtomRef) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::PrivateName(
            self.private_name(span, name).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_opt_chain_expr(
        &mut self,
        span: Span,
        optional: bool,
        base: OptChainBase,
    ) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::OptChain(
            self.opt_chain_expr(span, optional, base).into(),
        )))
    }
    #[inline]
    pub fn for_head_pat_expr_invalid(&mut self, span: Span) -> ForHead {
        ForHead::Pat(Pat::Expr(Expr::Invalid(self.invalid(span).into())))
    }
    #[inline]
    pub fn var_decl_or_expr_var_decl(
        &mut self,
        span: Span,
        kind: VarDeclKind,
        declare: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::VarDecl(self.var_decl(span, kind, declare, decls).into())
    }
    #[inline]
    pub fn var_decl_or_expr_expr_this_expr(&mut self, span: Span) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::This(self.this_expr(span).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_array_lit(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<ExprOrSpread>>,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Array(self.array_lit(span, elems).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_object_lit(
        &mut self,
        span: Span,
        props: TypedSubRange<PropOrSpread>,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Object(self.object_lit(span, props).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_fn_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        function: Function,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Fn(self.fn_expr(span, ident, function).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_unary_expr(
        &mut self,
        span: Span,
        op: UnaryOp,
        arg: Expr,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Unary(self.unary_expr(span, op, arg).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_update_expr(
        &mut self,
        span: Span,
        op: UpdateOp,
        prefix: bool,
        arg: Expr,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Update(self.update_expr(span, op, prefix, arg).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_bin_expr(
        &mut self,
        span: Span,
        op: BinaryOp,
        left: Expr,
        right: Expr,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Bin(self.bin_expr(span, op, left, right).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_assign_expr(
        &mut self,
        span: Span,
        op: AssignOp,
        left: AssignTarget,
        right: Expr,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Assign(self.assign_expr(span, op, left, right).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_member_expr(
        &mut self,
        span: Span,
        obj: Expr,
        prop: MemberProp,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Member(self.member_expr(span, obj, prop).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_super_prop_expr(
        &mut self,
        span: Span,
        obj: Super,
        prop: SuperProp,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::SuperProp(
            self.super_prop_expr(span, obj, prop).into(),
        ))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_cond_expr(
        &mut self,
        span: Span,
        test: Expr,
        cons: Expr,
        alt: Expr,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Cond(self.cond_expr(span, test, cons, alt).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_call_expr(
        &mut self,
        span: Span,
        callee: Callee,
        args: TypedSubRange<ExprOrSpread>,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Call(self.call_expr(span, callee, args).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_new_expr(
        &mut self,
        span: Span,
        callee: Expr,
        args: TypedSubRange<ExprOrSpread>,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::New(self.new_expr(span, callee, args).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_seq_expr(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Seq(self.seq_expr(span, exprs).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_ident(
        &mut self,
        span: Span,
        sym: AtomRef,
        optional: bool,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Ident(self.ident(span, sym, optional).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_lit_str(
        &mut self,
        span: Span,
        value: Wtf8AtomRef,
        raw: OptionalAtomRef,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Lit(Lit::Str(self.str(span, value, raw).into())))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_lit_bool(&mut self, span: Span, value: bool) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Lit(Lit::Bool(self.bool(span, value).into())))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_lit_null(&mut self, span: Span) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Lit(Lit::Null(self.null(span).into())))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_lit_number(
        &mut self,
        span: Span,
        value: f64,
        raw: OptionalAtomRef,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Lit(Lit::Num(self.number(span, value, raw).into())))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_lit_big_int(
        &mut self,
        span: Span,
        value: BigIntId,
        raw: OptionalAtomRef,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Lit(Lit::BigInt(
            self.big_int(span, value, raw).into(),
        )))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_lit_regex(
        &mut self,
        span: Span,
        exp: AtomRef,
        flags: AtomRef,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Lit(Lit::Regex(self.regex(span, exp, flags).into())))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_tpl(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
        quasis: TypedSubRange<TplElement>,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Tpl(self.tpl(span, exprs, quasis).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_tagged_tpl(
        &mut self,
        span: Span,
        tag: Expr,
        tpl: Tpl,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::TaggedTpl(self.tagged_tpl(span, tag, tpl).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_arrow_expr(
        &mut self,
        span: Span,
        params: TypedSubRange<Pat>,
        body: BlockStmtOrExpr,
        is_async: bool,
        is_generator: bool,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Arrow(
            self.arrow_expr(span, params, body, is_async, is_generator)
                .into(),
        ))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_class_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        class: Class,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Class(self.class_expr(span, ident, class).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_yield_expr(
        &mut self,
        span: Span,
        arg: Option<Expr>,
        delegate: bool,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Yield(self.yield_expr(span, arg, delegate).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_meta_prop_expr(
        &mut self,
        span: Span,
        kind: MetaPropKind,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::MetaProp(self.meta_prop_expr(span, kind).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_await_expr(&mut self, span: Span, arg: Expr) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Await(self.await_expr(span, arg).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_paren_expr(&mut self, span: Span, expr: Expr) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Paren(self.paren_expr(span, expr).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_private_name(
        &mut self,
        span: Span,
        name: AtomRef,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::PrivateName(self.private_name(span, name).into()))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_opt_chain_expr(
        &mut self,
        span: Span,
        optional: bool,
        base: OptChainBase,
    ) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::OptChain(
            self.opt_chain_expr(span, optional, base).into(),
        ))
    }
    #[inline]
    pub fn var_decl_or_expr_expr_invalid(&mut self, span: Span) -> VarDeclOrExpr {
        VarDeclOrExpr::Expr(Expr::Invalid(self.invalid(span).into()))
    }
    #[inline]
    pub fn decl_class_decl(
        &mut self,
        span: Span,
        ident: Ident,
        declare: bool,
        class: Class,
    ) -> Decl {
        Decl::Class(self.class_decl(span, ident, declare, class).into())
    }
    #[inline]
    pub fn decl_fn_decl(
        &mut self,
        span: Span,
        ident: Ident,
        declare: bool,
        function: Function,
    ) -> Decl {
        Decl::Fn(self.fn_decl(span, ident, declare, function).into())
    }
    #[inline]
    pub fn decl_var_decl(
        &mut self,
        span: Span,
        kind: VarDeclKind,
        declare: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> Decl {
        Decl::Var(self.var_decl(span, kind, declare, decls).into())
    }
    #[inline]
    pub fn decl_using_decl(
        &mut self,
        span: Span,
        is_await: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> Decl {
        Decl::Using(self.using_decl(span, is_await, decls).into())
    }
    #[inline]
    pub fn fn_decl(
        &mut self,
        span: Span,
        ident: Ident,
        declare: bool,
        function: Function,
    ) -> FnDecl {
        let _f0 = self.add_extra(ExtraData {
            node: ident.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: declare.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            node: function.node_id(),
        });
        FnDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::FnDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn class_decl(
        &mut self,
        span: Span,
        ident: Ident,
        declare: bool,
        class: Class,
    ) -> ClassDecl {
        let _f0 = self.add_extra(ExtraData {
            node: ident.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: declare.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            node: class.node_id(),
        });
        ClassDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::ClassDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn var_decl(
        &mut self,
        span: Span,
        kind: VarDeclKind,
        declare: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> VarDecl {
        let _f0 = self.add_extra(ExtraData {
            other: kind.to_extra_data(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: declare.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            sub_range: decls.into(),
        });
        VarDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::VarDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn var_declarator(&mut self, span: Span, name: Pat, init: Option<Expr>) -> VarDeclarator {
        let _f0 = self.add_extra(ExtraData {
            node: name.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: init.optional_node_id(),
        });
        VarDeclarator(self.add_node(AstNode {
            span,
            kind: NodeKind::VarDeclarator,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn using_decl(
        &mut self,
        span: Span,
        is_await: bool,
        decls: TypedSubRange<VarDeclarator>,
    ) -> UsingDecl {
        let _f0 = self.add_extra(ExtraData {
            bool: is_await.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: decls.into(),
        });
        UsingDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::UsingDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn expr_this_expr(&mut self, span: Span) -> Expr {
        Expr::This(self.this_expr(span).into())
    }
    #[inline]
    pub fn expr_array_lit(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<ExprOrSpread>>,
    ) -> Expr {
        Expr::Array(self.array_lit(span, elems).into())
    }
    #[inline]
    pub fn expr_object_lit(&mut self, span: Span, props: TypedSubRange<PropOrSpread>) -> Expr {
        Expr::Object(self.object_lit(span, props).into())
    }
    #[inline]
    pub fn expr_fn_expr(&mut self, span: Span, ident: Option<Ident>, function: Function) -> Expr {
        Expr::Fn(self.fn_expr(span, ident, function).into())
    }
    #[inline]
    pub fn expr_unary_expr(&mut self, span: Span, op: UnaryOp, arg: Expr) -> Expr {
        Expr::Unary(self.unary_expr(span, op, arg).into())
    }
    #[inline]
    pub fn expr_update_expr(&mut self, span: Span, op: UpdateOp, prefix: bool, arg: Expr) -> Expr {
        Expr::Update(self.update_expr(span, op, prefix, arg).into())
    }
    #[inline]
    pub fn expr_bin_expr(&mut self, span: Span, op: BinaryOp, left: Expr, right: Expr) -> Expr {
        Expr::Bin(self.bin_expr(span, op, left, right).into())
    }
    #[inline]
    pub fn expr_assign_expr(
        &mut self,
        span: Span,
        op: AssignOp,
        left: AssignTarget,
        right: Expr,
    ) -> Expr {
        Expr::Assign(self.assign_expr(span, op, left, right).into())
    }
    #[inline]
    pub fn expr_member_expr(&mut self, span: Span, obj: Expr, prop: MemberProp) -> Expr {
        Expr::Member(self.member_expr(span, obj, prop).into())
    }
    #[inline]
    pub fn expr_super_prop_expr(&mut self, span: Span, obj: Super, prop: SuperProp) -> Expr {
        Expr::SuperProp(self.super_prop_expr(span, obj, prop).into())
    }
    #[inline]
    pub fn expr_cond_expr(&mut self, span: Span, test: Expr, cons: Expr, alt: Expr) -> Expr {
        Expr::Cond(self.cond_expr(span, test, cons, alt).into())
    }
    #[inline]
    pub fn expr_call_expr(
        &mut self,
        span: Span,
        callee: Callee,
        args: TypedSubRange<ExprOrSpread>,
    ) -> Expr {
        Expr::Call(self.call_expr(span, callee, args).into())
    }
    #[inline]
    pub fn expr_new_expr(
        &mut self,
        span: Span,
        callee: Expr,
        args: TypedSubRange<ExprOrSpread>,
    ) -> Expr {
        Expr::New(self.new_expr(span, callee, args).into())
    }
    #[inline]
    pub fn expr_seq_expr(&mut self, span: Span, exprs: TypedSubRange<Expr>) -> Expr {
        Expr::Seq(self.seq_expr(span, exprs).into())
    }
    #[inline]
    pub fn expr_ident(&mut self, span: Span, sym: AtomRef, optional: bool) -> Expr {
        Expr::Ident(self.ident(span, sym, optional).into())
    }
    #[inline]
    pub fn expr_lit_str(&mut self, span: Span, value: Wtf8AtomRef, raw: OptionalAtomRef) -> Expr {
        Expr::Lit(Lit::Str(self.str(span, value, raw).into()))
    }
    #[inline]
    pub fn expr_lit_bool(&mut self, span: Span, value: bool) -> Expr {
        Expr::Lit(Lit::Bool(self.bool(span, value).into()))
    }
    #[inline]
    pub fn expr_lit_null(&mut self, span: Span) -> Expr {
        Expr::Lit(Lit::Null(self.null(span).into()))
    }
    #[inline]
    pub fn expr_lit_number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Expr {
        Expr::Lit(Lit::Num(self.number(span, value, raw).into()))
    }
    #[inline]
    pub fn expr_lit_big_int(&mut self, span: Span, value: BigIntId, raw: OptionalAtomRef) -> Expr {
        Expr::Lit(Lit::BigInt(self.big_int(span, value, raw).into()))
    }
    #[inline]
    pub fn expr_lit_regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> Expr {
        Expr::Lit(Lit::Regex(self.regex(span, exp, flags).into()))
    }
    #[inline]
    pub fn expr_tpl(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
        quasis: TypedSubRange<TplElement>,
    ) -> Expr {
        Expr::Tpl(self.tpl(span, exprs, quasis).into())
    }
    #[inline]
    pub fn expr_tagged_tpl(&mut self, span: Span, tag: Expr, tpl: Tpl) -> Expr {
        Expr::TaggedTpl(self.tagged_tpl(span, tag, tpl).into())
    }
    #[inline]
    pub fn expr_arrow_expr(
        &mut self,
        span: Span,
        params: TypedSubRange<Pat>,
        body: BlockStmtOrExpr,
        is_async: bool,
        is_generator: bool,
    ) -> Expr {
        Expr::Arrow(
            self.arrow_expr(span, params, body, is_async, is_generator)
                .into(),
        )
    }
    #[inline]
    pub fn expr_class_expr(&mut self, span: Span, ident: Option<Ident>, class: Class) -> Expr {
        Expr::Class(self.class_expr(span, ident, class).into())
    }
    #[inline]
    pub fn expr_yield_expr(&mut self, span: Span, arg: Option<Expr>, delegate: bool) -> Expr {
        Expr::Yield(self.yield_expr(span, arg, delegate).into())
    }
    #[inline]
    pub fn expr_meta_prop_expr(&mut self, span: Span, kind: MetaPropKind) -> Expr {
        Expr::MetaProp(self.meta_prop_expr(span, kind).into())
    }
    #[inline]
    pub fn expr_await_expr(&mut self, span: Span, arg: Expr) -> Expr {
        Expr::Await(self.await_expr(span, arg).into())
    }
    #[inline]
    pub fn expr_paren_expr(&mut self, span: Span, expr: Expr) -> Expr {
        Expr::Paren(self.paren_expr(span, expr).into())
    }
    #[inline]
    pub fn expr_private_name(&mut self, span: Span, name: AtomRef) -> Expr {
        Expr::PrivateName(self.private_name(span, name).into())
    }
    #[inline]
    pub fn expr_opt_chain_expr(&mut self, span: Span, optional: bool, base: OptChainBase) -> Expr {
        Expr::OptChain(self.opt_chain_expr(span, optional, base).into())
    }
    #[inline]
    pub fn expr_invalid(&mut self, span: Span) -> Expr {
        Expr::Invalid(self.invalid(span).into())
    }
    #[inline]
    pub fn this_expr(&mut self, span: Span) -> ThisExpr {
        ThisExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::ThisExpr,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn array_lit(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<ExprOrSpread>>,
    ) -> ArrayLit {
        let _f0 = self.add_extra(ExtraData {
            sub_range: elems.into(),
        });
        ArrayLit(self.add_node(AstNode {
            span,
            kind: NodeKind::ArrayLit,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn object_lit(&mut self, span: Span, props: TypedSubRange<PropOrSpread>) -> ObjectLit {
        let _f0 = self.add_extra(ExtraData {
            sub_range: props.into(),
        });
        ObjectLit(self.add_node(AstNode {
            span,
            kind: NodeKind::ObjectLit,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn prop_or_spread_spread_element(
        &mut self,
        span: Span,
        dot3_token: Span,
        expr: Expr,
    ) -> PropOrSpread {
        PropOrSpread::SpreadElement(self.spread_element(span, dot3_token, expr).into())
    }
    #[inline]
    pub fn prop_or_spread_prop_ident(
        &mut self,
        span: Span,
        sym: AtomRef,
        optional: bool,
    ) -> PropOrSpread {
        PropOrSpread::Prop(Prop::Shorthand(self.ident(span, sym, optional).into()))
    }
    #[inline]
    pub fn prop_or_spread_prop_key_value_prop(
        &mut self,
        span: Span,
        key: PropName,
        value: Expr,
    ) -> PropOrSpread {
        PropOrSpread::Prop(Prop::KeyValue(self.key_value_prop(span, key, value).into()))
    }
    #[inline]
    pub fn prop_or_spread_prop_assign_prop(
        &mut self,
        span: Span,
        key: Ident,
        value: Expr,
    ) -> PropOrSpread {
        PropOrSpread::Prop(Prop::Assign(self.assign_prop(span, key, value).into()))
    }
    #[inline]
    pub fn prop_or_spread_prop_getter_prop(
        &mut self,
        span: Span,
        key: PropName,
        body: Option<BlockStmt>,
    ) -> PropOrSpread {
        PropOrSpread::Prop(Prop::Getter(self.getter_prop(span, key, body).into()))
    }
    #[inline]
    pub fn prop_or_spread_prop_setter_prop(
        &mut self,
        span: Span,
        key: PropName,
        this_param: Option<Pat>,
        param: Pat,
        body: Option<BlockStmt>,
    ) -> PropOrSpread {
        PropOrSpread::Prop(Prop::Setter(
            self.setter_prop(span, key, this_param, param, body).into(),
        ))
    }
    #[inline]
    pub fn prop_or_spread_prop_method_prop(
        &mut self,
        span: Span,
        key: PropName,
        function: Function,
    ) -> PropOrSpread {
        PropOrSpread::Prop(Prop::Method(self.method_prop(span, key, function).into()))
    }
    #[inline]
    pub fn spread_element(&mut self, span: Span, dot3_token: Span, expr: Expr) -> SpreadElement {
        let _f0 = self.add_extra(ExtraData {
            span: dot3_token.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: expr.node_id(),
        });
        SpreadElement(self.add_node(AstNode {
            span,
            kind: NodeKind::SpreadElement,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn unary_expr(&mut self, span: Span, op: UnaryOp, arg: Expr) -> UnaryExpr {
        let _f0 = self.add_extra(ExtraData {
            other: op.to_extra_data(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: arg.node_id(),
        });
        UnaryExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::UnaryExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn update_expr(&mut self, span: Span, op: UpdateOp, prefix: bool, arg: Expr) -> UpdateExpr {
        let _f0 = self.add_extra(ExtraData {
            other: op.to_extra_data(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: prefix.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            node: arg.node_id(),
        });
        UpdateExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::UpdateExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn bin_expr(&mut self, span: Span, op: BinaryOp, left: Expr, right: Expr) -> BinExpr {
        let _f0 = self.add_extra(ExtraData {
            other: op.to_extra_data(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: left.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            node: right.node_id(),
        });
        BinExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::BinExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn fn_expr(&mut self, span: Span, ident: Option<Ident>, function: Function) -> FnExpr {
        let _f0 = self.add_extra(ExtraData {
            optional_node: ident.optional_node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: function.node_id(),
        });
        FnExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::FnExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn class_expr(&mut self, span: Span, ident: Option<Ident>, class: Class) -> ClassExpr {
        let _f0 = self.add_extra(ExtraData {
            optional_node: ident.optional_node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: class.node_id(),
        });
        ClassExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::ClassExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn assign_expr(
        &mut self,
        span: Span,
        op: AssignOp,
        left: AssignTarget,
        right: Expr,
    ) -> AssignExpr {
        let _f0 = self.add_extra(ExtraData {
            other: op.to_extra_data(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: left.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            node: right.node_id(),
        });
        AssignExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::AssignExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn member_expr(&mut self, span: Span, obj: Expr, prop: MemberProp) -> MemberExpr {
        let _f0 = self.add_extra(ExtraData {
            node: obj.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: prop.node_id(),
        });
        MemberExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::MemberExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn member_prop_ident_name(&mut self, span: Span, sym: AtomRef) -> MemberProp {
        MemberProp::Ident(self.ident_name(span, sym).into())
    }
    #[inline]
    pub fn member_prop_private_name(&mut self, span: Span, name: AtomRef) -> MemberProp {
        MemberProp::PrivateName(self.private_name(span, name).into())
    }
    #[inline]
    pub fn member_prop_computed_prop_name(&mut self, span: Span, expr: Expr) -> MemberProp {
        MemberProp::Computed(self.computed_prop_name(span, expr).into())
    }
    #[inline]
    pub fn super_prop_expr(&mut self, span: Span, obj: Super, prop: SuperProp) -> SuperPropExpr {
        let _f0 = self.add_extra(ExtraData {
            node: obj.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: prop.node_id(),
        });
        SuperPropExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::SuperPropExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn super_prop_ident_name(&mut self, span: Span, sym: AtomRef) -> SuperProp {
        SuperProp::Ident(self.ident_name(span, sym).into())
    }
    #[inline]
    pub fn super_prop_computed_prop_name(&mut self, span: Span, expr: Expr) -> SuperProp {
        SuperProp::Computed(self.computed_prop_name(span, expr).into())
    }
    #[inline]
    pub fn cond_expr(&mut self, span: Span, test: Expr, cons: Expr, alt: Expr) -> CondExpr {
        let _f0 = self.add_extra(ExtraData {
            node: test.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: cons.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            node: alt.node_id(),
        });
        CondExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::CondExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn call_expr(
        &mut self,
        span: Span,
        callee: Callee,
        args: TypedSubRange<ExprOrSpread>,
    ) -> CallExpr {
        let _f0 = self.add_extra(ExtraData {
            node: callee.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: args.into(),
        });
        CallExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::CallExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn new_expr(
        &mut self,
        span: Span,
        callee: Expr,
        args: TypedSubRange<ExprOrSpread>,
    ) -> NewExpr {
        let _f0 = self.add_extra(ExtraData {
            node: callee.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: args.into(),
        });
        NewExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::NewExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn seq_expr(&mut self, span: Span, exprs: TypedSubRange<Expr>) -> SeqExpr {
        let _f0 = self.add_extra(ExtraData {
            sub_range: exprs.into(),
        });
        SeqExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::SeqExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn arrow_expr(
        &mut self,
        span: Span,
        params: TypedSubRange<Pat>,
        body: BlockStmtOrExpr,
        is_async: bool,
        is_generator: bool,
    ) -> ArrowExpr {
        let _f0 = self.add_extra(ExtraData {
            sub_range: params.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_async.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            bool: is_generator.into(),
        });
        ArrowExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::ArrowExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn yield_expr(&mut self, span: Span, arg: Option<Expr>, delegate: bool) -> YieldExpr {
        let _f0 = self.add_extra(ExtraData {
            optional_node: arg.optional_node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: delegate.into(),
        });
        YieldExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::YieldExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn meta_prop_expr(&mut self, span: Span, kind: MetaPropKind) -> MetaPropExpr {
        let _f0 = self.add_extra(ExtraData {
            other: kind.to_extra_data(),
        });
        MetaPropExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::MetaPropExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn await_expr(&mut self, span: Span, arg: Expr) -> AwaitExpr {
        let _f0 = self.add_extra(ExtraData {
            node: arg.node_id(),
        });
        AwaitExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::AwaitExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn tpl(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
        quasis: TypedSubRange<TplElement>,
    ) -> Tpl {
        let _f0 = self.add_extra(ExtraData {
            sub_range: exprs.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: quasis.into(),
        });
        Tpl(self.add_node(AstNode {
            span,
            kind: NodeKind::Tpl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn tagged_tpl(&mut self, span: Span, tag: Expr, tpl: Tpl) -> TaggedTpl {
        let _f0 = self.add_extra(ExtraData {
            node: tag.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: tpl.node_id(),
        });
        TaggedTpl(self.add_node(AstNode {
            span,
            kind: NodeKind::TaggedTpl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn tpl_element(
        &mut self,
        span: Span,
        tail: bool,
        cooked: OptionalWtf8AtomRef,
        raw: AtomRef,
    ) -> TplElement {
        let _f0 = self.add_extra(ExtraData { bool: tail.into() });
        let _f1 = self.add_extra(ExtraData {
            optional_wtf8_atom: cooked.into(),
        });
        let _f2 = self.add_extra(ExtraData { atom: raw.into() });
        TplElement(self.add_node(AstNode {
            span,
            kind: NodeKind::TplElement,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn paren_expr(&mut self, span: Span, expr: Expr) -> ParenExpr {
        let _f0 = self.add_extra(ExtraData {
            node: expr.node_id(),
        });
        ParenExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::ParenExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn callee_super(&mut self, span: Span) -> Callee {
        Callee::Super(self.super_(span).into())
    }
    #[inline]
    pub fn callee_import(&mut self, span: Span, phase: ImportPhase) -> Callee {
        Callee::Import(self.import(span, phase).into())
    }
    #[inline]
    pub fn callee_expr_this_expr(&mut self, span: Span) -> Callee {
        Callee::Expr(Expr::This(self.this_expr(span).into()))
    }
    #[inline]
    pub fn callee_expr_array_lit(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<ExprOrSpread>>,
    ) -> Callee {
        Callee::Expr(Expr::Array(self.array_lit(span, elems).into()))
    }
    #[inline]
    pub fn callee_expr_object_lit(
        &mut self,
        span: Span,
        props: TypedSubRange<PropOrSpread>,
    ) -> Callee {
        Callee::Expr(Expr::Object(self.object_lit(span, props).into()))
    }
    #[inline]
    pub fn callee_expr_fn_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        function: Function,
    ) -> Callee {
        Callee::Expr(Expr::Fn(self.fn_expr(span, ident, function).into()))
    }
    #[inline]
    pub fn callee_expr_unary_expr(&mut self, span: Span, op: UnaryOp, arg: Expr) -> Callee {
        Callee::Expr(Expr::Unary(self.unary_expr(span, op, arg).into()))
    }
    #[inline]
    pub fn callee_expr_update_expr(
        &mut self,
        span: Span,
        op: UpdateOp,
        prefix: bool,
        arg: Expr,
    ) -> Callee {
        Callee::Expr(Expr::Update(self.update_expr(span, op, prefix, arg).into()))
    }
    #[inline]
    pub fn callee_expr_bin_expr(
        &mut self,
        span: Span,
        op: BinaryOp,
        left: Expr,
        right: Expr,
    ) -> Callee {
        Callee::Expr(Expr::Bin(self.bin_expr(span, op, left, right).into()))
    }
    #[inline]
    pub fn callee_expr_assign_expr(
        &mut self,
        span: Span,
        op: AssignOp,
        left: AssignTarget,
        right: Expr,
    ) -> Callee {
        Callee::Expr(Expr::Assign(self.assign_expr(span, op, left, right).into()))
    }
    #[inline]
    pub fn callee_expr_member_expr(&mut self, span: Span, obj: Expr, prop: MemberProp) -> Callee {
        Callee::Expr(Expr::Member(self.member_expr(span, obj, prop).into()))
    }
    #[inline]
    pub fn callee_expr_super_prop_expr(
        &mut self,
        span: Span,
        obj: Super,
        prop: SuperProp,
    ) -> Callee {
        Callee::Expr(Expr::SuperProp(
            self.super_prop_expr(span, obj, prop).into(),
        ))
    }
    #[inline]
    pub fn callee_expr_cond_expr(
        &mut self,
        span: Span,
        test: Expr,
        cons: Expr,
        alt: Expr,
    ) -> Callee {
        Callee::Expr(Expr::Cond(self.cond_expr(span, test, cons, alt).into()))
    }
    #[inline]
    pub fn callee_expr_call_expr(
        &mut self,
        span: Span,
        callee: Callee,
        args: TypedSubRange<ExprOrSpread>,
    ) -> Callee {
        Callee::Expr(Expr::Call(self.call_expr(span, callee, args).into()))
    }
    #[inline]
    pub fn callee_expr_new_expr(
        &mut self,
        span: Span,
        callee: Expr,
        args: TypedSubRange<ExprOrSpread>,
    ) -> Callee {
        Callee::Expr(Expr::New(self.new_expr(span, callee, args).into()))
    }
    #[inline]
    pub fn callee_expr_seq_expr(&mut self, span: Span, exprs: TypedSubRange<Expr>) -> Callee {
        Callee::Expr(Expr::Seq(self.seq_expr(span, exprs).into()))
    }
    #[inline]
    pub fn callee_expr_ident(&mut self, span: Span, sym: AtomRef, optional: bool) -> Callee {
        Callee::Expr(Expr::Ident(self.ident(span, sym, optional).into()))
    }
    #[inline]
    pub fn callee_expr_lit_str(
        &mut self,
        span: Span,
        value: Wtf8AtomRef,
        raw: OptionalAtomRef,
    ) -> Callee {
        Callee::Expr(Expr::Lit(Lit::Str(self.str(span, value, raw).into())))
    }
    #[inline]
    pub fn callee_expr_lit_bool(&mut self, span: Span, value: bool) -> Callee {
        Callee::Expr(Expr::Lit(Lit::Bool(self.bool(span, value).into())))
    }
    #[inline]
    pub fn callee_expr_lit_null(&mut self, span: Span) -> Callee {
        Callee::Expr(Expr::Lit(Lit::Null(self.null(span).into())))
    }
    #[inline]
    pub fn callee_expr_lit_number(
        &mut self,
        span: Span,
        value: f64,
        raw: OptionalAtomRef,
    ) -> Callee {
        Callee::Expr(Expr::Lit(Lit::Num(self.number(span, value, raw).into())))
    }
    #[inline]
    pub fn callee_expr_lit_big_int(
        &mut self,
        span: Span,
        value: BigIntId,
        raw: OptionalAtomRef,
    ) -> Callee {
        Callee::Expr(Expr::Lit(Lit::BigInt(
            self.big_int(span, value, raw).into(),
        )))
    }
    #[inline]
    pub fn callee_expr_lit_regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> Callee {
        Callee::Expr(Expr::Lit(Lit::Regex(self.regex(span, exp, flags).into())))
    }
    #[inline]
    pub fn callee_expr_tpl(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
        quasis: TypedSubRange<TplElement>,
    ) -> Callee {
        Callee::Expr(Expr::Tpl(self.tpl(span, exprs, quasis).into()))
    }
    #[inline]
    pub fn callee_expr_tagged_tpl(&mut self, span: Span, tag: Expr, tpl: Tpl) -> Callee {
        Callee::Expr(Expr::TaggedTpl(self.tagged_tpl(span, tag, tpl).into()))
    }
    #[inline]
    pub fn callee_expr_arrow_expr(
        &mut self,
        span: Span,
        params: TypedSubRange<Pat>,
        body: BlockStmtOrExpr,
        is_async: bool,
        is_generator: bool,
    ) -> Callee {
        Callee::Expr(Expr::Arrow(
            self.arrow_expr(span, params, body, is_async, is_generator)
                .into(),
        ))
    }
    #[inline]
    pub fn callee_expr_class_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        class: Class,
    ) -> Callee {
        Callee::Expr(Expr::Class(self.class_expr(span, ident, class).into()))
    }
    #[inline]
    pub fn callee_expr_yield_expr(
        &mut self,
        span: Span,
        arg: Option<Expr>,
        delegate: bool,
    ) -> Callee {
        Callee::Expr(Expr::Yield(self.yield_expr(span, arg, delegate).into()))
    }
    #[inline]
    pub fn callee_expr_meta_prop_expr(&mut self, span: Span, kind: MetaPropKind) -> Callee {
        Callee::Expr(Expr::MetaProp(self.meta_prop_expr(span, kind).into()))
    }
    #[inline]
    pub fn callee_expr_await_expr(&mut self, span: Span, arg: Expr) -> Callee {
        Callee::Expr(Expr::Await(self.await_expr(span, arg).into()))
    }
    #[inline]
    pub fn callee_expr_paren_expr(&mut self, span: Span, expr: Expr) -> Callee {
        Callee::Expr(Expr::Paren(self.paren_expr(span, expr).into()))
    }
    #[inline]
    pub fn callee_expr_private_name(&mut self, span: Span, name: AtomRef) -> Callee {
        Callee::Expr(Expr::PrivateName(self.private_name(span, name).into()))
    }
    #[inline]
    pub fn callee_expr_opt_chain_expr(
        &mut self,
        span: Span,
        optional: bool,
        base: OptChainBase,
    ) -> Callee {
        Callee::Expr(Expr::OptChain(
            self.opt_chain_expr(span, optional, base).into(),
        ))
    }
    #[inline]
    pub fn callee_expr_invalid(&mut self, span: Span) -> Callee {
        Callee::Expr(Expr::Invalid(self.invalid(span).into()))
    }
    #[inline]
    pub fn super_(&mut self, span: Span) -> Super {
        Super(self.add_node(AstNode {
            span,
            kind: NodeKind::Super,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn import(&mut self, span: Span, phase: ImportPhase) -> Import {
        let _f0 = self.add_extra(ExtraData {
            other: phase.to_extra_data(),
        });
        Import(self.add_node(AstNode {
            span,
            kind: NodeKind::Import,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn expr_or_spread(
        &mut self,
        span: Span,
        spread: Option<SpreadDot3Token>,
        expr: Expr,
    ) -> ExprOrSpread {
        let _f0 = self.add_extra(ExtraData {
            optional_node: spread.optional_node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: expr.node_id(),
        });
        ExprOrSpread(self.add_node(AstNode {
            span,
            kind: NodeKind::ExprOrSpread,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn spread_dot_3_token(&mut self, span: Span) -> SpreadDot3Token {
        SpreadDot3Token(self.add_node(AstNode {
            span,
            kind: NodeKind::SpreadDot3Token,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn block_stmt_or_expr_block_stmt(
        &mut self,
        span: Span,
        stmts: TypedSubRange<Stmt>,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::BlockStmt(self.block_stmt(span, stmts).into())
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_this_expr(&mut self, span: Span) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::This(self.this_expr(span).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_array_lit(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<ExprOrSpread>>,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Array(self.array_lit(span, elems).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_object_lit(
        &mut self,
        span: Span,
        props: TypedSubRange<PropOrSpread>,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Object(self.object_lit(span, props).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_fn_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        function: Function,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Fn(self.fn_expr(span, ident, function).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_unary_expr(
        &mut self,
        span: Span,
        op: UnaryOp,
        arg: Expr,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Unary(self.unary_expr(span, op, arg).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_update_expr(
        &mut self,
        span: Span,
        op: UpdateOp,
        prefix: bool,
        arg: Expr,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Update(self.update_expr(span, op, prefix, arg).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_bin_expr(
        &mut self,
        span: Span,
        op: BinaryOp,
        left: Expr,
        right: Expr,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Bin(self.bin_expr(span, op, left, right).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_assign_expr(
        &mut self,
        span: Span,
        op: AssignOp,
        left: AssignTarget,
        right: Expr,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Assign(self.assign_expr(span, op, left, right).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_member_expr(
        &mut self,
        span: Span,
        obj: Expr,
        prop: MemberProp,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Member(self.member_expr(span, obj, prop).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_super_prop_expr(
        &mut self,
        span: Span,
        obj: Super,
        prop: SuperProp,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::SuperProp(
            self.super_prop_expr(span, obj, prop).into(),
        ))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_cond_expr(
        &mut self,
        span: Span,
        test: Expr,
        cons: Expr,
        alt: Expr,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Cond(self.cond_expr(span, test, cons, alt).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_call_expr(
        &mut self,
        span: Span,
        callee: Callee,
        args: TypedSubRange<ExprOrSpread>,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Call(self.call_expr(span, callee, args).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_new_expr(
        &mut self,
        span: Span,
        callee: Expr,
        args: TypedSubRange<ExprOrSpread>,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::New(self.new_expr(span, callee, args).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_seq_expr(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Seq(self.seq_expr(span, exprs).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_ident(
        &mut self,
        span: Span,
        sym: AtomRef,
        optional: bool,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Ident(self.ident(span, sym, optional).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_lit_str(
        &mut self,
        span: Span,
        value: Wtf8AtomRef,
        raw: OptionalAtomRef,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Lit(Lit::Str(self.str(span, value, raw).into())))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_lit_bool(&mut self, span: Span, value: bool) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Lit(Lit::Bool(self.bool(span, value).into())))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_lit_null(&mut self, span: Span) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Lit(Lit::Null(self.null(span).into())))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_lit_number(
        &mut self,
        span: Span,
        value: f64,
        raw: OptionalAtomRef,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Lit(Lit::Num(self.number(span, value, raw).into())))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_lit_big_int(
        &mut self,
        span: Span,
        value: BigIntId,
        raw: OptionalAtomRef,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Lit(Lit::BigInt(
            self.big_int(span, value, raw).into(),
        )))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_lit_regex(
        &mut self,
        span: Span,
        exp: AtomRef,
        flags: AtomRef,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Lit(Lit::Regex(self.regex(span, exp, flags).into())))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_tpl(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
        quasis: TypedSubRange<TplElement>,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Tpl(self.tpl(span, exprs, quasis).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_tagged_tpl(
        &mut self,
        span: Span,
        tag: Expr,
        tpl: Tpl,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::TaggedTpl(self.tagged_tpl(span, tag, tpl).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_arrow_expr(
        &mut self,
        span: Span,
        params: TypedSubRange<Pat>,
        body: BlockStmtOrExpr,
        is_async: bool,
        is_generator: bool,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Arrow(
            self.arrow_expr(span, params, body, is_async, is_generator)
                .into(),
        ))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_class_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        class: Class,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Class(self.class_expr(span, ident, class).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_yield_expr(
        &mut self,
        span: Span,
        arg: Option<Expr>,
        delegate: bool,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Yield(self.yield_expr(span, arg, delegate).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_meta_prop_expr(
        &mut self,
        span: Span,
        kind: MetaPropKind,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::MetaProp(self.meta_prop_expr(span, kind).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_await_expr(&mut self, span: Span, arg: Expr) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Await(self.await_expr(span, arg).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_paren_expr(
        &mut self,
        span: Span,
        expr: Expr,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Paren(self.paren_expr(span, expr).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_private_name(
        &mut self,
        span: Span,
        name: AtomRef,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::PrivateName(self.private_name(span, name).into()))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_opt_chain_expr(
        &mut self,
        span: Span,
        optional: bool,
        base: OptChainBase,
    ) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::OptChain(
            self.opt_chain_expr(span, optional, base).into(),
        ))
    }
    #[inline]
    pub fn block_stmt_or_expr_expr_invalid(&mut self, span: Span) -> BlockStmtOrExpr {
        BlockStmtOrExpr::Expr(Expr::Invalid(self.invalid(span).into()))
    }
    #[inline]
    pub fn assign_target_simple_assign_target_binding_ident(
        &mut self,
        span: Span,
        id: Ident,
    ) -> AssignTarget {
        AssignTarget::Simple(SimpleAssignTarget::Ident(
            self.binding_ident(span, id).into(),
        ))
    }
    #[inline]
    pub fn assign_target_simple_assign_target_member_expr(
        &mut self,
        span: Span,
        obj: Expr,
        prop: MemberProp,
    ) -> AssignTarget {
        AssignTarget::Simple(SimpleAssignTarget::Member(
            self.member_expr(span, obj, prop).into(),
        ))
    }
    #[inline]
    pub fn assign_target_simple_assign_target_super_prop_expr(
        &mut self,
        span: Span,
        obj: Super,
        prop: SuperProp,
    ) -> AssignTarget {
        AssignTarget::Simple(SimpleAssignTarget::SuperProp(
            self.super_prop_expr(span, obj, prop).into(),
        ))
    }
    #[inline]
    pub fn assign_target_simple_assign_target_paren_expr(
        &mut self,
        span: Span,
        expr: Expr,
    ) -> AssignTarget {
        AssignTarget::Simple(SimpleAssignTarget::Paren(
            self.paren_expr(span, expr).into(),
        ))
    }
    #[inline]
    pub fn assign_target_simple_assign_target_opt_chain_expr(
        &mut self,
        span: Span,
        optional: bool,
        base: OptChainBase,
    ) -> AssignTarget {
        AssignTarget::Simple(SimpleAssignTarget::OptChain(
            self.opt_chain_expr(span, optional, base).into(),
        ))
    }
    #[inline]
    pub fn assign_target_simple_assign_target_invalid(&mut self, span: Span) -> AssignTarget {
        AssignTarget::Simple(SimpleAssignTarget::Invalid(self.invalid(span).into()))
    }
    #[inline]
    pub fn assign_target_assign_target_pat_array_pat(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<Pat>>,
        optional: bool,
    ) -> AssignTarget {
        AssignTarget::Pat(AssignTargetPat::Array(
            self.array_pat(span, elems, optional).into(),
        ))
    }
    #[inline]
    pub fn assign_target_assign_target_pat_object_pat(
        &mut self,
        span: Span,
        props: TypedSubRange<ObjectPatProp>,
        optional: bool,
    ) -> AssignTarget {
        AssignTarget::Pat(AssignTargetPat::Object(
            self.object_pat(span, props, optional).into(),
        ))
    }
    #[inline]
    pub fn assign_target_assign_target_pat_invalid(&mut self, span: Span) -> AssignTarget {
        AssignTarget::Pat(AssignTargetPat::Invalid(self.invalid(span).into()))
    }
    #[inline]
    pub fn assign_target_pat_array_pat(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<Pat>>,
        optional: bool,
    ) -> AssignTargetPat {
        AssignTargetPat::Array(self.array_pat(span, elems, optional).into())
    }
    #[inline]
    pub fn assign_target_pat_object_pat(
        &mut self,
        span: Span,
        props: TypedSubRange<ObjectPatProp>,
        optional: bool,
    ) -> AssignTargetPat {
        AssignTargetPat::Object(self.object_pat(span, props, optional).into())
    }
    #[inline]
    pub fn assign_target_pat_invalid(&mut self, span: Span) -> AssignTargetPat {
        AssignTargetPat::Invalid(self.invalid(span).into())
    }
    #[inline]
    pub fn simple_assign_target_binding_ident(
        &mut self,
        span: Span,
        id: Ident,
    ) -> SimpleAssignTarget {
        SimpleAssignTarget::Ident(self.binding_ident(span, id).into())
    }
    #[inline]
    pub fn simple_assign_target_member_expr(
        &mut self,
        span: Span,
        obj: Expr,
        prop: MemberProp,
    ) -> SimpleAssignTarget {
        SimpleAssignTarget::Member(self.member_expr(span, obj, prop).into())
    }
    #[inline]
    pub fn simple_assign_target_super_prop_expr(
        &mut self,
        span: Span,
        obj: Super,
        prop: SuperProp,
    ) -> SimpleAssignTarget {
        SimpleAssignTarget::SuperProp(self.super_prop_expr(span, obj, prop).into())
    }
    #[inline]
    pub fn simple_assign_target_paren_expr(
        &mut self,
        span: Span,
        expr: Expr,
    ) -> SimpleAssignTarget {
        SimpleAssignTarget::Paren(self.paren_expr(span, expr).into())
    }
    #[inline]
    pub fn simple_assign_target_opt_chain_expr(
        &mut self,
        span: Span,
        optional: bool,
        base: OptChainBase,
    ) -> SimpleAssignTarget {
        SimpleAssignTarget::OptChain(self.opt_chain_expr(span, optional, base).into())
    }
    #[inline]
    pub fn simple_assign_target_invalid(&mut self, span: Span) -> SimpleAssignTarget {
        SimpleAssignTarget::Invalid(self.invalid(span).into())
    }
    #[inline]
    pub fn opt_chain_expr(
        &mut self,
        span: Span,
        optional: bool,
        base: OptChainBase,
    ) -> OptChainExpr {
        let _f0 = self.add_extra(ExtraData {
            bool: optional.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: base.node_id(),
        });
        OptChainExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::OptChainExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn opt_chain_base_member_expr(
        &mut self,
        span: Span,
        obj: Expr,
        prop: MemberProp,
    ) -> OptChainBase {
        OptChainBase::Member(self.member_expr(span, obj, prop).into())
    }
    #[inline]
    pub fn opt_chain_base_opt_call(
        &mut self,
        span: Span,
        callee: Expr,
        args: TypedSubRange<ExprOrSpread>,
    ) -> OptChainBase {
        OptChainBase::Call(self.opt_call(span, callee, args).into())
    }
    #[inline]
    pub fn opt_call(
        &mut self,
        span: Span,
        callee: Expr,
        args: TypedSubRange<ExprOrSpread>,
    ) -> OptCall {
        let _f0 = self.add_extra(ExtraData {
            node: callee.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: args.into(),
        });
        OptCall(self.add_node(AstNode {
            span,
            kind: NodeKind::OptCall,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn invalid(&mut self, span: Span) -> Invalid {
        Invalid(self.add_node(AstNode {
            span,
            kind: NodeKind::Invalid,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn function(
        &mut self,
        span: Span,
        params: TypedSubRange<Param>,
        decorators: TypedSubRange<Decorator>,
        body: Option<BlockStmt>,
        is_generator: bool,
        is_async: bool,
    ) -> Function {
        let _f0 = self.add_extra(ExtraData {
            sub_range: params.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: decorators.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            optional_node: body.optional_node_id(),
        });
        let _f3 = self.add_extra(ExtraData {
            bool: is_generator.into(),
        });
        let _f4 = self.add_extra(ExtraData {
            bool: is_async.into(),
        });
        Function(self.add_node(AstNode {
            span,
            kind: NodeKind::Function,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn param(&mut self, span: Span, decorators: TypedSubRange<Decorator>, pat: Pat) -> Param {
        let _f0 = self.add_extra(ExtraData {
            sub_range: decorators.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: pat.node_id(),
        });
        Param(self.add_node(AstNode {
            span,
            kind: NodeKind::Param,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn param_or_ts_param_prop_param(
        &mut self,
        span: Span,
        decorators: TypedSubRange<Decorator>,
        pat: Pat,
    ) -> ParamOrTsParamProp {
        ParamOrTsParamProp::Param(self.param(span, decorators, pat).into())
    }
    #[inline]
    pub fn class(
        &mut self,
        span: Span,
        decorators: TypedSubRange<Decorator>,
        body: TypedSubRange<ClassMember>,
        super_class: Option<Expr>,
        is_abstract: bool,
    ) -> Class {
        let _f0 = self.add_extra(ExtraData {
            sub_range: decorators.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: body.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            optional_node: super_class.optional_node_id(),
        });
        let _f3 = self.add_extra(ExtraData {
            bool: is_abstract.into(),
        });
        Class(self.add_node(AstNode {
            span,
            kind: NodeKind::Class,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn class_member_constructor(
        &mut self,
        span: Span,
        key: PropName,
        params: TypedSubRange<ParamOrTsParamProp>,
        body: Option<BlockStmt>,
    ) -> ClassMember {
        ClassMember::Constructor(self.constructor(span, key, params, body).into())
    }
    #[inline]
    pub fn class_member_class_method(
        &mut self,
        span: Span,
        key: PropName,
        function: Function,
        kind: MethodKind,
        is_static: bool,
    ) -> ClassMember {
        ClassMember::Method(
            self.class_method(span, key, function, kind, is_static)
                .into(),
        )
    }
    #[inline]
    pub fn class_member_private_method(
        &mut self,
        span: Span,
        key: PrivateName,
        function: Function,
        kind: MethodKind,
        is_static: bool,
    ) -> ClassMember {
        ClassMember::PrivateMethod(
            self.private_method(span, key, function, kind, is_static)
                .into(),
        )
    }
    #[inline]
    pub fn class_member_class_prop(
        &mut self,
        span: Span,
        key: PropName,
        value: Option<Expr>,
        is_static: bool,
        decorators: TypedSubRange<Decorator>,
    ) -> ClassMember {
        ClassMember::ClassProp(
            self.class_prop(span, key, value, is_static, decorators)
                .into(),
        )
    }
    #[inline]
    pub fn class_member_private_prop(
        &mut self,
        span: Span,
        key: PrivateName,
        value: Option<Expr>,
        is_static: bool,
        decorators: TypedSubRange<Decorator>,
    ) -> ClassMember {
        ClassMember::PrivateProp(
            self.private_prop(span, key, value, is_static, decorators)
                .into(),
        )
    }
    #[inline]
    pub fn class_member_empty_stmt(&mut self, span: Span) -> ClassMember {
        ClassMember::Empty(self.empty_stmt(span).into())
    }
    #[inline]
    pub fn class_member_static_block(&mut self, span: Span, body: BlockStmt) -> ClassMember {
        ClassMember::StaticBlock(self.static_block(span, body).into())
    }
    #[inline]
    pub fn class_member_auto_accessor(
        &mut self,
        span: Span,
        key: Key,
        value: Option<Expr>,
        is_static: bool,
        decorators: TypedSubRange<Decorator>,
    ) -> ClassMember {
        ClassMember::AutoAccessor(
            self.auto_accessor(span, key, value, is_static, decorators)
                .into(),
        )
    }
    #[inline]
    pub fn class_prop(
        &mut self,
        span: Span,
        key: PropName,
        value: Option<Expr>,
        is_static: bool,
        decorators: TypedSubRange<Decorator>,
    ) -> ClassProp {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: value.optional_node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_static.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            sub_range: decorators.into(),
        });
        ClassProp(self.add_node(AstNode {
            span,
            kind: NodeKind::ClassProp,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn private_prop(
        &mut self,
        span: Span,
        key: PrivateName,
        value: Option<Expr>,
        is_static: bool,
        decorators: TypedSubRange<Decorator>,
    ) -> PrivateProp {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: value.optional_node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_static.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            sub_range: decorators.into(),
        });
        PrivateProp(self.add_node(AstNode {
            span,
            kind: NodeKind::PrivateProp,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn class_method(
        &mut self,
        span: Span,
        key: PropName,
        function: Function,
        kind: MethodKind,
        is_static: bool,
    ) -> ClassMethod {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: function.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            other: kind.to_extra_data(),
        });
        let _f3 = self.add_extra(ExtraData {
            bool: is_static.into(),
        });
        ClassMethod(self.add_node(AstNode {
            span,
            kind: NodeKind::ClassMethod,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn private_method(
        &mut self,
        span: Span,
        key: PrivateName,
        function: Function,
        kind: MethodKind,
        is_static: bool,
    ) -> PrivateMethod {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: function.node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            other: kind.to_extra_data(),
        });
        let _f3 = self.add_extra(ExtraData {
            bool: is_static.into(),
        });
        PrivateMethod(self.add_node(AstNode {
            span,
            kind: NodeKind::PrivateMethod,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn constructor(
        &mut self,
        span: Span,
        key: PropName,
        params: TypedSubRange<ParamOrTsParamProp>,
        body: Option<BlockStmt>,
    ) -> Constructor {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            sub_range: params.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            optional_node: body.optional_node_id(),
        });
        Constructor(self.add_node(AstNode {
            span,
            kind: NodeKind::Constructor,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn decorator(&mut self, span: Span, expr: Expr) -> Decorator {
        let _f0 = self.add_extra(ExtraData {
            node: expr.node_id(),
        });
        Decorator(self.add_node(AstNode {
            span,
            kind: NodeKind::Decorator,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn static_block(&mut self, span: Span, body: BlockStmt) -> StaticBlock {
        let _f0 = self.add_extra(ExtraData {
            node: body.node_id(),
        });
        StaticBlock(self.add_node(AstNode {
            span,
            kind: NodeKind::StaticBlock,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn key_private_name(&mut self, span: Span, name: AtomRef) -> Key {
        Key::Private(self.private_name(span, name).into())
    }
    #[inline]
    pub fn key_prop_name_ident_name(&mut self, span: Span, sym: AtomRef) -> Key {
        Key::Public(PropName::Ident(self.ident_name(span, sym).into()))
    }
    #[inline]
    pub fn key_prop_name_str(
        &mut self,
        span: Span,
        value: Wtf8AtomRef,
        raw: OptionalAtomRef,
    ) -> Key {
        Key::Public(PropName::Str(self.str(span, value, raw).into()))
    }
    #[inline]
    pub fn key_prop_name_number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Key {
        Key::Public(PropName::Num(self.number(span, value, raw).into()))
    }
    #[inline]
    pub fn key_prop_name_computed_prop_name(&mut self, span: Span, expr: Expr) -> Key {
        Key::Public(PropName::Computed(
            self.computed_prop_name(span, expr).into(),
        ))
    }
    #[inline]
    pub fn key_prop_name_big_int(
        &mut self,
        span: Span,
        value: BigIntId,
        raw: OptionalAtomRef,
    ) -> Key {
        Key::Public(PropName::BigInt(self.big_int(span, value, raw).into()))
    }
    #[inline]
    pub fn auto_accessor(
        &mut self,
        span: Span,
        key: Key,
        value: Option<Expr>,
        is_static: bool,
        decorators: TypedSubRange<Decorator>,
    ) -> AutoAccessor {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: value.optional_node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_static.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            sub_range: decorators.into(),
        });
        AutoAccessor(self.add_node(AstNode {
            span,
            kind: NodeKind::AutoAccessor,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn prop_ident(&mut self, span: Span, sym: AtomRef, optional: bool) -> Prop {
        Prop::Shorthand(self.ident(span, sym, optional).into())
    }
    #[inline]
    pub fn prop_key_value_prop(&mut self, span: Span, key: PropName, value: Expr) -> Prop {
        Prop::KeyValue(self.key_value_prop(span, key, value).into())
    }
    #[inline]
    pub fn prop_assign_prop(&mut self, span: Span, key: Ident, value: Expr) -> Prop {
        Prop::Assign(self.assign_prop(span, key, value).into())
    }
    #[inline]
    pub fn prop_getter_prop(&mut self, span: Span, key: PropName, body: Option<BlockStmt>) -> Prop {
        Prop::Getter(self.getter_prop(span, key, body).into())
    }
    #[inline]
    pub fn prop_setter_prop(
        &mut self,
        span: Span,
        key: PropName,
        this_param: Option<Pat>,
        param: Pat,
        body: Option<BlockStmt>,
    ) -> Prop {
        Prop::Setter(self.setter_prop(span, key, this_param, param, body).into())
    }
    #[inline]
    pub fn prop_method_prop(&mut self, span: Span, key: PropName, function: Function) -> Prop {
        Prop::Method(self.method_prop(span, key, function).into())
    }
    #[inline]
    pub fn key_value_prop(&mut self, span: Span, key: PropName, value: Expr) -> KeyValueProp {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: value.node_id(),
        });
        KeyValueProp(self.add_node(AstNode {
            span,
            kind: NodeKind::KeyValueProp,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn assign_prop(&mut self, span: Span, key: Ident, value: Expr) -> AssignProp {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: value.node_id(),
        });
        AssignProp(self.add_node(AstNode {
            span,
            kind: NodeKind::AssignProp,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn getter_prop(
        &mut self,
        span: Span,
        key: PropName,
        body: Option<BlockStmt>,
    ) -> GetterProp {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: body.optional_node_id(),
        });
        GetterProp(self.add_node(AstNode {
            span,
            kind: NodeKind::GetterProp,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn setter_prop(
        &mut self,
        span: Span,
        key: PropName,
        this_param: Option<Pat>,
        param: Pat,
        body: Option<BlockStmt>,
    ) -> SetterProp {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: this_param.optional_node_id(),
        });
        let _f2 = self.add_extra(ExtraData {
            node: param.node_id(),
        });
        let _f3 = self.add_extra(ExtraData {
            optional_node: body.optional_node_id(),
        });
        SetterProp(self.add_node(AstNode {
            span,
            kind: NodeKind::SetterProp,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn method_prop(&mut self, span: Span, key: PropName, function: Function) -> MethodProp {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: function.node_id(),
        });
        MethodProp(self.add_node(AstNode {
            span,
            kind: NodeKind::MethodProp,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn prop_name_ident_name(&mut self, span: Span, sym: AtomRef) -> PropName {
        PropName::Ident(self.ident_name(span, sym).into())
    }
    #[inline]
    pub fn prop_name_str(
        &mut self,
        span: Span,
        value: Wtf8AtomRef,
        raw: OptionalAtomRef,
    ) -> PropName {
        PropName::Str(self.str(span, value, raw).into())
    }
    #[inline]
    pub fn prop_name_number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> PropName {
        PropName::Num(self.number(span, value, raw).into())
    }
    #[inline]
    pub fn prop_name_computed_prop_name(&mut self, span: Span, expr: Expr) -> PropName {
        PropName::Computed(self.computed_prop_name(span, expr).into())
    }
    #[inline]
    pub fn prop_name_big_int(
        &mut self,
        span: Span,
        value: BigIntId,
        raw: OptionalAtomRef,
    ) -> PropName {
        PropName::BigInt(self.big_int(span, value, raw).into())
    }
    #[inline]
    pub fn computed_prop_name(&mut self, span: Span, expr: Expr) -> ComputedPropName {
        let _f0 = self.add_extra(ExtraData {
            node: expr.node_id(),
        });
        ComputedPropName(self.add_node(AstNode {
            span,
            kind: NodeKind::ComputedPropName,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn pat_binding_ident(&mut self, span: Span, id: Ident) -> Pat {
        Pat::Ident(self.binding_ident(span, id).into())
    }
    #[inline]
    pub fn pat_array_pat(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<Pat>>,
        optional: bool,
    ) -> Pat {
        Pat::Array(self.array_pat(span, elems, optional).into())
    }
    #[inline]
    pub fn pat_rest_pat(&mut self, span: Span, dot3_token: Span, arg: Pat) -> Pat {
        Pat::Rest(self.rest_pat(span, dot3_token, arg).into())
    }
    #[inline]
    pub fn pat_object_pat(
        &mut self,
        span: Span,
        props: TypedSubRange<ObjectPatProp>,
        optional: bool,
    ) -> Pat {
        Pat::Object(self.object_pat(span, props, optional).into())
    }
    #[inline]
    pub fn pat_assign_pat(&mut self, span: Span, left: Pat, right: Expr) -> Pat {
        Pat::Assign(self.assign_pat(span, left, right).into())
    }
    #[inline]
    pub fn pat_invalid(&mut self, span: Span) -> Pat {
        Pat::Invalid(self.invalid(span).into())
    }
    #[inline]
    pub fn pat_expr_this_expr(&mut self, span: Span) -> Pat {
        Pat::Expr(Expr::This(self.this_expr(span).into()))
    }
    #[inline]
    pub fn pat_expr_array_lit(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<ExprOrSpread>>,
    ) -> Pat {
        Pat::Expr(Expr::Array(self.array_lit(span, elems).into()))
    }
    #[inline]
    pub fn pat_expr_object_lit(&mut self, span: Span, props: TypedSubRange<PropOrSpread>) -> Pat {
        Pat::Expr(Expr::Object(self.object_lit(span, props).into()))
    }
    #[inline]
    pub fn pat_expr_fn_expr(
        &mut self,
        span: Span,
        ident: Option<Ident>,
        function: Function,
    ) -> Pat {
        Pat::Expr(Expr::Fn(self.fn_expr(span, ident, function).into()))
    }
    #[inline]
    pub fn pat_expr_unary_expr(&mut self, span: Span, op: UnaryOp, arg: Expr) -> Pat {
        Pat::Expr(Expr::Unary(self.unary_expr(span, op, arg).into()))
    }
    #[inline]
    pub fn pat_expr_update_expr(
        &mut self,
        span: Span,
        op: UpdateOp,
        prefix: bool,
        arg: Expr,
    ) -> Pat {
        Pat::Expr(Expr::Update(self.update_expr(span, op, prefix, arg).into()))
    }
    #[inline]
    pub fn pat_expr_bin_expr(&mut self, span: Span, op: BinaryOp, left: Expr, right: Expr) -> Pat {
        Pat::Expr(Expr::Bin(self.bin_expr(span, op, left, right).into()))
    }
    #[inline]
    pub fn pat_expr_assign_expr(
        &mut self,
        span: Span,
        op: AssignOp,
        left: AssignTarget,
        right: Expr,
    ) -> Pat {
        Pat::Expr(Expr::Assign(self.assign_expr(span, op, left, right).into()))
    }
    #[inline]
    pub fn pat_expr_member_expr(&mut self, span: Span, obj: Expr, prop: MemberProp) -> Pat {
        Pat::Expr(Expr::Member(self.member_expr(span, obj, prop).into()))
    }
    #[inline]
    pub fn pat_expr_super_prop_expr(&mut self, span: Span, obj: Super, prop: SuperProp) -> Pat {
        Pat::Expr(Expr::SuperProp(
            self.super_prop_expr(span, obj, prop).into(),
        ))
    }
    #[inline]
    pub fn pat_expr_cond_expr(&mut self, span: Span, test: Expr, cons: Expr, alt: Expr) -> Pat {
        Pat::Expr(Expr::Cond(self.cond_expr(span, test, cons, alt).into()))
    }
    #[inline]
    pub fn pat_expr_call_expr(
        &mut self,
        span: Span,
        callee: Callee,
        args: TypedSubRange<ExprOrSpread>,
    ) -> Pat {
        Pat::Expr(Expr::Call(self.call_expr(span, callee, args).into()))
    }
    #[inline]
    pub fn pat_expr_new_expr(
        &mut self,
        span: Span,
        callee: Expr,
        args: TypedSubRange<ExprOrSpread>,
    ) -> Pat {
        Pat::Expr(Expr::New(self.new_expr(span, callee, args).into()))
    }
    #[inline]
    pub fn pat_expr_seq_expr(&mut self, span: Span, exprs: TypedSubRange<Expr>) -> Pat {
        Pat::Expr(Expr::Seq(self.seq_expr(span, exprs).into()))
    }
    #[inline]
    pub fn pat_expr_ident(&mut self, span: Span, sym: AtomRef, optional: bool) -> Pat {
        Pat::Expr(Expr::Ident(self.ident(span, sym, optional).into()))
    }
    #[inline]
    pub fn pat_expr_lit_str(
        &mut self,
        span: Span,
        value: Wtf8AtomRef,
        raw: OptionalAtomRef,
    ) -> Pat {
        Pat::Expr(Expr::Lit(Lit::Str(self.str(span, value, raw).into())))
    }
    #[inline]
    pub fn pat_expr_lit_bool(&mut self, span: Span, value: bool) -> Pat {
        Pat::Expr(Expr::Lit(Lit::Bool(self.bool(span, value).into())))
    }
    #[inline]
    pub fn pat_expr_lit_null(&mut self, span: Span) -> Pat {
        Pat::Expr(Expr::Lit(Lit::Null(self.null(span).into())))
    }
    #[inline]
    pub fn pat_expr_lit_number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Pat {
        Pat::Expr(Expr::Lit(Lit::Num(self.number(span, value, raw).into())))
    }
    #[inline]
    pub fn pat_expr_lit_big_int(
        &mut self,
        span: Span,
        value: BigIntId,
        raw: OptionalAtomRef,
    ) -> Pat {
        Pat::Expr(Expr::Lit(Lit::BigInt(
            self.big_int(span, value, raw).into(),
        )))
    }
    #[inline]
    pub fn pat_expr_lit_regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> Pat {
        Pat::Expr(Expr::Lit(Lit::Regex(self.regex(span, exp, flags).into())))
    }
    #[inline]
    pub fn pat_expr_tpl(
        &mut self,
        span: Span,
        exprs: TypedSubRange<Expr>,
        quasis: TypedSubRange<TplElement>,
    ) -> Pat {
        Pat::Expr(Expr::Tpl(self.tpl(span, exprs, quasis).into()))
    }
    #[inline]
    pub fn pat_expr_tagged_tpl(&mut self, span: Span, tag: Expr, tpl: Tpl) -> Pat {
        Pat::Expr(Expr::TaggedTpl(self.tagged_tpl(span, tag, tpl).into()))
    }
    #[inline]
    pub fn pat_expr_arrow_expr(
        &mut self,
        span: Span,
        params: TypedSubRange<Pat>,
        body: BlockStmtOrExpr,
        is_async: bool,
        is_generator: bool,
    ) -> Pat {
        Pat::Expr(Expr::Arrow(
            self.arrow_expr(span, params, body, is_async, is_generator)
                .into(),
        ))
    }
    #[inline]
    pub fn pat_expr_class_expr(&mut self, span: Span, ident: Option<Ident>, class: Class) -> Pat {
        Pat::Expr(Expr::Class(self.class_expr(span, ident, class).into()))
    }
    #[inline]
    pub fn pat_expr_yield_expr(&mut self, span: Span, arg: Option<Expr>, delegate: bool) -> Pat {
        Pat::Expr(Expr::Yield(self.yield_expr(span, arg, delegate).into()))
    }
    #[inline]
    pub fn pat_expr_meta_prop_expr(&mut self, span: Span, kind: MetaPropKind) -> Pat {
        Pat::Expr(Expr::MetaProp(self.meta_prop_expr(span, kind).into()))
    }
    #[inline]
    pub fn pat_expr_await_expr(&mut self, span: Span, arg: Expr) -> Pat {
        Pat::Expr(Expr::Await(self.await_expr(span, arg).into()))
    }
    #[inline]
    pub fn pat_expr_paren_expr(&mut self, span: Span, expr: Expr) -> Pat {
        Pat::Expr(Expr::Paren(self.paren_expr(span, expr).into()))
    }
    #[inline]
    pub fn pat_expr_private_name(&mut self, span: Span, name: AtomRef) -> Pat {
        Pat::Expr(Expr::PrivateName(self.private_name(span, name).into()))
    }
    #[inline]
    pub fn pat_expr_opt_chain_expr(
        &mut self,
        span: Span,
        optional: bool,
        base: OptChainBase,
    ) -> Pat {
        Pat::Expr(Expr::OptChain(
            self.opt_chain_expr(span, optional, base).into(),
        ))
    }
    #[inline]
    pub fn pat_expr_invalid(&mut self, span: Span) -> Pat {
        Pat::Expr(Expr::Invalid(self.invalid(span).into()))
    }
    #[inline]
    pub fn array_pat(
        &mut self,
        span: Span,
        elems: TypedSubRange<Option<Pat>>,
        optional: bool,
    ) -> ArrayPat {
        let _f0 = self.add_extra(ExtraData {
            sub_range: elems.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: optional.into(),
        });
        ArrayPat(self.add_node(AstNode {
            span,
            kind: NodeKind::ArrayPat,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn object_pat(
        &mut self,
        span: Span,
        props: TypedSubRange<ObjectPatProp>,
        optional: bool,
    ) -> ObjectPat {
        let _f0 = self.add_extra(ExtraData {
            sub_range: props.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: optional.into(),
        });
        ObjectPat(self.add_node(AstNode {
            span,
            kind: NodeKind::ObjectPat,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn assign_pat(&mut self, span: Span, left: Pat, right: Expr) -> AssignPat {
        let _f0 = self.add_extra(ExtraData {
            node: left.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: right.node_id(),
        });
        AssignPat(self.add_node(AstNode {
            span,
            kind: NodeKind::AssignPat,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn rest_pat(&mut self, span: Span, dot3_token: Span, arg: Pat) -> RestPat {
        let _f0 = self.add_extra(ExtraData {
            span: dot3_token.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: arg.node_id(),
        });
        RestPat(self.add_node(AstNode {
            span,
            kind: NodeKind::RestPat,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn object_pat_prop_key_value_pat_prop(
        &mut self,
        span: Span,
        key: PropName,
        value: Pat,
    ) -> ObjectPatProp {
        ObjectPatProp::KeyValue(self.key_value_pat_prop(span, key, value).into())
    }
    #[inline]
    pub fn object_pat_prop_assign_pat_prop(
        &mut self,
        span: Span,
        key: BindingIdent,
        value: Option<Expr>,
    ) -> ObjectPatProp {
        ObjectPatProp::Assign(self.assign_pat_prop(span, key, value).into())
    }
    #[inline]
    pub fn object_pat_prop_rest_pat(
        &mut self,
        span: Span,
        dot3_token: Span,
        arg: Pat,
    ) -> ObjectPatProp {
        ObjectPatProp::Rest(self.rest_pat(span, dot3_token, arg).into())
    }
    #[inline]
    pub fn key_value_pat_prop(&mut self, span: Span, key: PropName, value: Pat) -> KeyValuePatProp {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: value.node_id(),
        });
        KeyValuePatProp(self.add_node(AstNode {
            span,
            kind: NodeKind::KeyValuePatProp,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn assign_pat_prop(
        &mut self,
        span: Span,
        key: BindingIdent,
        value: Option<Expr>,
    ) -> AssignPatProp {
        let _f0 = self.add_extra(ExtraData {
            node: key.node_id(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: value.optional_node_id(),
        });
        AssignPatProp(self.add_node(AstNode {
            span,
            kind: NodeKind::AssignPatProp,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn ident(&mut self, span: Span, sym: AtomRef, optional: bool) -> Ident {
        let _f0 = self.add_extra(ExtraData { atom: sym.into() });
        let _f1 = self.add_extra(ExtraData {
            bool: optional.into(),
        });
        Ident(self.add_node(AstNode {
            span,
            kind: NodeKind::Ident,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn ident_name(&mut self, span: Span, sym: AtomRef) -> IdentName {
        let _f0 = self.add_extra(ExtraData { atom: sym.into() });
        IdentName(self.add_node(AstNode {
            span,
            kind: NodeKind::IdentName,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn private_name(&mut self, span: Span, name: AtomRef) -> PrivateName {
        let _f0 = self.add_extra(ExtraData { atom: name.into() });
        PrivateName(self.add_node(AstNode {
            span,
            kind: NodeKind::PrivateName,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn binding_ident(&mut self, span: Span, id: Ident) -> BindingIdent {
        let _f0 = self.add_extra(ExtraData { node: id.node_id() });
        BindingIdent(self.add_node(AstNode {
            span,
            kind: NodeKind::BindingIdent,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn lit_str(&mut self, span: Span, value: Wtf8AtomRef, raw: OptionalAtomRef) -> Lit {
        Lit::Str(self.str(span, value, raw).into())
    }
    #[inline]
    pub fn lit_bool(&mut self, span: Span, value: bool) -> Lit {
        Lit::Bool(self.bool(span, value).into())
    }
    #[inline]
    pub fn lit_null(&mut self, span: Span) -> Lit {
        Lit::Null(self.null(span).into())
    }
    #[inline]
    pub fn lit_number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Lit {
        Lit::Num(self.number(span, value, raw).into())
    }
    #[inline]
    pub fn lit_big_int(&mut self, span: Span, value: BigIntId, raw: OptionalAtomRef) -> Lit {
        Lit::BigInt(self.big_int(span, value, raw).into())
    }
    #[inline]
    pub fn lit_regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> Lit {
        Lit::Regex(self.regex(span, exp, flags).into())
    }
    #[inline]
    pub fn str(&mut self, span: Span, value: Wtf8AtomRef, raw: OptionalAtomRef) -> Str {
        let _f0 = self.add_extra(ExtraData {
            wtf8_atom: value.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: raw.into(),
        });
        Str(self.add_node(AstNode {
            span,
            kind: NodeKind::Str,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn bool(&mut self, span: Span, value: bool) -> Bool {
        let _f0 = self.add_extra(ExtraData { bool: value.into() });
        Bool(self.add_node(AstNode {
            span,
            kind: NodeKind::Bool,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn null(&mut self, span: Span) -> Null {
        Null(self.add_node(AstNode {
            span,
            kind: NodeKind::Null,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Number {
        let _f0 = self.add_extra(ExtraData {
            number: value.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: raw.into(),
        });
        Number(self.add_node(AstNode {
            span,
            kind: NodeKind::Number,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn big_int(&mut self, span: Span, value: BigIntId, raw: OptionalAtomRef) -> BigInt {
        let _f0 = self.add_extra(ExtraData {
            bigint: value.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: raw.into(),
        });
        BigInt(self.add_node(AstNode {
            span,
            kind: NodeKind::BigInt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> Regex {
        let _f0 = self.add_extra(ExtraData { atom: exp.into() });
        let _f1 = self.add_extra(ExtraData { atom: flags.into() });
        Regex(self.add_node(AstNode {
            span,
            kind: NodeKind::Regex,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
}
