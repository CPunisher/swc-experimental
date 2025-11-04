use swc_common::Span;

use crate::{
    Ast, AstNode, ExtraData, NodeData, NodeKind,
    ast::{
        lit::{BigInt, Bool, JSXText, Null, Number, Regex, Str},
        module::{Module, ModuleItem, Script},
        stmt::Stmt,
    },
    node_id::{AtomRef, BigIntId, OptionalAtomRef, TypedNodeId, TypedSubRange},
};

impl Ast {
    pub fn build_module(
        &mut self,
        span: Span,
        body: TypedSubRange<ModuleItem>,
        shebang: AtomRef,
    ) -> TypedNodeId<Module> {
        let body = self.add_extra(ExtraData {
            sub_range: body.into(),
        });
        let _shebang = self.add_extra(ExtraData { atom: shebang });

        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Module,
                data: NodeData {
                    extra_data_start: body,
                },
            })
            .cast_to_typed()
        }
    }

    pub fn build_script(
        &mut self,
        span: Span,
        body: TypedSubRange<Stmt>,
        shebang: AtomRef,
    ) -> TypedNodeId<Script> {
        let body = self.add_extra(ExtraData {
            sub_range: body.into(),
        });
        let _shebang = self.add_extra(ExtraData { atom: shebang });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Script,
                data: NodeData {
                    extra_data_start: body,
                },
            })
            .cast_to_typed()
        }
    }

    pub fn build_str(
        &mut self,
        span: Span,
        value: AtomRef,
        raw: OptionalAtomRef,
    ) -> TypedNodeId<Str> {
        let value = self.add_extra(ExtraData { atom: value });
        let _raw = self.add_extra(ExtraData { optional_atom: raw });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Str,
                data: NodeData {
                    extra_data_start: value,
                },
            })
            .cast_to_typed()
        }
    }

    pub fn build_bool(&mut self, span: Span, value: bool) -> TypedNodeId<Bool> {
        let value = self.add_extra(ExtraData { bool: value });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Bool,
                data: NodeData {
                    extra_data_start: value,
                },
            })
            .cast_to_typed()
        }
    }

    pub fn build_null(&mut self, span: Span) -> TypedNodeId<Null> {
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Null,
                data: NodeData { empty: () },
            })
            .cast_to_typed()
        }
    }

    pub fn build_number(
        &mut self,
        span: Span,
        value: f64,
        raw: OptionalAtomRef,
    ) -> TypedNodeId<Number> {
        let value = self.add_extra(ExtraData { number: value });
        let _raw = self.add_extra(ExtraData { optional_atom: raw });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Number,
                data: NodeData {
                    extra_data_start: value,
                },
            })
            .cast_to_typed()
        }
    }

    pub fn build_regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> TypedNodeId<Regex> {
        let exp = self.add_extra(ExtraData { atom: exp });
        let _flags = self.add_extra(ExtraData { atom: flags });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Regex,
                data: NodeData {
                    extra_data_start: exp,
                },
            })
            .cast_to_typed()
        }
    }

    pub fn build_bigint(
        &mut self,
        span: Span,
        value: BigIntId,
        raw: OptionalAtomRef,
    ) -> TypedNodeId<BigInt> {
        let value = self.add_extra(ExtraData { bigint: value });
        let _raw = self.add_extra(ExtraData { optional_atom: raw });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::BigInt,
                data: NodeData {
                    extra_data_start: value,
                },
            })
            .cast_to_typed()
        }
    }

    pub fn build_jsx_text(
        &mut self,
        span: Span,
        value: AtomRef,
        raw: AtomRef,
    ) -> TypedNodeId<JSXText> {
        let value = self.add_extra(ExtraData { atom: value });
        let _raw = self.add_extra(ExtraData { atom: raw });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::JSXText,
                data: NodeData {
                    extra_data_start: value,
                },
            })
            .cast_to_typed()
        }
    }
}
