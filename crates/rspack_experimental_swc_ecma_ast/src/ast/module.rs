use swc_common::Span;

use crate::node_id::{NodeId, OptionalAtomId};

pub enum Program {
    Module(Module),
    Script(Script),
}

pub struct Module {
    span: Span,
    body: NodeId,
    shebang: OptionalAtomId,
}

pub struct Script {
    span: Span,
    body: NodeId,
    shebang: OptionalAtomId,
}

pub enum ModuleItem {
    ModuleDecl(NodeId),
    Stmt(NodeId),
}
