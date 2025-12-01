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
use swc_atoms::wtf8::{Wtf8, Wtf8Buf};

pub use ast::*;
pub use common::*;
pub use derive::*;
pub use generated::ast_visitor::*;
pub use node_id::{
    BigIntId, ExtraDataId, FromNodeId, GetNodeId, GetOptionalNodeId, NodeId, OptionalNodeId,
    OptionalUtf8Ref, OptionalWtf8Ref, SubRange, TypedSubRange, Utf8Ref, Wtf8Ref,
};

use crate::ast_list::NodeList;

pub struct Ast {
    nodes: NodeList,
    extra_data: IndexVec<ExtraDataId, ExtraData>,
    bigint: IndexVec<BigIntId, BigIntValue>,
    allocated_utf8: String,
    allocated_wtf8: Wtf8Buf,
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
    bigint: BigIntId,
    utf8: Utf8Ref,
    wtf8: Wtf8Ref,
    optional_node: OptionalNodeId,
    optional_utf8: OptionalUtf8Ref,
    optional_wtf8: OptionalWtf8Ref,

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
    pub fn new(source_len: usize) -> Self {
        Self {
            nodes: NodeList::default(),
            extra_data: IndexVec::new(),
            bigint: IndexVec::new(),
            allocated_utf8: String::with_capacity(source_len / 2),
            allocated_wtf8: Wtf8Buf::new(),
        }
    }

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
    pub fn add_utf8(&mut self, s: &str) -> Utf8Ref {
        let lo = self.allocated_utf8.len() as u32;
        self.allocated_utf8.push_str(s);
        let hi = self.allocated_utf8.len() as u32;
        Utf8Ref::new_ref(lo, hi)
    }

    #[inline]
    pub fn add_optional_utf8(&mut self, s: Option<&str>) -> OptionalUtf8Ref {
        match s {
            Some(s) => self.add_utf8(s).into(),
            None => OptionalUtf8Ref::new_none(),
        }
    }

    #[inline]
    pub fn add_wtf8(&mut self, s: &Wtf8) -> Wtf8Ref {
        let lo = self.allocated_wtf8.len() as u32;
        self.allocated_wtf8.push_wtf8(s);
        let hi = self.allocated_wtf8.len() as u32;
        Wtf8Ref::new_ref(lo, hi)
    }

    #[inline]
    pub fn add_optional_wtf8(&mut self, s: Option<&Wtf8>) -> OptionalWtf8Ref {
        match s {
            Some(s) => self.add_wtf8(s).into(),
            None => OptionalWtf8Ref::new_none(),
        }
    }

    #[inline]
    pub fn get_utf8(&self, id: Utf8Ref) -> &str {
        &self.allocated_utf8[id.lo() as usize..id.hi() as usize]
    }

    #[inline]
    pub fn get_optional_utf8(&self, id: OptionalUtf8Ref) -> Option<&str> {
        let id = id.to_option()?;
        Some(self.get_utf8(id))
    }

    #[inline]
    pub fn get_wtf8(&self, id: Wtf8Ref) -> &Wtf8 {
        &self
            .allocated_wtf8
            .slice(id.lo() as usize, id.hi() as usize)
    }

    #[inline]
    pub fn get_optional_wtf8(&self, id: OptionalWtf8Ref) -> Option<&Wtf8> {
        let id = id.to_option()?;
        Some(self.get_wtf8(id))
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

    #[inline]
    pub fn get_node(&self, node_id: NodeId) -> &AstNode {
        &self.nodes[node_id]
    }

    #[inline]
    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &AstNode)> {
        self.nodes.iter()
    }

    #[inline]
    pub fn nodes_len(&self) -> u32 {
        self.nodes.len()
    }

    #[inline]
    pub fn nodes_capacity(&self) -> usize {
        self.nodes.capacity()
    }
}
