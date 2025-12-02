#![allow(unused)]
use crate::{ast::*, node_id::*};
use crate::{Ast, CloneIn, NodeKind};
impl CloneIn for Program {
    type Cloned = Program;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Module(it) => Self::Module(it.clone_in(ast)),
            Self::Script(it) => Self::Script(it.clone_in(ast)),
        }
    }
}
impl CloneIn for Module {
    type Cloned = Module;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Script {
    type Cloned = Script;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ModuleItem {
    type Cloned = ModuleItem;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::ModuleDecl(it) => Self::ModuleDecl(it.clone_in(ast)),
            Self::Stmt(it) => Self::Stmt(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ModuleDecl {
    type Cloned = ModuleDecl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Import(it) => Self::Import(it.clone_in(ast)),
            Self::ExportDecl(it) => Self::ExportDecl(it.clone_in(ast)),
            Self::ExportNamed(it) => Self::ExportNamed(it.clone_in(ast)),
            Self::ExportDefaultDecl(it) => Self::ExportDefaultDecl(it.clone_in(ast)),
            Self::ExportDefaultExpr(it) => Self::ExportDefaultExpr(it.clone_in(ast)),
            Self::ExportAll(it) => Self::ExportAll(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ImportDecl {
    type Cloned = ImportDecl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ImportSpecifier {
    type Cloned = ImportSpecifier;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Named(it) => Self::Named(it.clone_in(ast)),
            Self::Default(it) => Self::Default(it.clone_in(ast)),
            Self::Namespace(it) => Self::Namespace(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ImportNamedSpecifier {
    type Cloned = ImportNamedSpecifier;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ImportDefaultSpecifier {
    type Cloned = ImportDefaultSpecifier;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ImportStarAsSpecifier {
    type Cloned = ImportStarAsSpecifier;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ExportDecl {
    type Cloned = ExportDecl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for NamedExport {
    type Cloned = NamedExport;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ExportSpecifier {
    type Cloned = ExportSpecifier;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Namespace(it) => Self::Namespace(it.clone_in(ast)),
            Self::Default(it) => Self::Default(it.clone_in(ast)),
            Self::Named(it) => Self::Named(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ExportNamespaceSpecifier {
    type Cloned = ExportNamespaceSpecifier;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ModuleExportName {
    type Cloned = ModuleExportName;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
            Self::Str(it) => Self::Str(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ExportDefaultSpecifier {
    type Cloned = ExportDefaultSpecifier;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ExportNamedSpecifier {
    type Cloned = ExportNamedSpecifier;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ExportDefaultDecl {
    type Cloned = ExportDefaultDecl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for DefaultDecl {
    type Cloned = DefaultDecl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Class(it) => Self::Class(it.clone_in(ast)),
            Self::Fn(it) => Self::Fn(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ExportDefaultExpr {
    type Cloned = ExportDefaultExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ExportAll {
    type Cloned = ExportAll;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for BlockStmt {
    type Cloned = BlockStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Stmt {
    type Cloned = Stmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Block(it) => Self::Block(it.clone_in(ast)),
            Self::Empty(it) => Self::Empty(it.clone_in(ast)),
            Self::Debugger(it) => Self::Debugger(it.clone_in(ast)),
            Self::With(it) => Self::With(it.clone_in(ast)),
            Self::Return(it) => Self::Return(it.clone_in(ast)),
            Self::Labeled(it) => Self::Labeled(it.clone_in(ast)),
            Self::Break(it) => Self::Break(it.clone_in(ast)),
            Self::Continue(it) => Self::Continue(it.clone_in(ast)),
            Self::If(it) => Self::If(it.clone_in(ast)),
            Self::Switch(it) => Self::Switch(it.clone_in(ast)),
            Self::Throw(it) => Self::Throw(it.clone_in(ast)),
            Self::Try(it) => Self::Try(it.clone_in(ast)),
            Self::While(it) => Self::While(it.clone_in(ast)),
            Self::DoWhile(it) => Self::DoWhile(it.clone_in(ast)),
            Self::For(it) => Self::For(it.clone_in(ast)),
            Self::ForIn(it) => Self::ForIn(it.clone_in(ast)),
            Self::ForOf(it) => Self::ForOf(it.clone_in(ast)),
            Self::Decl(it) => Self::Decl(it.clone_in(ast)),
            Self::Expr(it) => Self::Expr(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ExprStmt {
    type Cloned = ExprStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for EmptyStmt {
    type Cloned = EmptyStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for DebuggerStmt {
    type Cloned = DebuggerStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for WithStmt {
    type Cloned = WithStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ReturnStmt {
    type Cloned = ReturnStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for LabeledStmt {
    type Cloned = LabeledStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for BreakStmt {
    type Cloned = BreakStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ContinueStmt {
    type Cloned = ContinueStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for IfStmt {
    type Cloned = IfStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for SwitchStmt {
    type Cloned = SwitchStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ThrowStmt {
    type Cloned = ThrowStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for TryStmt {
    type Cloned = TryStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for WhileStmt {
    type Cloned = WhileStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for DoWhileStmt {
    type Cloned = DoWhileStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ForStmt {
    type Cloned = ForStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ForInStmt {
    type Cloned = ForInStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ForOfStmt {
    type Cloned = ForOfStmt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for SwitchCase {
    type Cloned = SwitchCase;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for CatchClause {
    type Cloned = CatchClause;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ForHead {
    type Cloned = ForHead;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::VarDecl(it) => Self::VarDecl(it.clone_in(ast)),
            Self::UsingDecl(it) => Self::UsingDecl(it.clone_in(ast)),
            Self::Pat(it) => Self::Pat(it.clone_in(ast)),
        }
    }
}
impl CloneIn for VarDeclOrExpr {
    type Cloned = VarDeclOrExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::VarDecl(it) => Self::VarDecl(it.clone_in(ast)),
            Self::Expr(it) => Self::Expr(it.clone_in(ast)),
        }
    }
}
impl CloneIn for Decl {
    type Cloned = Decl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Class(it) => Self::Class(it.clone_in(ast)),
            Self::Fn(it) => Self::Fn(it.clone_in(ast)),
            Self::Var(it) => Self::Var(it.clone_in(ast)),
            Self::Using(it) => Self::Using(it.clone_in(ast)),
        }
    }
}
impl CloneIn for FnDecl {
    type Cloned = FnDecl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ClassDecl {
    type Cloned = ClassDecl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for VarDecl {
    type Cloned = VarDecl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for VarDeclarator {
    type Cloned = VarDeclarator;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for UsingDecl {
    type Cloned = UsingDecl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Expr {
    type Cloned = Expr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::This(it) => Self::This(it.clone_in(ast)),
            Self::Array(it) => Self::Array(it.clone_in(ast)),
            Self::Object(it) => Self::Object(it.clone_in(ast)),
            Self::Fn(it) => Self::Fn(it.clone_in(ast)),
            Self::Unary(it) => Self::Unary(it.clone_in(ast)),
            Self::Update(it) => Self::Update(it.clone_in(ast)),
            Self::Bin(it) => Self::Bin(it.clone_in(ast)),
            Self::Assign(it) => Self::Assign(it.clone_in(ast)),
            Self::Member(it) => Self::Member(it.clone_in(ast)),
            Self::SuperProp(it) => Self::SuperProp(it.clone_in(ast)),
            Self::Cond(it) => Self::Cond(it.clone_in(ast)),
            Self::Call(it) => Self::Call(it.clone_in(ast)),
            Self::New(it) => Self::New(it.clone_in(ast)),
            Self::Seq(it) => Self::Seq(it.clone_in(ast)),
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
            Self::Lit(it) => Self::Lit(it.clone_in(ast)),
            Self::Tpl(it) => Self::Tpl(it.clone_in(ast)),
            Self::TaggedTpl(it) => Self::TaggedTpl(it.clone_in(ast)),
            Self::Arrow(it) => Self::Arrow(it.clone_in(ast)),
            Self::Class(it) => Self::Class(it.clone_in(ast)),
            Self::Yield(it) => Self::Yield(it.clone_in(ast)),
            Self::MetaProp(it) => Self::MetaProp(it.clone_in(ast)),
            Self::Await(it) => Self::Await(it.clone_in(ast)),
            Self::Paren(it) => Self::Paren(it.clone_in(ast)),
            Self::PrivateName(it) => Self::PrivateName(it.clone_in(ast)),
            Self::OptChain(it) => Self::OptChain(it.clone_in(ast)),
            Self::Invalid(it) => Self::Invalid(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ThisExpr {
    type Cloned = ThisExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ArrayLit {
    type Cloned = ArrayLit;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ObjectLit {
    type Cloned = ObjectLit;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for PropOrSpread {
    type Cloned = PropOrSpread;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::SpreadElement(it) => Self::SpreadElement(it.clone_in(ast)),
            Self::Prop(it) => Self::Prop(it.clone_in(ast)),
        }
    }
}
impl CloneIn for SpreadElement {
    type Cloned = SpreadElement;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for UnaryExpr {
    type Cloned = UnaryExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for UpdateExpr {
    type Cloned = UpdateExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for BinExpr {
    type Cloned = BinExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for FnExpr {
    type Cloned = FnExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ClassExpr {
    type Cloned = ClassExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for AssignExpr {
    type Cloned = AssignExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for MemberExpr {
    type Cloned = MemberExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for MemberProp {
    type Cloned = MemberProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
            Self::PrivateName(it) => Self::PrivateName(it.clone_in(ast)),
            Self::Computed(it) => Self::Computed(it.clone_in(ast)),
        }
    }
}
impl CloneIn for SuperPropExpr {
    type Cloned = SuperPropExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for SuperProp {
    type Cloned = SuperProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
            Self::Computed(it) => Self::Computed(it.clone_in(ast)),
        }
    }
}
impl CloneIn for CondExpr {
    type Cloned = CondExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for CallExpr {
    type Cloned = CallExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for NewExpr {
    type Cloned = NewExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for SeqExpr {
    type Cloned = SeqExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ArrowExpr {
    type Cloned = ArrowExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for YieldExpr {
    type Cloned = YieldExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for MetaPropExpr {
    type Cloned = MetaPropExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for AwaitExpr {
    type Cloned = AwaitExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Tpl {
    type Cloned = Tpl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for TaggedTpl {
    type Cloned = TaggedTpl;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for TplElement {
    type Cloned = TplElement;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ParenExpr {
    type Cloned = ParenExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Callee {
    type Cloned = Callee;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Super(it) => Self::Super(it.clone_in(ast)),
            Self::Import(it) => Self::Import(it.clone_in(ast)),
            Self::Expr(it) => Self::Expr(it.clone_in(ast)),
        }
    }
}
impl CloneIn for Super {
    type Cloned = Super;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Import {
    type Cloned = Import;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ExprOrSpread {
    type Cloned = ExprOrSpread;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for SpreadDot3Token {
    type Cloned = SpreadDot3Token;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for BlockStmtOrExpr {
    type Cloned = BlockStmtOrExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::BlockStmt(it) => Self::BlockStmt(it.clone_in(ast)),
            Self::Expr(it) => Self::Expr(it.clone_in(ast)),
        }
    }
}
impl CloneIn for AssignTarget {
    type Cloned = AssignTarget;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Simple(it) => Self::Simple(it.clone_in(ast)),
            Self::Pat(it) => Self::Pat(it.clone_in(ast)),
        }
    }
}
impl CloneIn for AssignTargetPat {
    type Cloned = AssignTargetPat;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Array(it) => Self::Array(it.clone_in(ast)),
            Self::Object(it) => Self::Object(it.clone_in(ast)),
            Self::Invalid(it) => Self::Invalid(it.clone_in(ast)),
        }
    }
}
impl CloneIn for SimpleAssignTarget {
    type Cloned = SimpleAssignTarget;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
            Self::Member(it) => Self::Member(it.clone_in(ast)),
            Self::SuperProp(it) => Self::SuperProp(it.clone_in(ast)),
            Self::Paren(it) => Self::Paren(it.clone_in(ast)),
            Self::OptChain(it) => Self::OptChain(it.clone_in(ast)),
            Self::Invalid(it) => Self::Invalid(it.clone_in(ast)),
        }
    }
}
impl CloneIn for OptChainExpr {
    type Cloned = OptChainExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for OptChainBase {
    type Cloned = OptChainBase;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Member(it) => Self::Member(it.clone_in(ast)),
            Self::Call(it) => Self::Call(it.clone_in(ast)),
        }
    }
}
impl CloneIn for OptCall {
    type Cloned = OptCall;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Invalid {
    type Cloned = Invalid;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Function {
    type Cloned = Function;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Param {
    type Cloned = Param;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ParamOrTsParamProp {
    type Cloned = ParamOrTsParamProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Param(it) => Self::Param(it.clone_in(ast)),
        }
    }
}
impl CloneIn for Class {
    type Cloned = Class;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ClassMember {
    type Cloned = ClassMember;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Constructor(it) => Self::Constructor(it.clone_in(ast)),
            Self::Method(it) => Self::Method(it.clone_in(ast)),
            Self::PrivateMethod(it) => Self::PrivateMethod(it.clone_in(ast)),
            Self::ClassProp(it) => Self::ClassProp(it.clone_in(ast)),
            Self::PrivateProp(it) => Self::PrivateProp(it.clone_in(ast)),
            Self::Empty(it) => Self::Empty(it.clone_in(ast)),
            Self::StaticBlock(it) => Self::StaticBlock(it.clone_in(ast)),
            Self::AutoAccessor(it) => Self::AutoAccessor(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ClassProp {
    type Cloned = ClassProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for PrivateProp {
    type Cloned = PrivateProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ClassMethod {
    type Cloned = ClassMethod;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for PrivateMethod {
    type Cloned = PrivateMethod;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Constructor {
    type Cloned = Constructor;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Decorator {
    type Cloned = Decorator;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for StaticBlock {
    type Cloned = StaticBlock;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Key {
    type Cloned = Key;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Private(it) => Self::Private(it.clone_in(ast)),
            Self::Public(it) => Self::Public(it.clone_in(ast)),
        }
    }
}
impl CloneIn for AutoAccessor {
    type Cloned = AutoAccessor;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Prop {
    type Cloned = Prop;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Shorthand(it) => Self::Shorthand(it.clone_in(ast)),
            Self::KeyValue(it) => Self::KeyValue(it.clone_in(ast)),
            Self::Assign(it) => Self::Assign(it.clone_in(ast)),
            Self::Getter(it) => Self::Getter(it.clone_in(ast)),
            Self::Setter(it) => Self::Setter(it.clone_in(ast)),
            Self::Method(it) => Self::Method(it.clone_in(ast)),
        }
    }
}
impl CloneIn for KeyValueProp {
    type Cloned = KeyValueProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for AssignProp {
    type Cloned = AssignProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for GetterProp {
    type Cloned = GetterProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for SetterProp {
    type Cloned = SetterProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for MethodProp {
    type Cloned = MethodProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for PropName {
    type Cloned = PropName;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
            Self::Str(it) => Self::Str(it.clone_in(ast)),
            Self::Num(it) => Self::Num(it.clone_in(ast)),
            Self::Computed(it) => Self::Computed(it.clone_in(ast)),
            Self::BigInt(it) => Self::BigInt(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ComputedPropName {
    type Cloned = ComputedPropName;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Pat {
    type Cloned = Pat;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
            Self::Array(it) => Self::Array(it.clone_in(ast)),
            Self::Rest(it) => Self::Rest(it.clone_in(ast)),
            Self::Object(it) => Self::Object(it.clone_in(ast)),
            Self::Assign(it) => Self::Assign(it.clone_in(ast)),
            Self::Invalid(it) => Self::Invalid(it.clone_in(ast)),
            Self::Expr(it) => Self::Expr(it.clone_in(ast)),
        }
    }
}
impl CloneIn for ArrayPat {
    type Cloned = ArrayPat;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ObjectPat {
    type Cloned = ObjectPat;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for AssignPat {
    type Cloned = AssignPat;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for RestPat {
    type Cloned = RestPat;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for ObjectPatProp {
    type Cloned = ObjectPatProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::KeyValue(it) => Self::KeyValue(it.clone_in(ast)),
            Self::Assign(it) => Self::Assign(it.clone_in(ast)),
            Self::Rest(it) => Self::Rest(it.clone_in(ast)),
        }
    }
}
impl CloneIn for KeyValuePatProp {
    type Cloned = KeyValuePatProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for AssignPatProp {
    type Cloned = AssignPatProp;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Ident {
    type Cloned = Ident;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for IdentName {
    type Cloned = IdentName;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for PrivateName {
    type Cloned = PrivateName;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for BindingIdent {
    type Cloned = BindingIdent;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Lit {
    type Cloned = Lit;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Str(it) => Self::Str(it.clone_in(ast)),
            Self::Bool(it) => Self::Bool(it.clone_in(ast)),
            Self::Null(it) => Self::Null(it.clone_in(ast)),
            Self::Num(it) => Self::Num(it.clone_in(ast)),
            Self::BigInt(it) => Self::BigInt(it.clone_in(ast)),
            Self::Regex(it) => Self::Regex(it.clone_in(ast)),
        }
    }
}
impl CloneIn for Str {
    type Cloned = Str;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Bool {
    type Cloned = Bool;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Null {
    type Cloned = Null;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Number {
    type Cloned = Number;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for BigInt {
    type Cloned = BigInt;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for Regex {
    type Cloned = Regex;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXObject {
    type Cloned = JSXObject;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::JSXMemberExpr(it) => Self::JSXMemberExpr(it.clone_in(ast)),
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
        }
    }
}
impl CloneIn for JSXMemberExpr {
    type Cloned = JSXMemberExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXNamespacedName {
    type Cloned = JSXNamespacedName;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXEmptyExpr {
    type Cloned = JSXEmptyExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXExprContainer {
    type Cloned = JSXExprContainer;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXExpr {
    type Cloned = JSXExpr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::JSXEmptyExpr(it) => Self::JSXEmptyExpr(it.clone_in(ast)),
            Self::Expr(it) => Self::Expr(it.clone_in(ast)),
        }
    }
}
impl CloneIn for JSXSpreadChild {
    type Cloned = JSXSpreadChild;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXElementName {
    type Cloned = JSXElementName;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
            Self::JSXMemberExpr(it) => Self::JSXMemberExpr(it.clone_in(ast)),
            Self::JSXNamespacedName(it) => Self::JSXNamespacedName(it.clone_in(ast)),
        }
    }
}
impl CloneIn for JSXOpeningElement {
    type Cloned = JSXOpeningElement;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXAttrOrSpread {
    type Cloned = JSXAttrOrSpread;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::JSXAttr(it) => Self::JSXAttr(it.clone_in(ast)),
            Self::SpreadElement(it) => Self::SpreadElement(it.clone_in(ast)),
        }
    }
}
impl CloneIn for JSXClosingElement {
    type Cloned = JSXClosingElement;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXAttr {
    type Cloned = JSXAttr;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXAttrName {
    type Cloned = JSXAttrName;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Ident(it) => Self::Ident(it.clone_in(ast)),
            Self::JSXNamespacedName(it) => Self::JSXNamespacedName(it.clone_in(ast)),
        }
    }
}
impl CloneIn for JSXAttrValue {
    type Cloned = JSXAttrValue;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::Str(it) => Self::Str(it.clone_in(ast)),
            Self::JSXExprContainer(it) => Self::JSXExprContainer(it.clone_in(ast)),
            Self::JSXElement(it) => Self::JSXElement(it.clone_in(ast)),
            Self::JSXFragment(it) => Self::JSXFragment(it.clone_in(ast)),
        }
    }
}
impl CloneIn for JSXText {
    type Cloned = JSXText;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXElement {
    type Cloned = JSXElement;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXElementChild {
    type Cloned = JSXElementChild;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        match self {
            Self::JSXText(it) => Self::JSXText(it.clone_in(ast)),
            Self::JSXExprContainer(it) => Self::JSXExprContainer(it.clone_in(ast)),
            Self::JSXSpreadChild(it) => Self::JSXSpreadChild(it.clone_in(ast)),
            Self::JSXElement(it) => Self::JSXElement(it.clone_in(ast)),
            Self::JSXFragment(it) => Self::JSXFragment(it.clone_in(ast)),
        }
    }
}
impl CloneIn for JSXFragment {
    type Cloned = JSXFragment;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXOpeningFragment {
    type Cloned = JSXOpeningFragment;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
impl CloneIn for JSXClosingFragment {
    type Cloned = JSXClosingFragment;
    #[inline]
    fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
        todo!()
    }
}
