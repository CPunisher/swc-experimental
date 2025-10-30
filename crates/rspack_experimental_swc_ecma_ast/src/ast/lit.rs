use swc_common::Span;

use crate::node_id::{AtomRef, OptionalAtomRef};

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
    span: Span,
    value: AtomRef,
    raw: OptionalAtomRef,
}

pub struct Bool {
    span: Span,
    value: bool,
}

pub struct Null {
    span: Span,
}

pub struct Number {
    pub span: Span,
    pub value: f64,
    pub raw: OptionalAtomRef,
}

pub struct BigInt {
    pub span: Span,
    // pub value: Box<BigIntValue>,
    pub raw: OptionalAtomRef,
}

pub struct Regex {
    span: Span,
    exp: AtomRef,
    flags: AtomRef,
}

pub struct JSXText {
    pub span: Span,
    pub value: AtomRef,
    pub raw: AtomRef,
}
