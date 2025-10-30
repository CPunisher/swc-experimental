use swc_common::Span;

use crate::node_id::{NodeId, OptionalAtomRef};

pub enum Program {
    Module(Module),
    Script(Script),
}

pub struct Module {
    span: Span,
    body: NodeId,
    shebang: OptionalAtomRef,
}

pub struct Script {
    span: Span,
    body: NodeId,
    shebang: OptionalAtomRef,
}

pub enum ModuleItem {
    ModuleDecl(NodeId),
    Stmt(NodeId),
}
