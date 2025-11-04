use rspack_experimental_swc_ast_macros::ast;

use crate::{
    Ident,
    ast::{
        decl::Decl,
        expr::{ClassExpr, Expr, FnExpr, ObjectLit},
        lit::Str,
    },
    node_id::{TypedNodeId, TypedOptionalNodeId, TypedSubRange},
};

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
    specifiers: TypedSubRange<ImportSpecifier>,
    src: TypedNodeId<Str>,
    type_only: bool,
    with: TypedOptionalNodeId<ObjectLit>,
    // phase: ImportPhase,
}

#[ast]
pub enum ImportSpecifier {
    Named(ImportNamedSpecifier),
    Default(ImportDefaultSpecifier),
    Namespace(ImportStarAsSpecifier),
}

#[ast]
pub struct ImportNamedSpecifier {
    local: TypedNodeId<Ident>,
    imported: TypedOptionalNodeId<ModuleExportName>,
    is_type_only: bool,
}

#[ast]
pub struct ImportDefaultSpecifier {
    local: TypedNodeId<Ident>,
}

#[ast]
pub struct ImportStarAsSpecifier {
    local: TypedNodeId<Ident>,
}

#[ast]
pub struct ExportDecl {
    decl: TypedNodeId<Decl>,
}

#[ast]
pub struct NamedExport {
    specifiers: TypedSubRange<ExportSpecifier>,
    src: TypedOptionalNodeId<Str>,
    type_only: bool,
    with: TypedOptionalNodeId<ObjectLit>,
}

#[ast]
pub enum ExportSpecifier {
    Namespace(ExportNamespaceSpecifier),
    Default(ExportDefaultSpecifier),
    Named(ExportNamedSpecifier),
}

#[ast]
pub struct ExportNamespaceSpecifier {
    name: TypedNodeId<ModuleExportName>,
}

#[ast]
pub enum ModuleExportName {
    Ident(Ident),
    Str(Str),
}

#[ast]
pub struct ExportDefaultSpecifier {
    exported: TypedNodeId<Ident>,
}

#[ast]
pub struct ExportNamedSpecifier {
    orig: TypedNodeId<ModuleExportName>,
    exported: TypedOptionalNodeId<ModuleExportName>,
    is_type_only: bool,
}

#[ast]
pub struct ExportDefaultDecl {
    decl: TypedNodeId<DefaultDecl>,
}

#[ast]
pub enum DefaultDecl {
    Class(ClassExpr),
    Fn(FnExpr),
    // TsInterfaceDecl(TsInterfaceDecl),
}

#[ast]
pub struct ExportDefaultExpr {
    expr: TypedNodeId<Expr>,
}

#[ast]
pub struct ExportAll {
    src: TypedNodeId<Str>,
    type_only: bool,
    with: TypedOptionalNodeId<ObjectLit>,
}
