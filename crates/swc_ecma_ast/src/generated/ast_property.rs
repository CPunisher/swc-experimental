use crate::{ast::*, node_id::*};
impl Module {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> ModuleItem {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ModuleItem::from_node_id(ret, ast)
    }
    #[inline]
    pub fn shebang(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: ModuleItem) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
    #[inline]
    pub fn set_shebang(&self, ast: &mut crate::Ast, shebang: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = shebang.into();
    }
}
impl Script {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn shebang(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
    #[inline]
    pub fn set_shebang(&self, ast: &mut crate::Ast, shebang: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = shebang.into();
    }
}
impl ImportDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn specifiers(&self, ast: &crate::Ast) -> TypedSubRange<ImportSpecifier> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn src(&self, ast: &crate::Ast) -> Str {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Str::from_node_id(ret, ast)
    }
    #[inline]
    pub fn type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn with(&self, ast: &crate::Ast) -> Option<ObjectLit> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ObjectLit::from_node_id(id, ast))
    }
    #[inline]
    pub fn phase(&self, ast: &crate::Ast) -> ImportPhase {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 4usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_specifiers(&self, ast: &mut crate::Ast, specifiers: TypedSubRange<ImportSpecifier>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = specifiers.into();
    }
    #[inline]
    pub fn set_src(&self, ast: &mut crate::Ast, src: Str) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = src.node_id().into();
    }
    #[inline]
    pub fn set_type_only(&self, ast: &mut crate::Ast, type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = type_only.into();
    }
    #[inline]
    pub fn set_with(&self, ast: &mut crate::Ast, with: Option<ObjectLit>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].optional_node = with.optional_node_id().into();
    }
    #[inline]
    pub fn set_phase(&self, ast: &mut crate::Ast, phase: ImportPhase) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 4usize;
        ast.extra_data[offset].other = phase.into();
    }
}
impl ImportNamedSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn local(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn imported(&self, ast: &crate::Ast) -> Option<ModuleExportName> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ModuleExportName::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_local(&self, ast: &mut crate::Ast, local: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = local.node_id().into();
    }
    #[inline]
    pub fn set_imported(&self, ast: &mut crate::Ast, imported: Option<ModuleExportName>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = imported.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_type_only(&self, ast: &mut crate::Ast, is_type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_type_only.into();
    }
}
impl ImportDefaultSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn local(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_local(&self, ast: &mut crate::Ast, local: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = local.node_id().into();
    }
}
impl ImportStarAsSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn local(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_local(&self, ast: &mut crate::Ast, local: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = local.node_id().into();
    }
}
impl ExportDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn decl(&self, ast: &crate::Ast) -> Decl {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Decl::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_decl(&self, ast: &mut crate::Ast, decl: Decl) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = decl.node_id().into();
    }
}
impl NamedExport {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn specifiers(&self, ast: &crate::Ast) -> TypedSubRange<ExportSpecifier> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn src(&self, ast: &crate::Ast) -> Option<Str> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Str::from_node_id(id, ast))
    }
    #[inline]
    pub fn type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn with(&self, ast: &crate::Ast) -> Option<ObjectLit> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ObjectLit::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_specifiers(&self, ast: &mut crate::Ast, specifiers: TypedSubRange<ExportSpecifier>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = specifiers.into();
    }
    #[inline]
    pub fn set_src(&self, ast: &mut crate::Ast, src: Option<Str>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = src.optional_node_id().into();
    }
    #[inline]
    pub fn set_type_only(&self, ast: &mut crate::Ast, type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = type_only.into();
    }
    #[inline]
    pub fn set_with(&self, ast: &mut crate::Ast, with: Option<ObjectLit>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].optional_node = with.optional_node_id().into();
    }
}
impl ExportNamespaceSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn name(&self, ast: &crate::Ast) -> ModuleExportName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ModuleExportName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_name(&self, ast: &mut crate::Ast, name: ModuleExportName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = name.node_id().into();
    }
}
impl ExportDefaultSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn exported(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_exported(&self, ast: &mut crate::Ast, exported: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = exported.node_id().into();
    }
}
impl ExportNamedSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn orig(&self, ast: &crate::Ast) -> ModuleExportName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ModuleExportName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn exported(&self, ast: &crate::Ast) -> Option<ModuleExportName> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ModuleExportName::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_orig(&self, ast: &mut crate::Ast, orig: ModuleExportName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = orig.node_id().into();
    }
    #[inline]
    pub fn set_exported(&self, ast: &mut crate::Ast, exported: Option<ModuleExportName>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = exported.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_type_only(&self, ast: &mut crate::Ast, is_type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_type_only.into();
    }
}
impl ExportDefaultDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn decl(&self, ast: &crate::Ast) -> DefaultDecl {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        DefaultDecl::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_decl(&self, ast: &mut crate::Ast, decl: DefaultDecl) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = decl.node_id().into();
    }
}
impl ExportDefaultExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn expr(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_expr(&self, ast: &mut crate::Ast, expr: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = expr.node_id().into();
    }
}
impl ExportAll {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn src(&self, ast: &crate::Ast) -> Str {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Str::from_node_id(ret, ast)
    }
    #[inline]
    pub fn type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn with(&self, ast: &crate::Ast) -> Option<ObjectLit> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ObjectLit::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_src(&self, ast: &mut crate::Ast, src: Str) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = src.node_id().into();
    }
    #[inline]
    pub fn set_type_only(&self, ast: &mut crate::Ast, type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = type_only.into();
    }
    #[inline]
    pub fn set_with(&self, ast: &mut crate::Ast, with: Option<ObjectLit>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = with.optional_node_id().into();
    }
}
impl BlockStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn stmts(&self, ast: &crate::Ast) -> TypedSubRange<Stmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_stmts(&self, ast: &mut crate::Ast, stmts: TypedSubRange<Stmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = stmts.into();
    }
}
impl ExprStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn expr(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_expr(&self, ast: &mut crate::Ast, expr: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = expr.node_id().into();
    }
}
impl EmptyStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl DebuggerStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl WithStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn obj(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_obj(&self, ast: &mut crate::Ast, obj: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = obj.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl ReturnStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = arg.optional_node_id().into();
    }
}
impl LabeledStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn label(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_label(&self, ast: &mut crate::Ast, label: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = label.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl BreakStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn label(&self, ast: &crate::Ast) -> Option<Ident> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Ident::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_label(&self, ast: &mut crate::Ast, label: Option<Ident>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = label.optional_node_id().into();
    }
}
impl ContinueStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn label(&self, ast: &crate::Ast) -> Option<Ident> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Ident::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_label(&self, ast: &mut crate::Ast, label: Option<Ident>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = label.optional_node_id().into();
    }
}
impl IfStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn cons(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn alt(&self, ast: &crate::Ast) -> Option<Stmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Stmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = test.node_id().into();
    }
    #[inline]
    pub fn set_cons(&self, ast: &mut crate::Ast, cons: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = cons.node_id().into();
    }
    #[inline]
    pub fn set_alt(&self, ast: &mut crate::Ast, alt: Option<Stmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = alt.optional_node_id().into();
    }
}
impl SwitchStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn discriminant(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn cases(&self, ast: &crate::Ast) -> TypedSubRange<SwitchCase> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_discriminant(&self, ast: &mut crate::Ast, discriminant: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = discriminant.node_id().into();
    }
    #[inline]
    pub fn set_cases(&self, ast: &mut crate::Ast, cases: TypedSubRange<SwitchCase>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = cases.into();
    }
}
impl ThrowStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl TryStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn block(&self, ast: &crate::Ast) -> BlockStmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BlockStmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn handler(&self, ast: &crate::Ast) -> Option<CatchClause> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| CatchClause::from_node_id(id, ast))
    }
    #[inline]
    pub fn finalizer(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_block(&self, ast: &mut crate::Ast, block: BlockStmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = block.node_id().into();
    }
    #[inline]
    pub fn set_handler(&self, ast: &mut crate::Ast, handler: Option<CatchClause>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = handler.optional_node_id().into();
    }
    #[inline]
    pub fn set_finalizer(&self, ast: &mut crate::Ast, finalizer: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = finalizer.optional_node_id().into();
    }
}
impl WhileStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = test.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl DoWhileStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = test.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl ForStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn init(&self, ast: &crate::Ast) -> Option<VarDeclOrExpr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| VarDeclOrExpr::from_node_id(id, ast))
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn update(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_init(&self, ast: &mut crate::Ast, init: Option<VarDeclOrExpr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = init.optional_node_id().into();
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = test.optional_node_id().into();
    }
    #[inline]
    pub fn set_update(&self, ast: &mut crate::Ast, update: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = update.optional_node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl ForInStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> ForHead {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ForHead::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: ForHead) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl ForOfStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn is_await(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> ForHead {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ForHead::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_is_await(&self, ast: &mut crate::Ast, is_await: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = is_await.into();
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: ForHead) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl SwitchCase {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn cons(&self, ast: &crate::Ast) -> TypedSubRange<Stmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = test.optional_node_id().into();
    }
    #[inline]
    pub fn set_cons(&self, ast: &mut crate::Ast, cons: TypedSubRange<Stmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = cons.into();
    }
}
impl CatchClause {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn param(&self, ast: &crate::Ast) -> Option<Pat> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Pat::from_node_id(id, ast))
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> BlockStmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BlockStmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_param(&self, ast: &mut crate::Ast, param: Option<Pat>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = param.optional_node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: BlockStmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl FnDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn ident(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn declare(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_ident(&self, ast: &mut crate::Ast, ident: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = ident.node_id().into();
    }
    #[inline]
    pub fn set_declare(&self, ast: &mut crate::Ast, declare: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = declare.into();
    }
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = function.node_id().into();
    }
}
impl ClassDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn ident(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn declare(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn class(&self, ast: &crate::Ast) -> Class {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Class::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_ident(&self, ast: &mut crate::Ast, ident: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = ident.node_id().into();
    }
    #[inline]
    pub fn set_declare(&self, ast: &mut crate::Ast, declare: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = declare.into();
    }
    #[inline]
    pub fn set_class(&self, ast: &mut crate::Ast, class: Class) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = class.node_id().into();
    }
}
impl VarDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn kind(&self, ast: &crate::Ast) -> VarDeclKind {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn declare(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn decls(&self, ast: &crate::Ast) -> TypedSubRange<VarDeclarator> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_kind(&self, ast: &mut crate::Ast, kind: VarDeclKind) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = kind.into();
    }
    #[inline]
    pub fn set_declare(&self, ast: &mut crate::Ast, declare: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = declare.into();
    }
    #[inline]
    pub fn set_decls(&self, ast: &mut crate::Ast, decls: TypedSubRange<VarDeclarator>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].sub_range = decls.into();
    }
}
impl VarDeclarator {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn name(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn init(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_name(&self, ast: &mut crate::Ast, name: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = name.node_id().into();
    }
    #[inline]
    pub fn set_init(&self, ast: &mut crate::Ast, init: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = init.optional_node_id().into();
    }
}
impl UsingDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn is_await(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn decls(&self, ast: &crate::Ast) -> TypedSubRange<VarDeclarator> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_is_await(&self, ast: &mut crate::Ast, is_await: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = is_await.into();
    }
    #[inline]
    pub fn set_decls(&self, ast: &mut crate::Ast, decls: TypedSubRange<VarDeclarator>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = decls.into();
    }
}
impl ThisExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl ArrayLit {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn elems(&self, ast: &crate::Ast) -> TypedSubRange<ExprOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_elems(&self, ast: &mut crate::Ast, elems: TypedSubRange<ExprOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = elems.into();
    }
}
impl ObjectLit {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn props(&self, ast: &crate::Ast) -> TypedSubRange<PropOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_props(&self, ast: &mut crate::Ast, props: TypedSubRange<PropOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = props.into();
    }
}
impl SpreadElement {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn dot_3_token(&self, ast: &crate::Ast) -> Span {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].span };
        ret.into()
    }
    #[inline]
    pub fn expr(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_dot3_token(&self, ast: &mut crate::Ast, dot3_token: Span) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].span = dot3_token.into();
    }
    #[inline]
    pub fn set_expr(&self, ast: &mut crate::Ast, expr: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = expr.node_id().into();
    }
}
impl UnaryExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn op(&self, ast: &crate::Ast) -> UnaryOp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_op(&self, ast: &mut crate::Ast, op: UnaryOp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = op.into();
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl UpdateExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn op(&self, ast: &crate::Ast) -> UpdateOp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn prefix(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_op(&self, ast: &mut crate::Ast, op: UpdateOp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = op.into();
    }
    #[inline]
    pub fn set_prefix(&self, ast: &mut crate::Ast, prefix: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = prefix.into();
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl BinExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn op(&self, ast: &crate::Ast) -> BinaryOp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_op(&self, ast: &mut crate::Ast, op: BinaryOp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = op.into();
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
}
impl FnExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn ident(&self, ast: &crate::Ast) -> Option<Ident> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Ident::from_node_id(id, ast))
    }
    #[inline]
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_ident(&self, ast: &mut crate::Ast, ident: Option<Ident>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = ident.optional_node_id().into();
    }
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = function.node_id().into();
    }
}
impl ClassExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn ident(&self, ast: &crate::Ast) -> Option<Ident> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Ident::from_node_id(id, ast))
    }
    #[inline]
    pub fn class(&self, ast: &crate::Ast) -> Class {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Class::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_ident(&self, ast: &mut crate::Ast, ident: Option<Ident>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = ident.optional_node_id().into();
    }
    #[inline]
    pub fn set_class(&self, ast: &mut crate::Ast, class: Class) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = class.node_id().into();
    }
}
impl AssignExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn op(&self, ast: &crate::Ast) -> AssignOp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> AssignTarget {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        AssignTarget::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_op(&self, ast: &mut crate::Ast, op: AssignOp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = op.into();
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: AssignTarget) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
}
impl MemberExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn obj(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn prop(&self, ast: &crate::Ast) -> MemberProp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        MemberProp::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_obj(&self, ast: &mut crate::Ast, obj: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = obj.node_id().into();
    }
    #[inline]
    pub fn set_prop(&self, ast: &mut crate::Ast, prop: MemberProp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = prop.node_id().into();
    }
}
impl SuperPropExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn obj(&self, ast: &crate::Ast) -> Super {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Super::from_node_id(ret, ast)
    }
    #[inline]
    pub fn prop(&self, ast: &crate::Ast) -> SuperProp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        SuperProp::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_obj(&self, ast: &mut crate::Ast, obj: Super) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = obj.node_id().into();
    }
    #[inline]
    pub fn set_prop(&self, ast: &mut crate::Ast, prop: SuperProp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = prop.node_id().into();
    }
}
impl CondExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn cons(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn alt(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = test.node_id().into();
    }
    #[inline]
    pub fn set_cons(&self, ast: &mut crate::Ast, cons: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = cons.node_id().into();
    }
    #[inline]
    pub fn set_alt(&self, ast: &mut crate::Ast, alt: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = alt.node_id().into();
    }
}
impl CallExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn callee(&self, ast: &crate::Ast) -> Callee {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Callee::from_node_id(ret, ast)
    }
    #[inline]
    pub fn args(&self, ast: &crate::Ast) -> TypedSubRange<ExprOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_callee(&self, ast: &mut crate::Ast, callee: Callee) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = callee.node_id().into();
    }
    #[inline]
    pub fn set_args(&self, ast: &mut crate::Ast, args: TypedSubRange<ExprOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = args.into();
    }
}
impl NewExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn callee(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn args(&self, ast: &crate::Ast) -> TypedSubRange<ExprOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_callee(&self, ast: &mut crate::Ast, callee: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = callee.node_id().into();
    }
    #[inline]
    pub fn set_args(&self, ast: &mut crate::Ast, args: TypedSubRange<ExprOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = args.into();
    }
}
impl SeqExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn exprs(&self, ast: &crate::Ast) -> TypedSubRange<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_exprs(&self, ast: &mut crate::Ast, exprs: TypedSubRange<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = exprs.into();
    }
}
impl ArrowExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn params(&self, ast: &crate::Ast) -> TypedSubRange<Pat> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> BlockStmtOrExpr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BlockStmtOrExpr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn is_async(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn is_generator(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_params(&self, ast: &mut crate::Ast, params: TypedSubRange<Pat>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = params.into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: BlockStmtOrExpr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
    #[inline]
    pub fn set_is_async(&self, ast: &mut crate::Ast, is_async: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_async.into();
    }
    #[inline]
    pub fn set_is_generator(&self, ast: &mut crate::Ast, is_generator: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].bool = is_generator.into();
    }
}
impl YieldExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn delegate(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = arg.optional_node_id().into();
    }
    #[inline]
    pub fn set_delegate(&self, ast: &mut crate::Ast, delegate: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = delegate.into();
    }
}
impl MetaPropExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn kind(&self, ast: &crate::Ast) -> MetaPropKind {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_kind(&self, ast: &mut crate::Ast, kind: MetaPropKind) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = kind.into();
    }
}
impl AwaitExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl Tpl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn exprs(&self, ast: &crate::Ast) -> TypedSubRange<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn quasis(&self, ast: &crate::Ast) -> TypedSubRange<TplElement> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_exprs(&self, ast: &mut crate::Ast, exprs: TypedSubRange<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = exprs.into();
    }
    #[inline]
    pub fn set_quasis(&self, ast: &mut crate::Ast, quasis: TypedSubRange<TplElement>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = quasis.into();
    }
}
impl TaggedTpl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn tag(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn tpl(&self, ast: &crate::Ast) -> Tpl {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Tpl::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_tag(&self, ast: &mut crate::Ast, tag: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = tag.node_id().into();
    }
    #[inline]
    pub fn set_tpl(&self, ast: &mut crate::Ast, tpl: Tpl) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = tpl.node_id().into();
    }
}
impl TplElement {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn tail(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn cooked(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn raw(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_tail(&self, ast: &mut crate::Ast, tail: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = tail.into();
    }
    #[inline]
    pub fn set_cooked(&self, ast: &mut crate::Ast, cooked: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = cooked.into();
    }
    #[inline]
    pub fn set_raw(&self, ast: &mut crate::Ast, raw: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].atom = raw.into();
    }
}
impl ParenExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn expr(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_expr(&self, ast: &mut crate::Ast, expr: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = expr.node_id().into();
    }
}
impl Super {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl Import {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn phase(&self, ast: &crate::Ast) -> ImportPhase {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_phase(&self, ast: &mut crate::Ast, phase: ImportPhase) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = phase.into();
    }
}
impl OptChainExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn optional(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn base(&self, ast: &crate::Ast) -> OptChainBase {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        OptChainBase::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_optional(&self, ast: &mut crate::Ast, optional: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = optional.into();
    }
    #[inline]
    pub fn set_base(&self, ast: &mut crate::Ast, base: OptChainBase) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = base.node_id().into();
    }
}
impl OptCall {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn callee(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn args(&self, ast: &crate::Ast) -> TypedSubRange<ExprOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_callee(&self, ast: &mut crate::Ast, callee: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = callee.node_id().into();
    }
    #[inline]
    pub fn set_args(&self, ast: &mut crate::Ast, args: TypedSubRange<ExprOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = args.into();
    }
}
impl Invalid {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl Function {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn params(&self, ast: &crate::Ast) -> TypedSubRange<Param> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_generator(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn is_async(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_params(&self, ast: &mut crate::Ast, params: TypedSubRange<Param>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = params.into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = body.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_generator(&self, ast: &mut crate::Ast, is_generator: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_generator.into();
    }
    #[inline]
    pub fn set_is_async(&self, ast: &mut crate::Ast, is_async: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].bool = is_async.into();
    }
}
impl Param {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn pat(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_pat(&self, ast: &mut crate::Ast, pat: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = pat.node_id().into();
    }
}
impl Class {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> TypedSubRange<ClassMember> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn super_class(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_abstract(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: TypedSubRange<ClassMember>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = body.into();
    }
    #[inline]
    pub fn set_super_class(&self, ast: &mut crate::Ast, super_class: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = super_class.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_abstract(&self, ast: &mut crate::Ast, is_abstract: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_abstract.into();
    }
}
impl ClassProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = value.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl PrivateProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PrivateName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PrivateName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PrivateName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = value.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl ClassMethod {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
    }
    #[inline]
    pub fn kind(&self, ast: &crate::Ast) -> MethodKind {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = function.node_id().into();
    }
    #[inline]
    pub fn set_kind(&self, ast: &mut crate::Ast, kind: MethodKind) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].other = kind.into();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl PrivateMethod {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PrivateName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PrivateName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
    }
    #[inline]
    pub fn kind(&self, ast: &crate::Ast) -> MethodKind {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ret.into()
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PrivateName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = function.node_id().into();
    }
    #[inline]
    pub fn set_kind(&self, ast: &mut crate::Ast, kind: MethodKind) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].other = kind.into();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl Constructor {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn params(&self, ast: &crate::Ast) -> TypedSubRange<ParamOrTsParamProp> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_params(&self, ast: &mut crate::Ast, params: TypedSubRange<ParamOrTsParamProp>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = params.into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = body.optional_node_id().into();
    }
}
impl StaticBlock {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> BlockStmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BlockStmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: BlockStmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl AutoAccessor {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> Span {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].span };
        ret.into()
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> Key {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Key::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: Span) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].span = span.into();
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: Key) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = value.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl KeyValueProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = value.node_id().into();
    }
}
impl AssignProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = value.node_id().into();
    }
}
impl GetterProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = body.optional_node_id().into();
    }
}
impl SetterProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn this_param(&self, ast: &crate::Ast) -> Option<Pat> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Pat::from_node_id(id, ast))
    }
    #[inline]
    pub fn param(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_this_param(&self, ast: &mut crate::Ast, this_param: Option<Pat>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = this_param.optional_node_id().into();
    }
    #[inline]
    pub fn set_param(&self, ast: &mut crate::Ast, param: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = param.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].optional_node = body.optional_node_id().into();
    }
}
impl MethodProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = function.node_id().into();
    }
}
impl ComputedPropName {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn expr(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_expr(&self, ast: &mut crate::Ast, expr: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = expr.node_id().into();
    }
}
impl ArrayPat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn elems(&self, ast: &crate::Ast) -> TypedSubRange<Option<Pat>> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn optional(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_elems(&self, ast: &mut crate::Ast, elems: TypedSubRange<Option<Pat>>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = elems.into();
    }
    #[inline]
    pub fn set_optional(&self, ast: &mut crate::Ast, optional: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = optional.into();
    }
}
impl ObjectPat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn props(&self, ast: &crate::Ast) -> TypedSubRange<ObjectPatProp> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn optional(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_props(&self, ast: &mut crate::Ast, props: TypedSubRange<ObjectPatProp>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = props.into();
    }
    #[inline]
    pub fn set_optional(&self, ast: &mut crate::Ast, optional: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = optional.into();
    }
}
impl AssignPat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
}
impl RestPat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn dot_3_token(&self, ast: &crate::Ast) -> Span {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].span };
        ret.into()
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_dot3_token(&self, ast: &mut crate::Ast, dot3_token: Span) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].span = dot3_token.into();
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl KeyValuePatProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = value.node_id().into();
    }
}
impl AssignPatProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> BindingIdent {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BindingIdent::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: BindingIdent) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = value.optional_node_id().into();
    }
}
impl Ident {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn sym(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn optional(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_sym(&self, ast: &mut crate::Ast, sym: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = sym.into();
    }
    #[inline]
    pub fn set_optional(&self, ast: &mut crate::Ast, optional: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = optional.into();
    }
}
impl IdentName {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn sym(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_sym(&self, ast: &mut crate::Ast, sym: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = sym.into();
    }
}
impl PrivateName {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn name(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_name(&self, ast: &mut crate::Ast, name: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = name.into();
    }
}
impl BindingIdent {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn id(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_id(&self, ast: &mut crate::Ast, id: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = id.node_id().into();
    }
}
impl Str {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn raw(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = value.into();
    }
    #[inline]
    pub fn set_raw(&self, ast: &mut crate::Ast, raw: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = raw.into();
    }
}
impl Bool {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = value.into();
    }
}
impl Null {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl Num {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> f64 {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].number };
        ret.into()
    }
    #[inline]
    pub fn raw(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: f64) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].number = value.into();
    }
    #[inline]
    pub fn set_raw(&self, ast: &mut crate::Ast, raw: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = raw.into();
    }
}
impl BigInt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> BigIntId {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bigint };
        ret.into()
    }
    #[inline]
    pub fn raw(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: BigIntId) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bigint = value.into();
    }
    #[inline]
    pub fn set_raw(&self, ast: &mut crate::Ast, raw: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = raw.into();
    }
}
impl Regex {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn exp(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn flags(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_exp(&self, ast: &mut crate::Ast, exp: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = exp.into();
    }
    #[inline]
    pub fn set_flags(&self, ast: &mut crate::Ast, flags: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].atom = flags.into();
    }
}
