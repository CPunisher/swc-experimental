use swc_common::Span;

use crate::{
    Ast, AstNode, ExtraData, NodeKind,
    node_id::{AtomId, AtomRef, NodeId, SubRange},
};

impl Ast {
    pub fn build_module(&mut self, span: Span, body: NodeId, shebang: AtomId) -> NodeId {
        let body = self.add_extra(ExtraData { node: body });
        let shebang = self.add_extra(ExtraData { atom: shebang });
        self.add_node(AstNode {
            span,
            kind: NodeKind::Module,
            data: SubRange::new(body, shebang),
        })
    }

    pub fn build_script(&mut self, span: Span, body: NodeId, shebang: AtomId) -> NodeId {
        let body = self.add_extra(ExtraData { node: body });
        let shebang = self.add_extra(ExtraData { atom: shebang });
        self.add_node(AstNode {
            span,
            kind: NodeKind::Script,
            data: SubRange::new(body, shebang),
        })
    }
}
