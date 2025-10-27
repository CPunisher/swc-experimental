use swc_common::Span;

use crate::node_id::AtomRef;

pub struct Ident {
    span: Span,
    sym: AtomRef,
    optional: bool,
}

pub struct BindingIdent {}
