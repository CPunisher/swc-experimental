use rspack_experimental_swc_ast_macros::ast;

use crate::ast::{ModuleDecl, Stmt};

#[ast]
pub enum Program {
    Module(Module),
    Script(Script),
}

#[ast]
pub struct Module {
    body: ModuleItem,
    shebang: OptionalAtomRef,
}

#[ast]
pub struct Script {
    body: Stmt,
    shebang: OptionalAtomRef,
}

#[ast]
pub enum ModuleItem {
    ModuleDecl(ModuleDecl),
    Stmt(Stmt),
}
