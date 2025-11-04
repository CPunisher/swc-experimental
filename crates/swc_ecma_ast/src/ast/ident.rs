use rspack_experimental_swc_ast_macros::ast;

use crate::node_id::{AtomRef, TypedNodeId};

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
pub struct Privatename {
    name: AtomRef,
}

#[ast]
pub struct BindingIdent {
    id: TypedNodeId<Ident>,
}
