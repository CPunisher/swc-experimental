#![allow(unused)]

/// crates/swc_ecma_transforms_base/src/resolver/mod.rs
use oxc_index::IndexVec;
use rustc_hash::FxHashMap;
use swc_experimental_ecma_ast::*;
use swc_experimental_ecma_visit::{Visit, VisitWith};

use crate::{
    node::NodeInfo,
    reference::{Reference, ReferenceId},
    scope::{Scope, ScopeFlags, ScopeId},
    symbol::{Symbol, SymbolId},
};

mod legacy;
mod node;
mod reference;
mod scope;
mod symbol;

pub use legacy::resolver;

pub struct Semantic {
    nodes: IndexVec<NodeId, NodeInfo>,
}

impl Semantic {
    pub fn node_scope(&self, id: NodeId) -> ScopeId {
        self.nodes[id].scope_id()
    }
}

pub struct SemanticBuilder {
    scopes: IndexVec<ScopeId, Scope>,
    current_scope_id: ScopeId,
    /// Map node ids to symbol ids
    nodes: FxHashMap<NodeId, SymbolId>,
    /// Symbols are created by declarations (definition)
    symbols: IndexVec<SymbolId, Symbol>,
    /// References are created by uses
    references: IndexVec<ReferenceId, Reference>,
}

impl SemanticBuilder {
    fn enter_scope(&mut self, flags: ScopeFlags, node_id: NodeId) -> ScopeId {
        let parent_scope_id = self.current_scope_id;
        let parent_flags = self.scopes[parent_scope_id].flags();

        let flags = flags | (parent_flags & ScopeFlags::StrictMode);
        self.current_scope_id = self
            .scopes
            .push(Scope::new(Some(parent_scope_id), flags, node_id));
        self.current_scope_id
    }

    fn leave_scope(&mut self) {
        let parent_id = self.scopes[self.current_scope_id].parent();
        if let Some(parent_id) = parent_id {
            self.current_scope_id = parent_id
        }
    }

    fn create_symbol(&mut self) {}

    fn create_reference(&mut self) {}
}

impl Visit for SemanticBuilder {
    fn visit_module(&mut self, node: Module, ast: &Ast) {
        self.enter_scope(ScopeFlags::Fn, node.node_id());
        node.visit_children_with(self, ast);
    }

    fn visit_script(&mut self, node: Script, ast: &Ast) {
        let strict = node
            .body(ast)
            .iter()
            .next()
            .map(|stmt| ast.get_node_in_sub_range(stmt).is_use_strict(ast))
            .unwrap_or(false);

        let mut flags = ScopeFlags::Fn;
        if strict {
            flags |= ScopeFlags::StrictMode;
        }

        self.enter_scope(flags, node.node_id());
        node.visit_children_with(self, ast);
    }

    fn visit_function(&mut self, node: Function, ast: &Ast) {
        self.enter_scope(ScopeFlags::Fn, node.node_id());

        self.leave_scope();
    }

    fn visit_arrow_expr(&mut self, node: ArrowExpr, ast: &Ast) {
        self.enter_scope(ScopeFlags::Fn, node.node_id());

        self.leave_scope();
    }

    fn visit_block_stmt(&mut self, node: BlockStmt, ast: &Ast) {
        self.enter_scope(ScopeFlags::Block, node.node_id());

        self.leave_scope();
    }

    fn visit_catch_clause(&mut self, node: CatchClause, ast: &Ast) {
        self.enter_scope(ScopeFlags::Fn, node.node_id());

        self.leave_scope();
    }

    fn visit_class(&mut self, node: Class, ast: &Ast) {
        self.enter_scope(ScopeFlags::Block, node.node_id());

        self.leave_scope();
    }

    fn visit_for_stmt(&mut self, node: ForStmt, ast: &Ast) {
        self.enter_scope(ScopeFlags::Block, node.node_id());

        self.leave_scope();
    }

    fn visit_for_in_stmt(&mut self, node: ForInStmt, ast: &Ast) {
        self.enter_scope(ScopeFlags::Block, node.node_id());

        self.leave_scope();
    }

    fn visit_for_of_stmt(&mut self, node: ForOfStmt, ast: &Ast) {
        self.enter_scope(ScopeFlags::Block, node.node_id());

        self.leave_scope();
    }

    // declarations
    fn visit_decl(&mut self, node: Decl, ast: &Ast) {
        match node {
            Decl::Class(class_decl) => todo!(),
            Decl::Fn(fn_decl) => todo!(),
            Decl::Var(var_decl) => todo!(),
            Decl::Using(using_decl) => todo!(),
        }
    }
}
