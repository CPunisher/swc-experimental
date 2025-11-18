use swc_experimental_ecma_ast::NodeId;

use crate::reference::ReferenceId;

oxc_index::define_index_type! {
    pub struct SymbolId = u32;
}

pub struct Symbol {
    node_id: NodeId,
    references: Vec<ReferenceId>,
}
