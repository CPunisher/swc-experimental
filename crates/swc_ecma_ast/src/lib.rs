mod assert_layout;
mod ast;
mod ast_list;
mod common;
mod derive;
mod node_id;
mod visit;
mod generated {
    mod ast_builder;
    mod ast_clone_in;
    mod ast_node_id;
    mod ast_property;
    pub(crate) mod ast_visitor;
}

use std::marker::PhantomData;
use swc_common::BytePos;

use num_bigint::BigInt as BigIntValue;
use oxc_index::IndexVec;
use swc_atoms::{Atom, Wtf8Atom};

pub use ast::*;
pub use common::*;
pub use derive::*;
pub use generated::ast_visitor::*;
pub use node_id::{
    AtomRef, BigIntId, ExtraDataId, GetNodeId, GetOptionalNodeId, NodeId, OptionalAtomRef,
    OptionalNodeId, OptionalWtf8AtomRef, SubRange, TypedSubRange, Wtf8AtomRef,
};

use crate::{
    ast_list::NodeList,
    node_id::{AtomId, Wtf8AtomId},
};

#[derive(Default)]
pub struct Ast {
    nodes: NodeList,
    extra_data: IndexVec<ExtraDataId, ExtraData>,
    allocated_atom: IndexVec<AtomId, Atom>,
    allocated_wtf8: IndexVec<Wtf8AtomId, Wtf8Atom>,
    bigint: IndexVec<BigIntId, BigIntValue>,
}

pub struct AstNode {
    pub span: Span,
    pub kind: NodeKind,
    data: NodeData,
}

pub union NodeData {
    empty: (),
    extra_data_start: ExtraDataId,
    next_free: OptionalNodeId,
}

pub union ExtraData {
    span: Span,
    node: NodeId,
    atom: AtomRef,
    wtf8_atom: Wtf8AtomRef,
    bigint: BigIntId,
    optional_node: OptionalNodeId,
    optional_atom: OptionalAtomRef,
    optional_wtf8_atom: OptionalWtf8AtomRef,

    bool: bool,
    number: f64,
    sub_range: SubRange,

    /// Any other data (usually enum) that can be representated within 8 bytes
    other: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    ExprOrSpread,
    SpreadDot3Token,
    UnaryExpr,
    BinExpr,
    UpdateExpr,
    FnExpr,
    AssignExpr,
    MemberExpr,
    Super,
    SuperPropExpr,
    CondExpr,
    CallExpr,
    NewExpr,
    SeqExpr,
    Tpl,
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
    BlockStmt,
    EmptyStmt,
    DebuggerStmt,
    WithStmt,
    ReturnStmt,
    LabeledStmt,
    BreakStmt,
    ContinueStmt,
    IfStmt,
    SwitchStmt,
    ThrowStmt,
    TryStmt,
    WhileStmt,
    DoWhileStmt,
    ForStmt,
    ForInStmt,
    ForOfStmt,
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
    VarDeclarator,

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
    BindingIdent,
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

    __FREED,
}

impl Ast {
    #[inline]
    fn add_node(&mut self, node: AstNode) -> NodeId {
        self.nodes.add_node(node)
    }

    #[inline]
    fn add_extra(&mut self, extra: ExtraData) -> ExtraDataId {
        self.extra_data.push(extra)
    }

    #[inline]
    pub fn add_typed_sub_range<N: GetNodeId>(&mut self, range: &[NodeId]) -> TypedSubRange<N> {
        let start = self.extra_data.next_idx();
        self.extra_data
            .extend(range.iter().map(|n| ExtraData { node: *n }));
        TypedSubRange {
            inner: SubRange {
                start,
                end: self.extra_data.next_idx(),
            },
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn add_typed_opt_sub_range<N: GetOptionalNodeId>(
        &mut self,
        range: &[OptionalNodeId],
    ) -> TypedSubRange<Option<N>> {
        let start = self.extra_data.next_idx();
        self.extra_data
            .extend(range.iter().map(|n| ExtraData { optional_node: *n }));
        TypedSubRange {
            inner: SubRange {
                start,
                end: self.extra_data.next_idx(),
            },
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn add_atom_ref(&mut self, atom: Atom) -> AtomRef {
        AtomRef::new_alloc(self.allocated_atom.push(atom))
    }

    #[inline]
    pub fn add_wtf8_atom_ref(&mut self, atom: Wtf8Atom) -> Wtf8AtomRef {
        Wtf8AtomRef::new_alloc(self.allocated_wtf8.push(atom))
    }

    #[inline]
    pub fn add_optional_wtf8_atom_ref(&mut self, atom: Option<Wtf8Atom>) -> OptionalWtf8AtomRef {
        match atom {
            Some(atom) => Wtf8AtomRef::new_alloc(self.allocated_wtf8.push(atom)).into(),
            None => OptionalWtf8AtomRef::new_none(),
        }
    }

    // TODO: support atom ref
    #[inline]
    pub fn get_atom(&self, id: AtomRef) -> &Atom {
        &self.allocated_atom[AtomId::from_raw(id.lo.0)]
    }

    #[inline]
    pub fn get_optional_atom(&self, id: OptionalAtomRef) -> Option<&Atom> {
        let id = id.to_option()?;
        Some(self.get_atom(id))
    }

    #[inline]
    pub fn get_wtf8_atom(&self, id: Wtf8AtomRef) -> &Wtf8Atom {
        &self.allocated_wtf8[Wtf8AtomId::from_raw(id.lo.0)]
    }

    #[inline]
    pub fn add_bigint(&mut self, big_int: BigIntValue) -> BigIntId {
        self.bigint.push(big_int)
    }
}

impl Ast {
    #[inline]
    pub fn free_node(&mut self, node_id: NodeId) {
        self.nodes.free_node(node_id);
    }

    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &AstNode)> {
        self.nodes.iter()
    }

    pub fn nodes_len(&self) -> u32 {
        self.nodes.len()
    }
}
