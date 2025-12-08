use oxc_index::IndexVec;
use rustc_hash::{FxHashMap, FxHashSet};
use swc_experimental_ecma_ast::*;
use swc_experimental_ecma_visit::{Visit, VisitMut, VisitMutWith, VisitWith};
// use swc_ecma_utils::{find_pat_ids, stack_size::maybe_grow_default};

use crate::{legacy::utils::find_pat_ids, scope::ScopeId};

use super::scope::{DeclKind, IdentType, ScopeKind};

const LOG: bool = false && cfg!(debug_assertions);

/// See [Ident] for know how does swc manages identifiers.
///
/// # When to run
///
/// The resolver expects 'clean' ast. You can get clean ast by parsing, or by
/// removing all syntax context in ast nodes.
///
/// # What does it do
///
/// Firstly all scopes (fn, block) has it's own SyntaxContext.
/// Resolver visits all identifiers in module, and look for binding identifies
/// in the scope. Those identifiers now have the SyntaxContext of scope (fn,
/// block). While doing so, resolver tries to resolve normal identifiers (no
/// hygiene info) as a reference to identifier of scope. If the resolver find
/// suitable variable, the identifier reference will have same context as the
/// variable.
///
///
/// # Panics
///
/// `top_level_mark` should not be root.
///
/// # Example
///
/// ```js
/// let a = 1;
/// {
///     let a = 2;
///     use(a);
/// }
/// use(a)
/// ```
///
/// resolver does
///
/// 1. Define `a` with top level context.
///
/// 2. Found a block, so visit block with a new syntax context.
///
/// 3. Defined `a` with syntax context of the block statement.
///
/// 4. Found usage of `a`, and determines that it's reference to `a` in the
///    block. So the reference to `a` will have same syntax context as `a` in
///    the block.
///
/// 5. Found usage of `a` (last line), and determines that it's a reference to
///    top-level `a`, and change syntax context of `a` on last line to top-level
///    syntax context.
///
///
/// # Parameters
///
/// ## `unresolved_mark`
///
/// [Mark] applied to unresolved references.
///
/// A pass should accept this [Mark] if it's going to generate a refernce to
/// globals like `require`.
///
/// e.g. `common_js` pass generates calls to `require`, and this should not
/// be shadowed by a declaration named `require` in the same file.
/// So it uses this value.
///
/// ## `top_level_mark`
///
/// [Mark] applied to top-level bindings.
///
/// **NOTE**: This is **not** globals. This is for top level items declared by
/// users.
///
/// A pass should accept this [Mark] if it requires user-defined top-level
/// items.
///
/// e.g. `jsx` pass requires to call `React` imported by the user.
///
/// ```js
/// import React from 'react';
/// ```
///
/// In the code above, `React` has this [Mark]. `jsx` passes need to
/// reference this [Mark], so it accpets this.
///
/// This [Mark] should be used for referencing top-level bindings written by
/// user. If you are going to create a binding, use `private_ident`
/// instead.
///
/// In other words, **this [Mark] should not be used for determining if a
/// variable is top-level.** This is simply a configuration of the `resolver`
/// pass.
///
///
/// ## `typescript`
///
/// Enable this only if you are going to strip types or apply type-aware
/// passes like decorators pass.
///
///
/// # FAQ
///
/// ## Does a pair `(Atom, SyntaxContext)` always uniquely identifiers a
/// variable binding?
///
/// Yes, but multiple variables can have the exactly same name.
///
/// In the code below,
///
/// ```js
/// var a = 1, a = 2;
/// ```
///
/// both of them have the same name, so the `(Atom, SyntaxContext)` pair will
/// be also identical.
pub fn resolver<'ast, N: VisitWith<Resolver<'ast>>>(root: N, ast: &'ast Ast) -> Semantic {
    let top_level_scope = Scope::new(ScopeKind::Fn, None);
    let mut scopes = IndexVec::default();
    let top_level_scope_id = scopes.push(top_level_scope);

    let node_cap = ast.nodes_capacity();
    let mut parent_ids = IndexVec::with_capacity(node_cap);
    parent_ids.resize(node_cap, NodeId::from_raw(0));

    let mut resolver = Resolver {
        ast,
        current_node_id: NodeId::from_raw(0),
        parent_ids,
        symbol_scopes: FxHashMap::default(),
        block_scopes: FxHashMap::default(),
        scopes,

        top_level_scope_id,
        current: top_level_scope_id,

        ident_type: IdentType::Ref,
        in_type: false,
        is_module: false,
        in_ts_module: false,
        decl_kind: DeclKind::Lexical,
        strict_mode: false,
    };

    root.visit_with(&mut resolver, ast);
    Semantic {
        top_level_scope_id,
        parent_ids: resolver.parent_ids,
        symbol_scopes: resolver.symbol_scopes,
        block_scopes: resolver.block_scopes,
    }
}

pub const UNRESOLVED_SCOPE_ID: ScopeId = ScopeId::from_raw(u32::MAX);

pub struct Semantic {
    top_level_scope_id: ScopeId,
    parent_ids: IndexVec<NodeId, NodeId>,
    symbol_scopes: FxHashMap<NodeId, ScopeId>,
    block_scopes: FxHashMap<NodeId, ScopeId>,
}

impl Semantic {
    #[inline]
    pub fn parent_node(&self, node: NodeId) -> NodeId {
        self.parent_ids[node]
    }

    #[inline]
    pub fn node_scope(&self, ident: Ident) -> ScopeId {
        let Some(scope_id) = self.symbol_scopes.get(&ident.node_id()).cloned() else {
            return UNRESOLVED_SCOPE_ID;
        };

        scope_id
    }

    #[inline]
    pub fn top_level_scope_id(&self) -> ScopeId {
        self.top_level_scope_id
    }
}

#[derive(Debug, Clone)]
struct Scope<'ast> {
    /// Parent scope of the scope
    parent: Option<ScopeId>,

    /// Kind of the scope.
    kind: ScopeKind,

    /// All declarations in the scope
    declared_symbols: FxHashMap<&'ast str, DeclKind>,

    /// All types declared in the scope
    declared_types: FxHashSet<&'ast str>,
}

impl Scope<'_> {
    pub fn new(kind: ScopeKind, parent: Option<ScopeId>) -> Self {
        Scope {
            parent,
            kind,
            declared_symbols: Default::default(),
            declared_types: Default::default(),
        }
    }
}

/// # Phases
///
/// ## Hoisting phase
///
/// ## Resolving phase
pub struct Resolver<'ast> {
    // Changed
    ast: &'ast Ast,
    current_node_id: NodeId,
    parent_ids: IndexVec<NodeId, NodeId>,
    symbol_scopes: FxHashMap<NodeId, ScopeId>,
    block_scopes: FxHashMap<NodeId, ScopeId>,

    top_level_scope_id: ScopeId,
    scopes: IndexVec<ScopeId, Scope<'ast>>,
    current: ScopeId,

    ident_type: IdentType,
    in_type: bool,
    is_module: bool,
    in_ts_module: bool,
    decl_kind: DeclKind,
    strict_mode: bool,
}

impl<'ast> Resolver<'ast> {
    fn is_declared(&self, symbol: &str, start_scope: ScopeId) -> Option<&DeclKind> {
        let mut scope = Some(start_scope);
        while let Some(cur_scope) = scope {
            let cur_scope = &self.scopes[cur_scope];
            if let Some(kind) = cur_scope.declared_symbols.get(symbol) {
                return Some(kind);
            }
            scope = cur_scope.parent;
        }
        None
    }

    fn with_child<F>(&mut self, kind: ScopeKind, op: F)
    where
        F: FnOnce(&mut Resolver),
    {
        let scope = self.current;
        let ident_type = self.ident_type;
        let in_type = self.in_type;
        let is_module = self.is_module;
        let in_ts_module = self.in_ts_module;
        let decl_kind = self.decl_kind;
        let strict_mode = self.strict_mode;

        let child_scope = Scope::new(kind, Some(scope));
        let child_scope = self.scopes.push(child_scope);
        self.current = child_scope;
        op(self);

        self.current = scope;
        self.ident_type = ident_type;
        self.in_type = in_type;
        self.is_module = is_module;
        self.in_ts_module = in_ts_module;
        self.decl_kind = decl_kind;
        self.strict_mode = strict_mode;
    }

    fn visit_stmt_within_child_scope(&mut self, ast: &Ast, s: Stmt) {
        self.with_child(ScopeKind::Block, |child| match s {
            Stmt::Block(s) => {
                child.mark_block(s.node_id());
                s.visit_children_with(child, ast);
            }
            _ => s.visit_with(child, ast),
        });
    }

    /// Returns a [Mark] for an identifier reference.
    fn mark_for_ref(&self, sym: &'ast str) -> Option<ScopeId> {
        self.mark_for_ref_inner(sym, false)
    }

    fn mark_for_ref_inner(&self, sym: &'ast str, stop_an_fn_scope: bool) -> Option<ScopeId> {
        // if self.config.handle_types && self.in_type {
        //     let mut mark = self.current.mark;
        //     let mut scope = Some(&self.current);

        //     while let Some(cur) = scope {
        //         // if cur.declared_types.contains(sym) ||
        //         // cur.hoisted_symbols.borrow().contains(sym) {
        //         if cur.declared_types.contains(sym) {
        //             if mark == Mark::root() {
        //                 break;
        //             }
        //             return Some(mark);
        //         }

        //         if cur.kind == ScopeKind::Fn && stop_an_fn_scope {
        //             return None;
        //         }

        //         if let Some(parent) = &cur.parent {
        //             mark = parent.mark;
        //         }
        //         scope = cur.parent;
        //     }
        // }

        let mut scope = Some(self.current);
        while let Some(cur) = scope {
            let cur_scope = &self.scopes[cur];
            if cur_scope.declared_symbols.contains_key(sym) {
                return match sym {
                    // https://tc39.es/ecma262/multipage/global-object.html#sec-value-properties-of-the-global-object-infinity
                    // non configurable global value
                    "undefined" | "NaN" | "Infinity"
                        if cur == self.top_level_scope_id && !self.is_module =>
                    {
                        Some(UNRESOLVED_SCOPE_ID)
                    }
                    _ => Some(cur),
                };
            }

            if cur_scope.kind == ScopeKind::Fn && stop_an_fn_scope {
                return None;
            }

            scope = cur_scope.parent;
        }

        None
    }

    /// Modifies a binding identifier.
    fn modify(&mut self, ast: &'ast Ast, id: Ident, kind: DeclKind) {
        let node_id = id.node_id();

        if self.symbol_scopes.contains_key(&node_id) {
            return;
        }

        if self.in_type {
            self.scopes[self.current]
                .declared_types
                .insert(ast.get_utf8(id.sym(ast)));
        } else {
            self.scopes[self.current]
                .declared_symbols
                .insert(ast.get_utf8(id.sym(ast)), kind);
        }

        let scope_id = self.current;
        self.symbol_scopes.insert(node_id, scope_id);
    }

    fn mark_block(&mut self, node_id: NodeId) {
        if self.block_scopes.contains_key(&node_id) {
            return;
        }

        let scope_id = self.current;
        self.block_scopes.insert(node_id, scope_id);
    }

    // fn try_resolving_as_type(&mut self, i: &mut Ident, ast: &Ast) {
    //     if i.ctxt.outer() == self.config.unresolved_mark {
    //         i.ctxt = SyntaxContext::empty()
    //     }

    //     self.in_type = true;
    //     i.visit_with(self, ast);
    //     self.in_type = false;
    // }
}

// macro_rules! typed {
//     ($name:ident, $T:ty) => {
//         fn $name(&mut self, node: &mut $T) {
//             if self.config.handle_types {
//                 node.visit_children_with(self)
//             }
//         }
//     };
// }

// macro_rules! typed_ref {
//     ($name:ident, $T:ty) => {
//         fn $name(&mut self, node: &mut $T) {
//             if self.config.handle_types {
//                 let in_type = self.in_type;
//                 let ident_type = self.ident_type;
//                 self.in_type = true;
//                 node.visit_children_with(self);
//                 self.ident_type = ident_type;
//                 self.in_type = in_type;
//             }
//         }
//     };
// }

// macro_rules! typed_ref_init {
//     ($name:ident, $T:ty) => {
//         fn $name(&mut self, node: &mut $T) {
//             if self.config.handle_types {
//                 let in_type = self.in_type;
//                 let ident_type = self.ident_type;
//                 self.ident_type = IdentType::Ref;
//                 self.in_type = true;
//                 node.visit_children_with(self);
//                 self.ident_type = ident_type;
//                 self.in_type = in_type;
//             }
//         }
//     };
// }

// macro_rules! typed_decl {
//     ($name:ident, $T:ty) => {
//         fn $name(&mut self, node: &mut $T) {
//             if self.config.handle_types {
//                 let in_type = self.in_type;
//                 self.ident_type = IdentType::Binding;
//                 self.in_type = true;
//                 node.visit_children_with(self);
//                 self.in_type = in_type;
//             }
//         }
//     };
// }

// macro_rules! noop {
//     ($name:ident, $T:ty) => {
//         #[inline]
//         fn $name(&mut self, _: &mut $T) {}
//     };
// }

impl<'ast> Visit for Resolver<'ast> {
    // noop!(visit_accessibility, Accessibility);

    // noop!(visit_true_plus_minus, TruePlusMinus);

    // noop!(visit_ts_keyword_type, TsKeywordType);

    // noop!(visit_ts_keyword_type_kind, TsKeywordTypeKind);

    // noop!(visit_ts_type_operator_op, TsTypeOperatorOp);

    // noop!(visit_ts_enum_member_id, TsEnumMemberId);

    // noop!(visit_ts_external_module_ref, TsExternalModuleRef);

    // noop!(visit_ts_module_name, TsModuleName);

    // noop!(visit_ts_this_type, TsThisType);

    // typed_ref!(visit_ts_array_type, TsArrayType);

    // typed_ref!(visit_ts_conditional_type, TsConditionalType);

    // typed_ref_init!(
    //     visit_ts_type_param_instantiation,
    //     TsTypeParamInstantiation
    // );

    // typed_ref!(visit_ts_type_query, TsTypeQuery);

    // typed_ref!(visit_ts_type_query_expr, TsTypeQueryExpr);

    // typed_ref!(visit_ts_type_operator, TsTypeOperator);

    // typed_ref_init!(visit_ts_type, TsType);

    // typed_ref_init!(visit_ts_type_ann, TsTypeAnn);

    // typed!(
    //     visit_ts_union_or_intersection_type,
    //     TsUnionOrIntersectionType
    // );

    // typed!(visit_ts_fn_or_constructor_type, TsFnOrConstructorType);

    // typed_ref!(visit_ts_union_type, TsUnionType);

    // typed_ref!(visit_ts_infer_type, TsInferType);

    // typed_ref!(visit_ts_tuple_type, TsTupleType);

    // typed_ref!(visit_ts_intersection_type, TsIntersectionType);

    // typed_ref!(visit_ts_type_ref, TsTypeRef);

    // typed_decl!(visit_ts_type_param_decl, TsTypeParamDecl);

    // typed!(visit_ts_fn_param, TsFnParam);

    // typed!(visit_ts_indexed_access_type, TsIndexedAccessType);

    // typed!(visit_ts_index_signature, TsIndexSignature);

    // typed!(visit_ts_interface_body, TsInterfaceBody);

    // typed!(visit_ts_parenthesized_type, TsParenthesizedType);

    // typed!(visit_ts_type_lit, TsTypeLit);

    // typed!(visit_ts_type_element, TsTypeElement);

    // typed!(visit_ts_optional_type, TsOptionalType);

    // typed!(visit_ts_rest_type, TsRestType);

    // typed!(visit_ts_type_predicate, TsTypePredicate);

    // typed_ref!(visit_ts_this_type_or_ident, TsThisTypeOrIdent);

    // visit_obj_and_computed!();

    // // TODO: How should I handle this?
    // typed!(visit_ts_namespace_export_decl, TsNamespaceExportDecl);

    fn enter_node(&mut self, node_id: NodeId, ast: &Ast) {
        self.parent_ids[node_id] = self.current_node_id;
        self.current_node_id = node_id;
    }

    fn leave_node(&mut self, node_id: NodeId, ast: &Ast) {
        self.current_node_id = self.parent_ids[node_id];
    }

    fn visit_arrow_expr(&mut self, e: ArrowExpr, ast: &Ast) {
        self.enter_node(e.node_id(), ast);

        self.with_child(ScopeKind::Fn, |child| {
            // e.type_params.visit_with(child);

            let old = child.ident_type;
            child.ident_type = IdentType::Binding;
            {
                let params = e
                    .params(ast)
                    .iter()
                    .filter(|p| !ast.get_node_in_sub_range(*p).is_rest())
                    .flat_map(|p| find_pat_ids(child.ast, ast.get_node_in_sub_range(p)));

                for id in params {
                    child.scopes[child.current]
                        .declared_symbols
                        .insert(id, DeclKind::Param);
                }
            }
            e.params(ast).visit_with(child, ast);
            child.ident_type = old;

            match &mut e.body(ast) {
                BlockStmtOrExpr::BlockStmt(s) => {
                    child.mark_block(s.node_id());

                    let old_strict_mode = child.strict_mode;

                    if !child.strict_mode {
                        child.strict_mode = s
                            .stmts(ast)
                            .first()
                            .map(|stmt| ast.get_node_in_sub_range(stmt).is_use_strict(ast))
                            .unwrap_or(false);
                    }
                    // Prevent creating new scope.
                    s.stmts(ast).visit_with(child, ast);
                    child.strict_mode = old_strict_mode;
                }
                BlockStmtOrExpr::Expr(e) => e.visit_with(child, ast),
            }

            // e.return_type.visit_with(child);
        });

        self.leave_node(e.node_id(), ast);
    }

    fn visit_assign_pat(&mut self, node: AssignPat, ast: &Ast) {
        self.enter_node(node.node_id(), ast);

        // visit the type first so that it doesn't resolve any
        // identifiers from the others
        node.left(ast).visit_with(self, ast);
        node.right(ast).visit_with(self, ast);

        self.leave_node(node.node_id(), ast);
    }

    fn visit_binding_ident(&mut self, i: BindingIdent, ast: &Ast) {
        self.enter_node(i.node_id(), ast);

        let ident_type = self.ident_type;
        let in_type = self.in_type;

        self.ident_type = IdentType::Ref;
        // i.type_ann.visit_with(self);

        self.ident_type = ident_type;
        i.id(ast).visit_with(self, ast);

        self.in_type = in_type;
        self.ident_type = ident_type;

        self.leave_node(i.node_id(), ast);
    }

    fn visit_block_stmt(&mut self, block: BlockStmt, ast: &Ast) {
        self.enter_node(block.node_id(), ast);

        self.with_child(ScopeKind::Block, |child| {
            child.mark_block(block.node_id());
            block.visit_children_with(child, ast);
        });

        self.leave_node(block.node_id(), ast);
    }

    fn visit_break_stmt(&mut self, s: BreakStmt, ast: &Ast) {
        self.enter_node(s.node_id(), ast);

        let old = self.ident_type;
        self.ident_type = IdentType::Label;
        s.label(ast).visit_with(self, ast);
        self.ident_type = old;

        self.leave_node(s.node_id(), ast);
    }

    fn visit_catch_clause(&mut self, c: CatchClause, ast: &Ast) {
        // Child folder
        self.enter_node(c.node_id(), ast);

        self.with_child(ScopeKind::Fn, |child| {
            child.ident_type = IdentType::Binding;
            c.param(ast).visit_with(child, ast);
            child.ident_type = IdentType::Ref;

            let body = c.body(ast);
            child.mark_block(body.node_id());
            body.visit_children_with(child, ast);
        });

        self.leave_node(c.node_id(), ast);
    }

    fn visit_class(&mut self, c: Class, ast: &Ast) {
        self.enter_node(c.node_id(), ast);

        let old_strict_mode = self.strict_mode;
        self.strict_mode = true;

        let old = self.ident_type;
        self.ident_type = IdentType::Ref;
        // c.decorators.visit_with(self);

        self.ident_type = IdentType::Ref;
        c.super_class(ast).visit_with(self, ast);

        self.ident_type = IdentType::Binding;
        // c.type_params.visit_with(self);

        self.ident_type = IdentType::Ref;
        // c.super_type_params.visit_with(self);

        self.ident_type = IdentType::Ref;
        // c.implements.visit_with(self);
        self.ident_type = old;

        c.body(ast).visit_with(self, ast);
        self.strict_mode = old_strict_mode;

        self.leave_node(c.node_id(), ast);
    }

    fn visit_class_decl(&mut self, n: ClassDecl, ast: &Ast) {
        self.enter_node(n.node_id(), ast);

        // if n.declare && !self.config.handle_types {
        //     return;
        // }
        self.modify(self.ast, n.ident(ast), DeclKind::Lexical);

        // n.class(ast).decorators.visit_with(self);

        // Create a child scope. The class name is only accessible within the class.

        self.with_child(ScopeKind::Fn, |child| {
            child.ident_type = IdentType::Ref;

            n.class(ast).visit_with(child, ast);
        });

        self.leave_node(n.node_id(), ast);
    }

    fn visit_class_expr(&mut self, n: ClassExpr, ast: &Ast) {
        self.enter_node(n.node_id(), ast);

        // Create a child scope. The class name is only accessible within the class.

        n.class(ast).super_class(ast).visit_with(self, ast);

        self.with_child(ScopeKind::Fn, |child| {
            child.ident_type = IdentType::Binding;
            n.ident(ast).visit_with(child, ast);
            child.ident_type = IdentType::Ref;

            n.class(ast).visit_with(child, ast);
        });

        self.leave_node(n.node_id(), ast);
    }

    fn visit_class_method(&mut self, m: ClassMethod, ast: &Ast) {
        self.enter_node(m.node_id(), ast);

        m.key(ast).visit_with(self, ast);

        // for p in m.function(ast).params(ast).iter() {
        //     let p = ast.get_node(p);
        //     p.decorators.visit_with(self);
        // }

        self.with_child(ScopeKind::Fn, |child| {
            m.function(ast).visit_with(child, ast)
        });

        self.leave_node(m.node_id(), ast);
    }

    fn visit_class_prop(&mut self, p: ClassProp, ast: &Ast) {
        self.enter_node(p.node_id(), ast);

        // p.decorators.visit_with(self);

        if let PropName::Computed(key) = p.key(ast) {
            let old = self.ident_type;
            self.ident_type = IdentType::Binding;
            key.expr(ast).visit_with(self, ast);
            self.ident_type = old;
        }

        let old = self.ident_type;
        self.ident_type = IdentType::Ref;
        p.value(ast).visit_with(self, ast);
        self.ident_type = old;

        // p.type_ann.visit_with(self);
        self.leave_node(p.node_id(), ast);
    }

    fn visit_constructor(&mut self, c: Constructor, ast: &Ast) {
        self.enter_node(c.node_id(), ast);

        // for p in c.params(ast).iter() {
        //     let p = ast.get_node(p);
        //     match p {
        //         ParamOrTsParamProp::TsParamProp(p) => {
        //             p.decorators.visit_with(self);
        //         }
        //         ParamOrTsParamProp::Param(p) => {
        //             p.decorators.visit_with(self);
        //         }
        //         #[cfg(swc_ast_unknown)]
        //         _ => (),
        //     }
        // }

        self.with_child(ScopeKind::Fn, |child| {
            let old = child.ident_type;
            child.ident_type = IdentType::Binding;
            {
                let params = c
                    .params(ast)
                    .iter()
                    .filter(|p| {
                        let p = ast.get_node_in_sub_range(*p);
                        match p {
                            // ParamOrTsParamProp::TsParamProp(_) => false,
                            ParamOrTsParamProp::Param(p) => !p.pat(ast).is_rest(),
                        }
                    })
                    .flat_map(|p| find_pat_ids(child.ast, ast.get_node_in_sub_range(p)));

                for id in params {
                    child.scopes[child.current]
                        .declared_symbols
                        .insert(id, DeclKind::Param);
                }
            }
            c.params(ast).visit_with(child, ast);
            child.ident_type = old;

            if let Some(body) = c.body(ast) {
                child.mark_block(body.node_id());
                body.visit_children_with(child, ast);
            }
        });

        self.leave_node(c.node_id(), ast);
    }

    fn visit_continue_stmt(&mut self, s: ContinueStmt, ast: &Ast) {
        self.enter_node(s.node_id(), ast);

        let old = self.ident_type;
        self.ident_type = IdentType::Label;
        s.label(ast).visit_with(self, ast);
        self.ident_type = old;

        self.leave_node(s.node_id(), ast);
    }

    fn visit_export_default_decl(&mut self, e: ExportDefaultDecl, ast: &Ast) {
        self.enter_node(e.node_id(), ast);

        // Treat default exported functions and classes as declarations
        // even though they are parsed as expressions.
        match e.decl(ast) {
            DefaultDecl::Fn(f) => {
                if f.ident(ast).is_some() {
                    self.with_child(ScopeKind::Fn, |child| {
                        f.function(ast).visit_with(child, ast);
                    });
                } else {
                    f.visit_with(self, ast)
                }
            }
            DefaultDecl::Class(c) => {
                // Skip class expression visitor to treat as a declaration.
                c.class(ast).visit_with(self, ast)
            }
            _ => e.visit_children_with(self, ast),
        }

        self.leave_node(e.node_id(), ast);
    }

    fn visit_export_default_expr(&mut self, node: ExportDefaultExpr, ast: &Ast) {
        self.enter_node(node.node_id(), ast);

        node.expr(ast).visit_with(self, ast);

        // if self.config.handle_types {
        //     if let Expr::Ident(i) = &mut *node.expr {
        //         self.try_resolving_as_type(i);
        //     }
        // }

        self.leave_node(node.node_id(), ast);
    }

    fn visit_export_named_specifier(&mut self, e: ExportNamedSpecifier, ast: &Ast) {
        self.enter_node(e.node_id(), ast);

        e.visit_children_with(self, ast);
        // if self.config.handle_types {
        //     match &mut e.orig {
        //         ModuleExportName::Ident(orig) => {
        //             self.try_resolving_as_type(orig);
        //         }
        //         ModuleExportName::Str(_) => {}
        //         #[cfg(swc_ast_unknown)]
        //         _ => {}
        //     }
        // }

        self.leave_node(e.node_id(), ast);
    }

    fn visit_export_specifier(&mut self, s: ExportSpecifier, ast: &Ast) {
        let old = self.ident_type;
        self.ident_type = IdentType::Ref;
        s.visit_children_with(self, ast);
        self.ident_type = old;
    }

    fn visit_expr(&mut self, expr: Expr, ast: &Ast) {
        // let _span = if LOG {
        //     Some(span!(Level::ERROR, "visit_expr").entered())
        // } else {
        //     None
        // };

        let old = self.ident_type;
        self.ident_type = IdentType::Ref;
        // maybe_grow_default(|| expr.visit_children_with(self));
        expr.visit_children_with(self, ast);
        self.ident_type = old;
    }

    fn visit_fn_decl(&mut self, node: FnDecl, ast: &Ast) {
        self.enter_node(node.node_id(), ast);

        // if node.declare && !self.config.handle_types {
        //     return;
        // }

        // We don't fold ident as Hoister handles this.
        // node.function(ast).decorators.visit_with(self, ast);

        self.with_child(ScopeKind::Fn, |child| {
            node.function(ast).visit_with(child, ast)
        });

        self.leave_node(node.node_id(), ast);
    }

    fn visit_fn_expr(&mut self, e: FnExpr, ast: &Ast) {
        self.enter_node(e.node_id(), ast);

        // e.function(ast).decorators.visit_with(self);

        if let Some(ident) = e.ident(ast) {
            self.with_child(ScopeKind::Fn, |child| {
                child.modify(child.ast, ident, DeclKind::Function);
                child.with_child(ScopeKind::Fn, |child| {
                    e.function(ast).visit_with(child, ast);
                });
            });
        } else {
            self.with_child(ScopeKind::Fn, |child| {
                e.function(ast).visit_with(child, ast);
            });
        }

        self.leave_node(e.node_id(), ast);
    }

    fn visit_for_in_stmt(&mut self, n: ForInStmt, ast: &Ast) {
        self.enter_node(n.node_id(), ast);

        self.with_child(ScopeKind::Block, |child| {
            n.left(ast).visit_with(child, ast);
            n.right(ast).visit_with(child, ast);

            child.visit_stmt_within_child_scope(ast, n.body(ast));
        });

        self.leave_node(n.node_id(), ast);
    }

    fn visit_for_of_stmt(&mut self, n: ForOfStmt, ast: &Ast) {
        self.enter_node(n.node_id(), ast);

        self.with_child(ScopeKind::Block, |child| {
            n.left(ast).visit_with(child, ast);
            n.right(ast).visit_with(child, ast);

            child.visit_stmt_within_child_scope(ast, n.body(ast));
        });

        self.leave_node(n.node_id(), ast);
    }

    fn visit_for_stmt(&mut self, n: ForStmt, ast: &Ast) {
        self.enter_node(n.node_id(), ast);

        self.with_child(ScopeKind::Block, |child| {
            child.ident_type = IdentType::Binding;
            n.init(ast).visit_with(child, ast);
            child.ident_type = IdentType::Ref;
            n.test(ast).visit_with(child, ast);
            child.ident_type = IdentType::Ref;
            n.update(ast).visit_with(child, ast);

            child.visit_stmt_within_child_scope(ast, n.body(ast));
        });

        self.leave_node(n.node_id(), ast);
    }

    fn visit_function(&mut self, f: Function, ast: &Ast) {
        self.enter_node(f.node_id(), ast);

        self.mark_block(f.node_id());
        // f.type_params.visit_with(self);

        self.ident_type = IdentType::Ref;
        // f.decorators.visit_with(self);

        {
            let params = f
                .params(ast)
                .iter()
                .filter(|p| !ast.get_node_in_sub_range(*p).pat(ast).is_rest())
                .flat_map(|p| find_pat_ids(self.ast, ast.get_node_in_sub_range(p)));

            for id in params {
                self.scopes[self.current]
                    .declared_symbols
                    .insert(id, DeclKind::Param);
            }
        }
        self.ident_type = IdentType::Binding;
        f.params(ast).visit_with(self, ast);

        // f.return_type.visit_with(self);

        self.ident_type = IdentType::Ref;
        if let Some(body) = f.body(ast) {
            self.mark_block(body.node_id());
            let old_strict_mode = self.strict_mode;
            if !self.strict_mode {
                self.strict_mode = body
                    .stmts(ast)
                    .first()
                    .map(|stmt| ast.get_node_in_sub_range(stmt).is_use_strict(ast))
                    .unwrap_or(false);
            }
            // Prevent creating new scope.
            body.visit_children_with(self, ast);
            self.strict_mode = old_strict_mode;
        }

        self.leave_node(f.node_id(), ast);
    }

    fn visit_getter_prop(&mut self, f: GetterProp, ast: &Ast) {
        self.enter_node(f.node_id(), ast);

        let old = self.ident_type;
        self.ident_type = IdentType::Ref;
        f.key(ast).visit_with(self, ast);
        self.ident_type = old;

        // f.type_ann.visit_with(self);

        f.body(ast).visit_with(self, ast);

        self.leave_node(f.node_id(), ast);
    }

    // fn visit_jsx_element_name(&mut self, node: &mut JSXElementName, ast: &Ast) {
    //     if let JSXElementName::Ident(i) = node {
    //         if i.as_ref().starts_with(|c: char| c.is_ascii_lowercase()) {
    //             if cfg!(debug_assertions) && LOG {
    //                 debug!("\t -> JSXElementName");
    //             }

    //             let ctxt = i.ctxt.apply_mark(self.config.unresolved_mark);

    //             if cfg!(debug_assertions) && LOG {
    //                 debug!("\t -> {:?}", ctxt);
    //             }

    //             i.ctxt = ctxt;

    //             return;
    //         }
    //     }

    //     node.visit_children_with(self);
    // }

    fn visit_ident(&mut self, i: Ident, ast: &Ast) {
        self.enter_node(i.node_id(), ast);

        if self.symbol_scopes.contains_key(&i.node_id()) {
            self.leave_node(i.node_id(), ast);
            return;
        }

        match self.ident_type {
            IdentType::Binding => self.modify(self.ast, i, self.decl_kind),
            IdentType::Ref => {
                // if cfg!(debug_assertions) && LOG {
                //     debug!("IdentRef (type = {}) {}{:?}", self.in_type, sym, ctxt);
                // }

                if let Some(scope_id) = self.mark_for_ref(ast.get_utf8(i.sym(ast))) {
                    // if cfg!(debug_assertions) && LOG {
                    //     debug!("\t -> {:?}", ctxt);
                    // }
                    self.symbol_scopes.insert(i.node_id(), scope_id);
                } else {
                    // if cfg!(debug_assertions) && LOG {
                    //     debug!("\t -> Unresolved");
                    // }

                    // if cfg!(debug_assertions) && LOG {
                    //     debug!("\t -> {:?}", ctxt);
                    // }

                    self.symbol_scopes.insert(i.node_id(), UNRESOLVED_SCOPE_ID);
                    // Support hoisting
                    self.modify(self.ast, i, self.decl_kind)
                }
            }
            // We currently does not touch labels
            IdentType::Label => {}
        }

        self.leave_node(i.node_id(), ast);
    }

    fn visit_import_decl(&mut self, n: ImportDecl, ast: &Ast) {
        self.enter_node(n.node_id(), ast);

        // Always resolve the import declaration identifiers even if it's type only.
        // We need to analyze these identifiers for type stripping purposes.
        self.ident_type = IdentType::Binding;
        let old_in_type = self.in_type;
        self.in_type = n.type_only(ast);
        n.visit_children_with(self, ast);
        self.in_type = old_in_type;

        self.leave_node(n.node_id(), ast);
    }

    fn visit_import_named_specifier(&mut self, s: ImportNamedSpecifier, ast: &Ast) {
        self.enter_node(s.node_id(), ast);

        let old = self.ident_type;
        self.ident_type = IdentType::Binding;
        s.local(ast).visit_with(self, ast);
        // if self.config.handle_types {
        //     self.current.declared_types.insert(s.local.sym.clone());
        // }
        self.ident_type = old;

        self.leave_node(s.node_id(), ast);
    }

    fn visit_import_specifier(&mut self, s: ImportSpecifier, ast: &Ast) {
        let old = self.ident_type;
        self.ident_type = IdentType::Binding;

        match s {
            ImportSpecifier::Named(named) => {
                if named.imported(ast).is_none() {
                    s.visit_children_with(self, ast);
                } else {
                    named.local(ast).visit_with(self, ast);
                }
            }
            ImportSpecifier::Namespace(..) | ImportSpecifier::Default(..) => {
                s.visit_children_with(self, ast)
            }
        }

        self.ident_type = old;
    }

    fn visit_key_value_pat_prop(&mut self, n: KeyValuePatProp, ast: &Ast) {
        self.enter_node(n.node_id(), ast);

        n.key(ast).visit_with(self, ast);
        n.value(ast).visit_with(self, ast);

        self.leave_node(n.node_id(), ast);
    }

    fn visit_labeled_stmt(&mut self, s: LabeledStmt, ast: &Ast) {
        self.enter_node(s.node_id(), ast);

        let old = self.ident_type;
        self.ident_type = IdentType::Label;
        s.label(ast).visit_with(self, ast);
        self.ident_type = old;

        s.body(ast).visit_with(self, ast);

        self.leave_node(s.node_id(), ast);
    }

    fn visit_method_prop(&mut self, m: MethodProp, ast: &Ast) {
        self.enter_node(m.node_id(), ast);

        m.key(ast).visit_with(self, ast);

        // Child folder
        self.with_child(ScopeKind::Fn, |child| {
            m.function(ast).visit_with(child, ast)
        });

        self.leave_node(m.node_id(), ast);
    }

    fn visit_module(&mut self, module: Module, ast: &Ast) {
        self.enter_node(module.node_id(), ast);

        self.strict_mode = true;
        self.is_module = true;
        module.visit_children_with(self, ast);

        self.leave_node(module.node_id(), ast);
    }

    fn visit_module_items(&mut self, stmts: TypedSubRange<ModuleItem>, ast: &Ast) {
        if !self.in_ts_module && self.scopes[self.current].kind != ScopeKind::Fn {
            return stmts.visit_children_with(self, ast);
        }

        // Phase 1: Handle hoisting
        {
            let mut hoister = Hoister {
                kind: self.decl_kind,
                resolver: self,
                in_block: false,
                in_catch_body: false,
                catch_param_decls: Default::default(),
                excluded_from_catch: Default::default(),
            };
            stmts.visit_with(&mut hoister, ast)
        }

        // Phase 2.
        stmts.visit_children_with(self, ast);
    }

    fn visit_named_export(&mut self, e: NamedExport, ast: &Ast) {
        self.enter_node(e.node_id(), ast);

        if e.src(ast).is_some() {
            self.leave_node(e.node_id(), ast);
            return;
        }

        e.visit_children_with(self, ast);

        self.leave_node(e.node_id(), ast);
    }

    fn visit_object_lit(&mut self, o: ObjectLit, ast: &Ast) {
        self.enter_node(o.node_id(), ast);

        self.with_child(ScopeKind::Block, |child| {
            o.visit_children_with(child, ast);
        });

        self.leave_node(o.node_id(), ast);
    }

    fn visit_param(&mut self, param: Param, ast: &Ast) {
        self.enter_node(param.node_id(), ast);

        self.ident_type = IdentType::Binding;
        param.visit_children_with(self, ast);

        self.leave_node(param.node_id(), ast);
    }

    fn visit_pat(&mut self, p: Pat, ast: &Ast) {
        p.visit_children_with(self, ast);
    }

    fn visit_private_method(&mut self, m: PrivateMethod, ast: &Ast) {
        self.enter_node(m.node_id(), ast);

        m.key(ast).visit_with(self, ast);

        {
            // Child folder

            self.with_child(ScopeKind::Fn, |child| {
                m.function(ast).visit_with(child, ast)
            });
        }

        self.leave_node(m.node_id(), ast);
    }

    fn visit_private_name(&mut self, node: PrivateName, ast: &Ast) {
        self.enter_node(node.node_id(), ast);

        self.leave_node(node.node_id(), ast);
    }

    fn visit_prop_name(&mut self, n: PropName, ast: &Ast) {
        if let PropName::Computed(c) = n {
            c.visit_with(self, ast);
        }
    }

    fn visit_rest_pat(&mut self, node: RestPat, ast: &Ast) {
        self.enter_node(node.node_id(), ast);

        node.arg(ast).visit_with(self, ast);
        // node.type_ann.visit_with(self);

        self.leave_node(node.node_id(), ast);
    }

    fn visit_script(&mut self, script: Script, ast: &Ast) {
        self.enter_node(script.node_id(), ast);

        self.strict_mode = script
            .body(ast)
            .first()
            .map(|stmt| ast.get_node_in_sub_range(stmt).is_use_strict(ast))
            .unwrap_or(false);
        script.visit_children_with(self, ast);

        self.leave_node(script.node_id(), ast);
    }

    fn visit_setter_prop(&mut self, n: SetterProp, ast: &Ast) {
        self.enter_node(n.node_id(), ast);

        n.key(ast).visit_with(self, ast);

        {
            self.with_child(ScopeKind::Fn, |child| {
                child.ident_type = IdentType::Binding;
                // n.this_param.visit_with(child);
                n.param(ast).visit_with(child, ast);
                n.body(ast).visit_with(child, ast);
            });
        };

        self.leave_node(n.node_id(), ast);
    }

    fn visit_stmts(&mut self, stmts: TypedSubRange<Stmt>, ast: &Ast) {
        // let _span = if LOG {
        //     Some(span!(Level::ERROR, "visit_stmts").entered())
        // } else {
        //     None
        // };

        // Phase 1: Handle hoisting
        {
            // let _span = if LOG {
            //     Some(span!(Level::ERROR, "hoist").entered())
            // } else {
            //     None
            // };

            let mut hoister = Hoister {
                kind: self.decl_kind,
                resolver: self,
                in_block: false,
                in_catch_body: false,
                catch_param_decls: Default::default(),
                excluded_from_catch: Default::default(),
            };
            stmts.visit_with(&mut hoister, ast)
        }

        // Phase 2.
        stmts.visit_children_with(self, ast);
    }

    fn visit_switch_case(&mut self, n: SwitchCase, ast: &Ast) {
        self.enter_node(n.node_id(), ast);

        n.cons(ast).visit_with(self, ast);
        n.test(ast).visit_with(self, ast);

        self.leave_node(n.node_id(), ast);
    }

    fn visit_switch_stmt(&mut self, s: SwitchStmt, ast: &Ast) {
        self.enter_node(s.node_id(), ast);

        s.discriminant(ast).visit_with(self, ast);

        self.with_child(ScopeKind::Block, |child| {
            s.cases(ast).visit_with(child, ast);
        });

        self.leave_node(s.node_id(), ast);
    }

    // fn visit_ts_as_expr(&mut self, n: TsAsExpr, ast: &Ast) {
    //     // if self.config.handle_types {
    //     //     n.type_ann.visit_with(self);
    //     // }

    //     n.expr(ast).visit_with(self, ast);
    // }

    // fn visit_ts_call_signature_decl(&mut self, n: &mut TsCallSignatureDecl, ast: &Ast) {
    //     if !self.config.handle_types {
    //         return;
    //     }

    //     self.with_child(ScopeKind::Fn, |child| {
    //         child.in_type = true;

    //         n.type_params.visit_with(child);
    //         n.params.visit_with(child);
    //         n.type_ann.visit_with(child);
    //     });
    // }

    // fn visit_ts_construct_signature_decl(
    //     &mut self,
    //     decl: &mut TsConstructSignatureDecl,
    //     ast: &Ast,
    // ) {
    //     if !self.config.handle_types {
    //         return;
    //     }

    //     // Child folder
    //     self.with_child(ScopeKind::Fn, |child| {
    //         child.in_type = true;

    //         // order is important
    //         decl.type_params.visit_with(child);
    //         decl.params.visit_with(child);
    //         decl.type_ann.visit_with(child);
    //     });
    // }

    // fn visit_ts_constructor_type(&mut self, ty: &mut TsConstructorType, ast: &Ast) {
    //     if !self.config.handle_types {
    //         return;
    //     }

    //     self.with_child(ScopeKind::Fn, |child| {
    //         child.in_type = true;

    //         ty.type_params.visit_with(child);
    //         ty.params.visit_with(child);
    //         ty.type_ann.visit_with(child);
    //     });
    // }

    // fn visit_ts_enum_decl(&mut self, decl: &mut TsEnumDecl, ast: &Ast) {
    //     if decl.declare && !self.config.handle_types {
    //         return;
    //     }
    //     self.modify(&mut decl.id, DeclKind::Lexical);

    //     self.with_child(ScopeKind::Block, |child| {
    //         // add the enum member names as declared symbols for this scope
    //         // Ex. `enum Foo { a, b = a }`
    //         let member_names = decl.members.iter().filter_map(|m| match &m.id {
    //             TsEnumMemberId::Ident(id) => Some((id.sym.clone(), DeclKind::Lexical)),
    //             TsEnumMemberId::Str(_) => None,
    //             #[cfg(swc_ast_unknown)]
    //             _ => None,
    //         });
    //         child.current.declared_symbols.extend(member_names);

    //         decl.members.visit_with(child);
    //     });
    // }

    // fn visit_ts_export_assignment(&mut self, node: &mut TsExportAssignment, ast: &Ast) {
    //     node.expr.visit_with(self);

    //     if self.config.handle_types {
    //         if let Some(i) = leftmost(&mut node.expr) {
    //             self.try_resolving_as_type(i);
    //         }
    //     }
    // }

    // fn visit_ts_expr_with_type_args(&mut self, n: &mut TsExprWithTypeArgs, ast: &Ast) {
    //     if self.config.handle_types {
    //         let old = self.in_type;
    //         self.in_type = true;
    //         n.visit_children_with(self);
    //         self.in_type = old;
    //     }
    // }

    // fn visit_ts_fn_type(&mut self, ty: &mut TsFnType, ast: &Ast) {
    //     if !self.config.handle_types {
    //         return;
    //     }

    //     self.with_child(ScopeKind::Fn, |child| {
    //         child.in_type = true;

    //         ty.type_params.visit_with(child);
    //         ty.params.visit_with(child);
    //         ty.type_ann.visit_with(child);
    //     });
    // }

    // fn visit_ts_getter_signature(&mut self, n: &mut TsGetterSignature, ast: &Ast) {
    //     if n.computed {
    //         n.key.visit_with(self);
    //     }

    //     n.type_ann.visit_with(self);
    // }

    // fn visit_ts_import_equals_decl(&mut self, n: &mut TsImportEqualsDecl, ast: &Ast) {
    //     self.modify(&mut n.id, DeclKind::Lexical);

    //     n.module_ref.visit_with(self);
    // }

    // fn visit_ts_import_type(&mut self, n: &mut TsImportType, ast: &Ast) {
    //     if !self.config.handle_types {
    //         return;
    //     }

    //     n.type_args.visit_with(self);
    // }

    // fn visit_ts_interface_decl(&mut self, n: &mut TsInterfaceDecl, ast: &Ast) {
    //     // always resolve the identifier for type stripping purposes
    //     let old_in_type = self.in_type;
    //     let old_ident_type = self.ident_type;

    //     self.in_type = true;
    //     self.ident_type = IdentType::Ref;

    //     self.modify(&mut n.id, DeclKind::Type);

    //     if !self.config.handle_types {
    //         self.in_type = old_in_type;
    //         self.ident_type = old_ident_type;
    //         return;
    //     }

    //     self.with_child(ScopeKind::Fn, |child| {
    //         child.in_type = true;

    //         n.type_params.visit_with(child);
    //         n.extends.visit_with(child);
    //         n.body.visit_with(child);
    //     });

    //     self.in_type = old_in_type;
    //     self.ident_type = old_ident_type;
    // }

    // fn visit_ts_mapped_type(&mut self, n: &mut TsMappedType, ast: &Ast) {
    //     if !self.config.handle_types {
    //         return;
    //     }

    //     self.ident_type = IdentType::Binding;
    //     n.type_param.visit_with(self);
    //     self.ident_type = IdentType::Ref;
    //     n.name_type.visit_with(self);

    //     self.ident_type = IdentType::Ref;
    //     n.type_ann.visit_with(self);
    // }

    // fn visit_ts_method_signature(&mut self, n: &mut TsMethodSignature, ast: &Ast) {
    //     if !self.config.handle_types {
    //         return;
    //     }

    //     self.with_child(ScopeKind::Fn, |child| {
    //         child.in_type = true;

    //         n.type_params.visit_with(child);
    //         if n.computed {
    //             n.key.visit_with(child);
    //         }
    //         n.params.visit_with(child);
    //         n.type_ann.visit_with(child);
    //     });
    // }

    // fn visit_ts_module_decl(&mut self, decl: &mut TsModuleDecl, ast: &Ast) {
    //     if decl.declare && !self.config.handle_types {
    //         return;
    //     }

    //     match &mut decl.id {
    //         TsModuleName::Ident(i) => {
    //             self.modify(i, DeclKind::Lexical);
    //         }
    //         TsModuleName::Str(_) => {}
    //         #[cfg(swc_ast_unknown)]
    //         _ => {}
    //     }

    //     self.with_child(ScopeKind::Block, |child| {
    //         child.in_ts_module = true;

    //         decl.body.visit_children_with(child);
    //     });
    // }

    // fn visit_ts_namespace_decl(&mut self, n: &mut TsNamespaceDecl, ast: &Ast) {
    //     if n.declare && !self.config.handle_types {
    //         return;
    //     }

    //     self.modify(&mut n.id, DeclKind::Lexical);

    //     n.body.visit_with(self);
    // }

    // fn visit_ts_param_prop_param(&mut self, n: &mut TsParamPropParam, ast: &Ast) {
    //     self.ident_type = IdentType::Binding;
    //     n.visit_children_with(self)
    // }

    // fn visit_ts_property_signature(&mut self, n: &mut TsPropertySignature, ast: &Ast) {
    //     if !self.config.handle_types {
    //         return;
    //     }

    //     if n.computed {
    //         n.key.visit_with(self);
    //     }

    //     self.with_child(ScopeKind::Fn, |child| {
    //         child.in_type = true;

    //         n.type_ann.visit_with(child);
    //     });
    // }

    // fn visit_ts_qualified_name(&mut self, n: &mut TsQualifiedName, ast: &Ast) {
    //     self.ident_type = IdentType::Ref;

    //     n.left.visit_with(self)
    // }

    // fn visit_ts_satisfies_expr(&mut self, n: &mut TsSatisfiesExpr, ast: &Ast) {
    //     if self.config.handle_types {
    //         n.type_ann.visit_with(self);
    //     }

    //     n.expr.visit_with(self);
    // }

    // fn visit_ts_setter_signature(&mut self, n: &mut TsSetterSignature, ast: &Ast) {
    //     if n.computed {
    //         n.key.visit_with(self);
    //     }

    //     n.param.visit_with(self);
    // }

    // fn visit_ts_tuple_element(&mut self, e: &mut TsTupleElement, ast: &Ast) {
    //     if !self.config.handle_types {
    //         return;
    //     }
    //     self.ident_type = IdentType::Ref;
    //     e.ty.visit_with(self);
    // }

    // fn visit_ts_type_alias_decl(&mut self, n: &mut TsTypeAliasDecl, ast: &Ast) {
    //     // always resolve the identifier for type stripping purposes
    //     let old_in_type = self.in_type;
    //     self.in_type = true;
    //     self.modify(&mut n.id, DeclKind::Type);

    //     if !self.config.handle_types {
    //         self.in_type = old_in_type;
    //         return;
    //     }

    //     self.with_child(ScopeKind::Fn, |child| {
    //         child.in_type = true;

    //         n.type_params.visit_with(child);
    //         n.type_ann.visit_with(child);
    //     });
    //     self.in_type = old_in_type;
    // }

    // fn visit_ts_type_assertion(&mut self, n: &mut TsTypeAssertion, ast: &Ast) {
    //     if self.config.handle_types {
    //         n.type_ann.visit_with(self);
    //     }

    //     n.expr.visit_with(self);
    // }

    // fn visit_ts_type_param(&mut self, param: &mut TsTypeParam, ast: &Ast) {
    //     if !self.config.handle_types {
    //         return;
    //     }
    //     param.name.visit_with(self);

    //     let ident_type = self.ident_type;
    //     param.default.visit_with(self);
    //     param.constraint.visit_with(self);
    //     self.ident_type = ident_type;
    // }

    // fn visit_ts_type_params(&mut self, params: &mut Vec<TsTypeParam>, ast: &Ast) {
    //     for param in params.iter_mut() {
    //         param.name.visit_with(self);
    //     }

    //     params.visit_children_with(self);
    // }

    fn visit_using_decl(&mut self, decl: UsingDecl, ast: &Ast) {
        self.enter_node(decl.node_id(), ast);

        let old_kind = self.decl_kind;
        self.decl_kind = DeclKind::Lexical;
        decl.decls(ast).visit_with(self, ast);
        self.decl_kind = old_kind;

        self.leave_node(decl.node_id(), ast);
    }

    fn visit_var_decl(&mut self, decl: VarDecl, ast: &Ast) {
        self.enter_node(decl.node_id(), ast);

        // if decl.declare && !self.config.handle_types {
        //     return;
        // }

        let old_kind = self.decl_kind;
        self.decl_kind = decl.kind(ast).into();
        decl.decls(ast).visit_with(self, ast);
        self.decl_kind = old_kind;

        self.leave_node(decl.node_id(), ast);
    }

    fn visit_var_declarator(&mut self, decl: VarDeclarator, ast: &Ast) {
        self.enter_node(decl.node_id(), ast);

        // order is important

        let old_type = self.ident_type;
        self.ident_type = IdentType::Binding;
        decl.name(ast).visit_with(self, ast);
        self.ident_type = old_type;

        decl.init(ast).visit_children_with(self, ast);

        self.leave_node(decl.node_id(), ast);
    }
}

fn leftmost(ast: &Ast, expr: Expr) -> Option<Ident> {
    match expr {
        Expr::Ident(i) => Some(i),
        Expr::Member(member) => leftmost(ast, member.obj(ast)),
        Expr::Paren(paren) => leftmost(ast, paren.expr(ast)),
        _ => None,
    }
}

/// The folder which handles var / function hoisting.
struct Hoister<'resolver, 'ast> {
    resolver: &'resolver mut Resolver<'ast>,
    kind: DeclKind,
    /// Hoister should not touch let / const in the block.
    in_block: bool,

    in_catch_body: bool,

    excluded_from_catch: FxHashSet<&'ast str>,
    catch_param_decls: FxHashSet<&'ast str>,
}

impl<'resolver, 'ast> Hoister<'resolver, 'ast> {
    fn add_pat_id(&mut self, ast: &'ast Ast, id: BindingIdent) {
        let id = id.id(ast);
        let sym = ast.get_utf8(id.sym(ast));
        if self.in_catch_body {
            // If we have a binding, it's different variable.
            if self.resolver.mark_for_ref_inner(sym, true).is_some()
                && self.catch_param_decls.contains(sym)
            {
                return;
            }

            self.excluded_from_catch.insert(sym);
        } else {
            // Behavior is different
            if self.catch_param_decls.contains(sym) && !self.excluded_from_catch.contains(&sym) {
                return;
            }
        }

        self.resolver.modify(ast, id, self.kind)
    }
}

impl<'resolver, 'ast> Visit for Hoister<'resolver, 'ast> {
    // noop_visit_type!();

    #[inline]
    fn visit_arrow_expr(&mut self, _: ArrowExpr, ast: &Ast) {}

    fn visit_assign_pat_prop(&mut self, node: AssignPatProp, ast: &Ast) {
        node.visit_children_with(self, ast);

        self.add_pat_id(self.resolver.ast, node.key(ast));
    }

    fn visit_block_stmt(&mut self, n: BlockStmt, ast: &Ast) {
        let old_in_block = self.in_block;
        self.in_block = true;
        n.visit_children_with(self, ast);
        self.in_block = old_in_block;
    }

    /// The code below prints "PASS"
    ///
    /// ```js
    ///
    ///      var a = "PASS";
    ///      try {
    ///          throw "FAIL1";
    ///          } catch (a) {
    ///          var a = "FAIL2";
    ///      }
    ///      console.log(a);
    /// ```
    ///
    /// While the code below does not throw **ReferenceError** for `b`
    ///
    /// ```js
    ///
    ///      b()
    ///      try {
    ///      } catch (b) {
    ///          var b;
    ///      }
    /// ```
    ///
    /// while the code below throws **ReferenceError**
    ///
    /// ```js
    ///
    ///      b()
    ///      try {
    ///      } catch (b) {
    ///      }
    /// ```
    #[inline]
    fn visit_catch_clause(&mut self, c: CatchClause, ast: &Ast) {
        let old_exclude = self.excluded_from_catch.clone();
        self.excluded_from_catch = Default::default();

        let old_in_catch_body = self.in_catch_body;

        let params = find_pat_ids(self.resolver.ast, c.param(ast));

        let orig = self.catch_param_decls.clone();

        self.catch_param_decls.extend(params);

        self.in_catch_body = true;
        c.body(ast).visit_with(self, ast);

        // let mut excluded = find_ids::<_, Id>(&c.body);

        // excluded.retain(|id| {
        //     // If we already have a declartion named a, `var a` in the catch body is
        //     // different var.

        //     self.resolver.mark_for_ref(&id.0).is_none()
        // });

        self.in_catch_body = false;
        c.param(ast).visit_with(self, ast);

        self.catch_param_decls = orig;

        self.in_catch_body = old_in_catch_body;
        self.excluded_from_catch = old_exclude;
    }

    fn visit_class_decl(&mut self, node: ClassDecl, ast: &Ast) {
        // if node.declare && !self.resolver.config.handle_types {
        //     return;
        // }
        if self.in_block {
            return;
        }
        self.resolver
            .modify(self.resolver.ast, node.ident(ast), DeclKind::Lexical);

        // if self.resolver.config.handle_types {
        //     self.resolver
        //         .current
        //         .declared_types
        //         .insert(node.ident.sym.clone());
        // }
    }

    #[inline]
    fn visit_constructor(&mut self, _: Constructor, ast: &Ast) {}

    #[inline]
    fn visit_decl(&mut self, decl: Decl, ast: &Ast) {
        decl.visit_children_with(self, ast);

        // if self.resolver.config.handle_types {
        //     match decl {
        //         Decl::TsInterface(i) => {
        //             if self.in_block {
        //                 return;
        //             }

        //             let old_in_type = self.resolver.in_type;
        //             self.resolver.in_type = true;
        //             self.resolver.modify(&mut i.id, DeclKind::Type);
        //             self.resolver.in_type = old_in_type;
        //         }

        //         Decl::TsTypeAlias(a) => {
        //             let old_in_type = self.resolver.in_type;
        //             self.resolver.in_type = true;
        //             self.resolver.modify(&mut a.id, DeclKind::Type);
        //             self.resolver.in_type = old_in_type;
        //         }

        //         Decl::TsEnum(e) => {
        //             if !self.in_block {
        //                 let old_in_type = self.resolver.in_type;
        //                 self.resolver.in_type = false;
        //                 self.resolver.modify(&mut e.id, DeclKind::Lexical);
        //                 self.resolver.in_type = old_in_type;
        //             }
        //         }

        //         Decl::TsModule(v)
        //             if matches!(
        //                 &**v,
        //                 TsModuleDecl {
        //                     global: false,
        //                     id: TsModuleName::Ident(_),
        //                     ..
        //                 },
        //             ) =>
        //         {
        //             if !self.in_block {
        //                 let old_in_type = self.resolver.in_type;
        //                 self.resolver.in_type = false;
        //                 let id = v.id.as_mut_ident().unwrap();
        //                 self.resolver.modify(id, DeclKind::Lexical);
        //                 self.resolver.in_type = old_in_type;
        //             }
        //         }
        //         _ => {}
        //     }
        // }
    }

    fn visit_export_default_decl(&mut self, node: ExportDefaultDecl, ast: &Ast) {
        // Treat default exported functions and classes as declarations
        // even though they are parsed as expressions.
        match node.decl(ast) {
            DefaultDecl::Fn(f) => {
                if let Some(id) = f.ident(ast) {
                    self.resolver.modify(self.resolver.ast, id, DeclKind::Var);
                }

                f.visit_with(self, ast)
            }
            DefaultDecl::Class(c) => {
                if let Some(id) = c.ident(ast) {
                    self.resolver
                        .modify(self.resolver.ast, id, DeclKind::Lexical);
                }

                c.visit_with(self, ast)
            }
            _ => {
                node.visit_children_with(self, ast);
            }
        }
    }

    #[inline]
    fn visit_expr(&mut self, _: Expr, ast: &Ast) {}

    fn visit_fn_decl(&mut self, node: FnDecl, ast: &Ast) {
        // if node.declare && !self.resolver.config.handle_types {
        //     return;
        // }

        if self
            .catch_param_decls
            .contains(self.resolver.ast.get_utf8(node.ident(ast).sym(ast)))
        {
            return;
        }

        if self.in_block {
            // function declaration is block scoped in strict mode
            if self.resolver.strict_mode {
                return;
            }
            // If we are in nested block, and variable named `foo` is lexically declared or
            // a parameter, we should ignore function foo while handling upper scopes.
            if let Some(DeclKind::Lexical | DeclKind::Param) = self.resolver.is_declared(
                ast.get_utf8(node.ident(ast).sym(ast)),
                self.resolver.current,
            ) {
                return;
            }
        }

        self.resolver
            .modify(self.resolver.ast, node.ident(ast), DeclKind::Function);
    }

    #[inline]
    fn visit_function(&mut self, _: Function, ast: &Ast) {}

    fn visit_import_default_specifier(&mut self, n: ImportDefaultSpecifier, ast: &Ast) {
        n.visit_children_with(self, ast);

        self.resolver
            .modify(self.resolver.ast, n.local(ast), DeclKind::Lexical);

        // if self.resolver.config.handle_types {
        //     self.resolver
        //         .current
        //         .declared_types
        //         .insert(n.local.sym.clone());
        // }
    }

    fn visit_import_named_specifier(&mut self, n: ImportNamedSpecifier, ast: &Ast) {
        n.visit_children_with(self, ast);

        self.resolver
            .modify(self.resolver.ast, n.local(ast), DeclKind::Lexical);

        // if self.resolver.config.handle_types {
        //     self.resolver
        //         .current
        //         .declared_types
        //         .insert(n.local.sym.clone());
        // }
    }

    fn visit_import_star_as_specifier(&mut self, n: ImportStarAsSpecifier, ast: &Ast) {
        n.visit_children_with(self, ast);

        self.resolver
            .modify(self.resolver.ast, n.local(ast), DeclKind::Lexical);

        // if self.resolver.config.handle_types {
        //     self.resolver
        //         .current
        //         .declared_types
        //         .insert(n.local.sym.clone());
        // }
    }

    #[inline]
    fn visit_param(&mut self, _: Param, ast: &Ast) {}

    fn visit_pat(&mut self, node: Pat, ast: &Ast) {
        match node {
            Pat::Ident(i) => {
                self.add_pat_id(self.resolver.ast, i);
            }

            _ => node.visit_children_with(self, ast),
        }
    }

    #[inline]
    fn visit_assign_target(&mut self, _: AssignTarget, ast: &Ast) {}

    #[inline]
    fn visit_setter_prop(&mut self, _: SetterProp, ast: &Ast) {}

    fn visit_switch_stmt(&mut self, s: SwitchStmt, ast: &Ast) {
        s.discriminant(ast).visit_with(self, ast);

        let old_in_block = self.in_block;
        self.in_block = true;
        s.cases(ast).visit_with(self, ast);
        self.in_block = old_in_block;
    }

    #[inline]
    fn visit_tagged_tpl(&mut self, _: TaggedTpl, ast: &Ast) {}

    #[inline]
    fn visit_tpl(&mut self, _: Tpl, ast: &Ast) {}

    // #[inline]
    // fn visit_ts_module_block(&mut self, _: TsModuleBlock, ast: &Ast) {}

    #[inline]
    fn visit_using_decl(&mut self, node: UsingDecl, ast: &Ast) {
        if self.in_block {
            return;
        }

        let old_kind = self.kind;
        self.kind = DeclKind::Lexical;
        node.visit_children_with(self, ast);
        self.kind = old_kind;
    }

    fn visit_var_decl(&mut self, node: VarDecl, ast: &Ast) {
        // if node.declare && !self.resolver.config.handle_types {
        //     return;
        // }

        if self.in_block {
            match node.kind(ast) {
                VarDeclKind::Const | VarDeclKind::Let => return,
                _ => {}
            }
        }

        let old_kind = self.kind;
        self.kind = node.kind(ast).into();

        node.visit_children_with(self, ast);

        self.kind = old_kind;
    }

    fn visit_var_decl_or_expr(&mut self, n: VarDeclOrExpr, ast: &Ast) {
        if let VarDeclOrExpr::VarDecl(v) = n {
            let kind = v.kind(ast);
            if kind == VarDeclKind::Let || kind == VarDeclKind::Const {
                return;
            }
        }

        n.visit_children_with(self, ast);
    }

    fn visit_for_head(&mut self, n: ForHead, ast: &Ast) {
        if let ForHead::VarDecl(v) = n {
            let kind = v.kind(ast);
            if kind == VarDeclKind::Let || kind == VarDeclKind::Const {
                return;
            }
        }

        // Hoister should not handle lhs of for in statement below
        //
        // const b = [];
        // {
        //   let a;
        //   for (a in b) {
        //     console.log(a);
        //   }
        // }
        if let ForHead::Pat(_) = n {
            return;
        }

        n.visit_children_with(self, ast);
    }

    #[inline]
    fn visit_var_declarator(&mut self, node: VarDeclarator, ast: &Ast) {
        node.name(ast).visit_with(self, ast);
    }

    /// should visit var decls first, cause var decl may appear behind the
    /// usage. this can deal with code below:
    /// ```js
    /// try {} catch (Ic) {
    ///   throw Ic;
    /// }
    /// var Ic;
    /// ```
    /// the `Ic` defined by catch param and the `Ic` defined by `var Ic` are
    /// different variables.
    /// If we deal with the `var Ic` first, we can know
    /// that there is already an global declaration of Ic when deal with the try
    /// block.
    fn visit_module_items(&mut self, items: TypedSubRange<ModuleItem>, ast: &Ast) {
        items.visit_children_with(self, ast);
    }

    /// see docs for `self.visit_module_items`
    fn visit_stmts(&mut self, stmts: TypedSubRange<Stmt>, ast: &Ast) {
        let others = stmts
            .iter()
            .filter_map(|item| {
                let item = ast.get_node_in_sub_range(item);
                match item {
                    Stmt::Decl(Decl::Var(..)) => {
                        item.visit_with(self, ast);
                        None
                    }
                    Stmt::Decl(Decl::Fn(..)) => {
                        item.visit_with(self, ast);
                        None
                    }
                    _ => Some(item),
                }
            })
            .collect::<Vec<_>>();

        for other_stmt in others {
            other_stmt.visit_with(self, ast);
        }
    }
}
