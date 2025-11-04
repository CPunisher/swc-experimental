mod assert_layout;
mod ast;
mod node_id;
mod generated {
    mod ast_builder;
    // mod ast_visitor;
}

pub use ast::ident::Ident;
use num_bigint::BigInt;
use oxc_index::IndexVec;
use swc_atoms::Atom;
use swc_common::Span;

use crate::node_id::{
    AtomId, AtomRef, BigIntId, ExtraDataId, NodeId, OptionalAtomRef, OptionalNodeId, SubRange,
};

pub struct Ast {
    nodes: IndexVec<NodeId, AstNode>,
    extra_data: IndexVec<ExtraDataId, ExtraData>,
    allocated_str: IndexVec<AtomId, Atom>,
    bigint: IndexVec<BigIntId, BigInt>,
}

pub struct AstNode {
    span: Span,
    kind: NodeKind,
    data: NodeData,
}

pub union NodeData {
    empty: (),
    extra_data_start: ExtraDataId,
}

pub union ExtraData {
    node: NodeId,
    atom: AtomRef,
    bigint: BigIntId,
    optional_node: OptionalNodeId,
    optional_atom: OptionalAtomRef,

    bool: bool,
    number: f64,
    sub_range: SubRange,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeKind {
    // module.rs
    Module,
    Script,

    // module_decl.rs
    ImportDecl,
    ExportDecl,
    NamedExport,
    ImportDefaultSpecifier,
    ImportStarAsSpecifier,
    ImportNamedSpecifier,
    ExportNamespaceSpecifier,
    ExportDefaultSpecifier,
    ExportNamedSpecifier,
    ExportDefaultDecl,
    ExportDefaultExpr,
    ExportAll,

    // expr.rs
    ThisExpr,
    ArrayLit,
    ObjectLit,
    SpreadElement,
    UnaryMinus,
    UnaryPlus,
    UnaryBang,
    UnaryTilde,
    UnaryTypeOf,
    UnaryVoid,
    UnaryDelete,
    UpdatePlusPlus,
    UpdateMinusMinus,
    BinEqEq,
    BinNotEq,
    BinEqEqEq,
    BinNotEqEq,
    BinLt,
    BinLtEq,
    BinGt,
    BinGtEq,
    BinLShift,
    BinRShift,
    BinZeroFillRShift,
    BinAdd,
    BinSub,
    BinMul,
    BinDiv,
    BinMod,
    BinBitOr,
    BinBitXor,
    BinBitAnd,
    BinLogicalOr,
    BinLogicalAnd,
    BinIn,
    BinInstanceOf,
    BinExp,
    BinNullishCoalescing,
    FnExpr,
    AssignExpr,
    MemberExpr,
    Super,
    SuperPropExpr,
    CondExpr,
    CallExpr,
    NewExpr,
    SeqExpr,
    Lit,
    TplExpr,
    TaggedTpl,
    TplElement,
    ArrowExpr,
    ClassExpr,
    YieldExpr,
    MetaPropExpr,
    AwaitExpr,
    ParenExpr,
    OptChainExpr,
    OptCall,
    Import,
    Invalid,

    // stmt.rs
    Block,
    Empty,
    Debugger,
    With,
    Return,
    Labeled,
    Break,
    Continue,
    If,
    Switch,
    Throw,
    Try,
    While,
    DoWhile,
    For,
    ForIn,
    ForOf,
    SwitchCase,
    CatchClause,
    ExprStmt,

    // class.rs
    Class,
    ClassProp,
    PrivateProp,
    ClassMethod,
    PrivateMethod,
    Constructor,
    Decorator,
    StaticBlock,
    AutoAccessor,

    // prop.rs
    KeyValueProp,
    AssignProp,
    GetterProp,
    SetterProp,
    MethodProp,
    ComputedPropName,

    // decl.rs
    ClassDecl,
    FnDecl,
    VarDecl,
    UsingDecl,

    // function.rs
    Function,
    Param,

    // lit.rs
    BigInt,
    Str,
    Bool,
    Null,
    Regex,
    Number,

    // pat.rs
    ArrayPat,
    ObjectPat,
    AssignPat,
    RestPat,
    KeyValuePatProp,
    AssignPatProp,

    // ident.rs
    Ident,
    IdentName,
    PrivateName,

    // jsx.rs
    JSXMember,
    JSXNamespacedName,
    JSXEmpty,
    JSXExprContainer,
    JSXSpreadChild,
    JSXElementName,
    JSXOpeningElement,
    JSXClosingElement,
    JSXAttr,
    JSXText,
    JSXElement,
    JSXFragment,
    JSXOpeningFragment,
    JSXClosingFragment,

    // typescript.rs
    TsTypeAnn,
    TsTypeParamDecl,
    TsTypeParam,
    TsTypeParamInstantiation,
    TsParamProp,
    TsQualifiedName,
    TsCallSignatureDecl,
    TsConstructSignatureDecl,
    TsPropertySignature,
    TsGetterSignature,
    TsSetterSignature,
    TsMethodSignature,
    TsIndexSignature,
    TsKeywordType,
    TsThisType,
    TsFnType,
    TsConstructorType,
    TsTypeRef,
    TsTypePredicate,
    TsTypeQuery,
    TsImportCallOptions,
    TsImportType,
    TsTypeLit,
    TsArrayType,
    TsTupleType,
    TsTupleElement,
    TsOptionalType,
    TsRestType,
    TsUnionType,
    TsIntersectionType,
    TsConditionalType,
    TsInferType,
    TsParenthesizedType,
    TsTypeOperator,
    TsIndexedAccessType,
    TsMappedType,
    TsLitType,
    TsTplLitType,
    TsInterfaceDecl,
    TsInterfaceBody,
    TsExprWithTypeArgs,
    TsTypeAliasDecl,
    TsEnumDecl,
    TsEnumMember,
    TsModuleDecl,
    TsModuleBlock,
    TsNamespaceDecl,
    TsImportEqualsDecl,
    TsExternalModuleRef,
    TsExportAssginment,
    TsNamespaceExportDecl,
    TsAsExpr,
    TsTypeAssertion,
    TsNonNullExpr,
    TsSatisfiesExpr,
    TsConstAssertion,
    TsInstantiation,

    __LAST,
}

impl Ast {
    pub fn add_node(&mut self, node: AstNode) -> NodeId {
        self.nodes.push(node)
    }

    pub fn add_extra(&mut self, extra: ExtraData) -> ExtraDataId {
        self.extra_data.push(extra)
    }

    pub fn add_atom_ref(&mut self, atom: Atom) -> AtomId {
        self.allocated_str.push(atom)
    }

    pub fn add_bigint(&mut self, big_int: BigInt) -> BigIntId {
        self.bigint.push(big_int)
    }
}
