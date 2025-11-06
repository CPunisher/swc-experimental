use rspack_experimental_swc_ast_macros::ast;

#[ast]
pub enum Decl {
    Class(ClassDecl),
    Fn(FnDecl),
    Var(VarDecl),
    Using(UsingDecl),
    // TsInterface(Box<TsInterfaceDecl>),
    // TsTypeAlias(Box<TsTypeAliasDecl>),
    // TsEnum(Box<TsEnumDecl>),
    // TsModule(Box<TsModuleDecl>),
}

#[ast]
pub struct FnDecl {
    ident: Ident,
    declare: bool,
    function: Function,
}

#[ast]
pub struct ClassDecl {
    ident: Ident,
    declare: bool,
    class: Class,
}

#[ast]
pub struct VarDecl {
    kind: VarDeclKind,
    declare: bool,
    decls: Vec<VarDeclarator>,
}

pub enum VarDeclKind {
    Var,
    Let,
    Const,
}

#[ast]
pub struct VarDeclarator {
    name: Pat,
    init: Option<Expr>,
    // pub definite: bool,
}

#[ast]
pub struct UsingDecl {
    is_await: bool,
    decls: Vec<VarDeclarator>,
}
