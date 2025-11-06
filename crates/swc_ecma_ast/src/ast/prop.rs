use rspack_experimental_swc_ast_macros::ast;

use crate::ast::{BigInt, Expr, Ident, IdentName, Num, Str};

#[ast]
pub enum Prop {
    Shorthand(Ident),
    KeyValue(KeyValueProp),
    Assign(AssignProp),
    Getter(GetterProp),
    Setter(SetterProp),
    Method(MethodProp),
}

#[ast]
pub struct KeyValueProp {
    key: PropName,
    value: Expr,
}

pub struct AssignProp {
    key: Ident,
    value: Expr,
}

#[ast]
pub struct GetterProp {
    key: PropName,
    // type_ann: Option<TsTypeAnn>,
    body: Option<BlockStmt>,
}
#[ast]
pub struct SetterProp {
    key: PropName,
    this_param: Option<Pat>,
    param: Pat,
    body: Option<BlockStmt>,
}
#[ast]
pub struct MethodProp {
    key: PropName,
    function: Function,
}

#[ast]
pub enum PropName {
    Ident(IdentName),
    Str(Str),
    Num(Num),
    Computed(ComputedPropName),
    BigInt(BigInt),
}

#[ast]
pub struct ComputedPropName {
    expr: Expr,
}
