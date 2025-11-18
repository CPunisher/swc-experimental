use swc_experimental_ast_macros::ast;

#[ast]
pub enum Lit {
    Str(Str),
    Bool(Bool),
    Null(Null),
    Num(Number),
    BigInt(BigInt),
    Regex(Regex),
    // JSXText(JSXText),
}

#[ast]
pub struct Str {
    value: Wtf8AtomId,
    raw: OptionalAtomRef,
}

#[ast]
pub struct Bool {
    value: bool,
}

#[ast]
pub struct Null {}

#[ast]
pub struct Number {
    value: f64,
    raw: OptionalAtomRef,
}

#[ast]
pub struct BigInt {
    value: BigIntId,
    raw: OptionalAtomRef,
}

#[ast]
pub struct Regex {
    exp: AtomRef,
    flags: AtomRef,
}

// #[ast]
// pub struct JSXText {
//     value: AtomRef,
//     raw: AtomRef,
// }
