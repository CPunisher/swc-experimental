use swc_common::Span;

use crate::node_id::{AtomRef, BigIntId, OptionalAtomRef};

pub enum Lit {
    Str(Str),
    Bool(Bool),
    Null(Null),
    Num(Number),
    BigInt(BigInt),
    Regex(Regex),
    JSXText(JSXText),
}

pub struct Str {
    pub span: Span,
    pub value: AtomRef,
    pub raw: OptionalAtomRef,
}

pub struct Bool {
    pub span: Span,
    pub value: bool,
}

pub struct Null {
    pub span: Span,
}

pub struct Number {
    pub span: Span,
    pub value: f64,
    pub raw: OptionalAtomRef,
}

pub struct BigInt {
    pub span: Span,
    pub value: BigIntId,
    pub raw: OptionalAtomRef,
}

pub struct Regex {
    pub span: Span,
    pub exp: AtomRef,
    pub flags: AtomRef,
}

pub struct JSXText {
    pub span: Span,
    pub value: AtomRef,
    pub raw: AtomRef,
}
