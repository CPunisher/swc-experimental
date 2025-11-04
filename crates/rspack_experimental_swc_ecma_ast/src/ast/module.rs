use rspack_experimental_swc_ast_macros::ast;

use crate::{
    ast::{module_decl::ModuleDecl, stmt::Stmt},
    node_id::{OptionalAtomRef, TypedSubRange},
};

#[ast]
pub enum Program {
    Module(Module),
    Script(Script),
}

#[ast]
pub struct Module {
    body: TypedSubRange<ModuleItem>,
    shebang: OptionalAtomRef,
}

#[ast]
pub struct Script {
    body: TypedSubRange<Stmt>,
    shebang: OptionalAtomRef,
}

#[ast]
pub enum ModuleItem {
    ModuleDecl(ModuleDecl),
    Stmt(Stmt),
}
