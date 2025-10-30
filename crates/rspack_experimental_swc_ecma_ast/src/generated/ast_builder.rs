use swc_common::Span;

use crate::{
    Ast, AstNode, ExtraData, NodeData, NodeKind,
    node_id::{AtomRef, BigIntId, NodeId, OptionalAtomRef, SubRange},
};

impl Ast {
    pub fn build_module(&mut self, span: Span, body: SubRange, shebang: AtomRef) -> NodeId {
        let body = self.add_extra(ExtraData { sub_range: body });
        let _shebang = self.add_extra(ExtraData { atom: shebang });
        self.add_node(AstNode {
            span,
            kind: NodeKind::Module,
            data: NodeData {
                extra_data_start: body,
            },
        })
    }

    pub fn build_script(&mut self, span: Span, body: SubRange, shebang: AtomRef) -> NodeId {
        let body = self.add_extra(ExtraData { sub_range: body });
        let _shebang = self.add_extra(ExtraData { atom: shebang });
        self.add_node(AstNode {
            span,
            kind: NodeKind::Script,
            data: NodeData {
                extra_data_start: body,
            },
        })
    }

    pub fn build_str(&mut self, span: Span, value: AtomRef, raw: OptionalAtomRef) -> NodeId {
        let value = self.add_extra(ExtraData { atom: value });
        let _raw = self.add_extra(ExtraData { optional_atom: raw });
        self.add_node(AstNode {
            span,
            kind: NodeKind::Str,
            data: NodeData {
                extra_data_start: value,
            },
        })
    }

    pub fn build_bool(&mut self, span: Span, value: bool) -> NodeId {
        self.add_node(AstNode {
            span,
            kind: NodeKind::Bool,
            data: NodeData { bool: value },
        })
    }

    pub fn build_null(&mut self, span: Span) -> NodeId {
        self.add_node(AstNode {
            span,
            kind: NodeKind::Null,
            data: NodeData { empty: () },
        })
    }

    pub fn build_number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> NodeId {
        let value = self.add_extra(ExtraData { number: value });
        let _raw = self.add_extra(ExtraData { optional_atom: raw });
        self.add_node(AstNode {
            span,
            kind: NodeKind::Number,
            data: NodeData {
                extra_data_start: value,
            },
        })
    }

    pub fn build_regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> NodeId {
        let exp = self.add_extra(ExtraData { atom: exp });
        let _flags = self.add_extra(ExtraData { atom: flags });
        self.add_node(AstNode {
            span,
            kind: NodeKind::Regex,
            data: NodeData {
                extra_data_start: exp,
            },
        })
    }

    pub fn build_bigint(&mut self, span: Span, value: BigIntId, raw: OptionalAtomRef) -> NodeId {
        let value = self.add_extra(ExtraData { bigint: value });
        let _raw = self.add_extra(ExtraData { optional_atom: raw });
        self.add_node(AstNode {
            span,
            kind: NodeKind::BigInt,
            data: NodeData {
                extra_data_start: value,
            },
        })
    }

    pub fn build_jsx_text(&mut self, span: Span, value: AtomRef, raw: AtomRef) -> NodeId {
        let value = self.add_extra(ExtraData { atom: value });
        let _raw = self.add_extra(ExtraData { atom: raw });
        self.add_node(AstNode {
            span,
            kind: NodeKind::JSXText,
            data: NodeData {
                extra_data_start: value,
            },
        })
    }
}
