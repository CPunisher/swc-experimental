use rspack_experimental_swc_ast_macros::ast;

use crate::ast::{ClassExpr, FnExpr, Ident, Str};

#[ast]
pub enum ModuleDecl {
    Import(ImportDecl),
    ExportDecl(ExportDecl),
    ExportNamed(NamedExport),
    ExportDefaultDecl(ExportDefaultDecl),
    ExportDefaultExpr(ExportDefaultExpr),
    ExportAll(ExportAll),
    // TsImportEquals(TsImportEqualsDecl),
    // TsExportAssignment(TsExportAssignment),
    // TsNamespaceExport(TsNamespaceExportDecl),
}

#[ast]
pub struct ImportDecl {
    specifiers: Vec<ImportSpecifier>,
    src: Str,
    type_only: bool,
    with: Option<ObjectLit>,
    phase: ImportPhase,
}

#[ast]
pub enum ImportSpecifier {
    Named(ImportNamedSpecifier),
    Default(ImportDefaultSpecifier),
    Namespace(ImportStarAsSpecifier),
}

#[ast]
pub struct ImportNamedSpecifier {
    local: Ident,
    imported: Option<ModuleExportName>,
    is_type_only: bool,
}

#[ast]
pub struct ImportDefaultSpecifier {
    local: Ident,
}

#[ast]
pub struct ImportStarAsSpecifier {
    local: Ident,
}

#[ast]
pub struct ExportDecl {
    decl: Decl,
}

#[ast]
pub struct NamedExport {
    specifiers: Vec<ExportSpecifier>,
    src: Option<Str>,
    type_only: bool,
    with: Option<ObjectLit>,
}

#[ast]
pub enum ExportSpecifier {
    Namespace(ExportNamespaceSpecifier),
    Default(ExportDefaultSpecifier),
    Named(ExportNamedSpecifier),
}

#[ast]
pub struct ExportNamespaceSpecifier {
    name: ModuleExportName,
}

#[ast]
pub enum ModuleExportName {
    Ident(Ident),
    Str(Str),
}

#[ast]
pub struct ExportDefaultSpecifier {
    exported: Ident,
}

#[ast]
pub struct ExportNamedSpecifier {
    orig: ModuleExportName,
    exported: Option<ModuleExportName>,
    is_type_only: bool,
}

#[ast]
pub struct ExportDefaultDecl {
    decl: DefaultDecl,
}

#[ast]
pub enum DefaultDecl {
    Class(ClassExpr),
    Fn(FnExpr),
    // TsInterfaceDecl(TsInterfaceDecl),
}

#[ast]
pub struct ExportDefaultExpr {
    expr: Expr,
}

#[ast]
pub struct ExportAll {
    src: Str,
    type_only: bool,
    with: Option<ObjectLit>,
}
