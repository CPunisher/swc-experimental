use rspack_experimental_swc_ast_macros::ast;

#[ast]
pub struct Ident {
    sym: AtomRef,
    optional: bool,
}

#[ast]
pub struct IdentName {
    sym: AtomRef,
}

#[ast]
pub struct PrivateName {
    name: AtomRef,
}

#[ast]
pub struct BindingIdent {
    id: Ident,
}
