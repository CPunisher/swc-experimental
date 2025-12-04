use swc_atoms::{Atom, Wtf8Atom};
use swc_ecma_ast as legacy;
use swc_experimental_ecma_ast::{
    self as experimental, Ast, FromNodeId, OptionalUtf8Ref, OptionalWtf8Ref, TypedSubRange,
    Utf8Ref, Wtf8Ref,
};

pub fn transform_program(ast: &Ast, root: experimental::Program) -> legacy::Program {
    match root {
        experimental::Program::Module(module) => {
            legacy::Program::Module(transform_module(ast, module))
        }
        experimental::Program::Script(script) => {
            legacy::Program::Script(transform_script(ast, script))
        }
    }
}

pub fn transform_module(ast: &Ast, module: experimental::Module) -> legacy::Module {
    legacy::Module {
        span: module.span(ast),
        shebang: transform_opt_utf8_ref(ast, module.shebang(ast)),
        body: transform_type_sub_range(ast, module.body(ast), transform_module_item),
    }
}

pub fn transform_script(ast: &Ast, script: experimental::Script) -> legacy::Script {
    legacy::Script {
        span: script.span(ast),
        body: transform_type_sub_range(ast, script.body(ast), transform_stmt),
        shebang: transform_opt_utf8_ref(ast, script.shebang(ast)),
    }
}

fn transform_module_item(ast: &Ast, item: experimental::ModuleItem) -> legacy::ModuleItem {
    match item {
        experimental::ModuleItem::ModuleDecl(module_decl) => {
            legacy::ModuleItem::ModuleDecl(transform_module_decl(ast, module_decl))
        }
        experimental::ModuleItem::Stmt(stmt) => legacy::ModuleItem::Stmt(transform_stmt(ast, stmt)),
    }
}

fn transform_module_decl(ast: &Ast, module_decl: experimental::ModuleDecl) -> legacy::ModuleDecl {
    match module_decl {
        experimental::ModuleDecl::Import(import_decl) => {
            legacy::ModuleDecl::Import(transform_import_decl(ast, import_decl))
        }
        experimental::ModuleDecl::ExportDecl(export_decl) => {
            legacy::ModuleDecl::ExportDecl(transform_export_decl(ast, export_decl))
        }
        experimental::ModuleDecl::ExportNamed(named_export) => {
            legacy::ModuleDecl::ExportNamed(transform_export_named(ast, named_export))
        }
        experimental::ModuleDecl::ExportDefaultDecl(export_default_decl) => {
            legacy::ModuleDecl::ExportDefaultDecl(transform_export_default_decl(
                ast,
                export_default_decl,
            ))
        }
        experimental::ModuleDecl::ExportDefaultExpr(export_default_expr) => {
            legacy::ModuleDecl::ExportDefaultExpr(transform_export_default_expr(
                ast,
                export_default_expr,
            ))
        }
        experimental::ModuleDecl::ExportAll(export_all) => {
            legacy::ModuleDecl::ExportAll(transform_export_all(ast, export_all))
        }
    }
}

fn transform_import_decl(ast: &Ast, import_decl: experimental::ImportDecl) -> legacy::ImportDecl {
    legacy::ImportDecl {
        span: import_decl.span(ast),
        specifiers: transform_type_sub_range(
            ast,
            import_decl.specifiers(ast),
            transform_import_specifier,
        ),
        src: Box::new(transform_str(ast, import_decl.src(ast))),
        type_only: import_decl.type_only(ast),
        with: import_decl
            .with(ast)
            .map(|with| Box::new(transform_object_lit(ast, with))),
        phase: match import_decl.phase(ast) {
            experimental::ImportPhase::Evaluation => legacy::ImportPhase::Evaluation,
            experimental::ImportPhase::Source => legacy::ImportPhase::Source,
            experimental::ImportPhase::Defer => legacy::ImportPhase::Defer,
        },
    }
}

fn transform_export_decl(ast: &Ast, export_decl: experimental::ExportDecl) -> legacy::ExportDecl {
    legacy::ExportDecl {
        span: export_decl.span(ast),
        decl: transform_decl(ast, export_decl.decl(ast)),
    }
}

fn transform_export_named(
    ast: &Ast,
    export_named: experimental::NamedExport,
) -> legacy::NamedExport {
    legacy::NamedExport {
        span: export_named.span(ast),
        specifiers: transform_type_sub_range(
            ast,
            export_named.specifiers(ast),
            transform_export_specifier,
        ),
        src: export_named
            .src(ast)
            .map(|s| Box::new(transform_str(ast, s))),
        type_only: export_named.type_only(ast),
        with: export_named
            .with(ast)
            .map(|o| Box::new(transform_object_lit(ast, o))),
    }
}

fn transform_export_default_decl(
    ast: &Ast,
    export_default_decl: experimental::ExportDefaultDecl,
) -> legacy::ExportDefaultDecl {
    legacy::ExportDefaultDecl {
        span: export_default_decl.span(ast),
        decl: match export_default_decl.decl(ast) {
            experimental::DefaultDecl::Class(cls) => {
                legacy::DefaultDecl::Class(transform_class_expr(ast, cls))
            }
            experimental::DefaultDecl::Fn(f) => legacy::DefaultDecl::Fn(transform_fn_expr(ast, f)),
        },
    }
}

fn transform_export_default_expr(
    ast: &Ast,
    export_default_expr: experimental::ExportDefaultExpr,
) -> legacy::ExportDefaultExpr {
    legacy::ExportDefaultExpr {
        span: export_default_expr.span(ast),
        expr: Box::new(transform_expr(ast, export_default_expr.expr(ast))),
    }
}

fn transform_export_all(ast: &Ast, export_all: experimental::ExportAll) -> legacy::ExportAll {
    legacy::ExportAll {
        span: export_all.span(ast),
        src: Box::new(transform_str(ast, export_all.src(ast))),
        type_only: export_all.type_only(ast),
        with: export_all
            .with(ast)
            .map(|o| Box::new(transform_object_lit(ast, o))),
    }
}

fn transform_stmt(ast: &Ast, stmt: experimental::Stmt) -> legacy::Stmt {
    match stmt {
        experimental::Stmt::Block(block_stmt) => {
            legacy::Stmt::Block(transform_block_stmt(ast, block_stmt))
        }
        experimental::Stmt::Empty(empty_stmt) => {
            legacy::Stmt::Empty(transform_empty_stmt(ast, empty_stmt))
        }
        experimental::Stmt::Debugger(debugger_stmt) => {
            legacy::Stmt::Debugger(legacy::DebuggerStmt {
                span: debugger_stmt.span(ast),
            })
        }
        experimental::Stmt::With(with_stmt) => legacy::Stmt::With(legacy::WithStmt {
            span: with_stmt.span(ast),
            obj: Box::new(transform_expr(ast, with_stmt.obj(ast))),
            body: Box::new(transform_stmt(ast, with_stmt.body(ast))),
        }),
        experimental::Stmt::Return(return_stmt) => legacy::Stmt::Return(legacy::ReturnStmt {
            span: return_stmt.span(ast),
            arg: return_stmt
                .arg(ast)
                .map(|arg| Box::new(transform_expr(ast, arg))),
        }),
        experimental::Stmt::Labeled(labeled_stmt) => legacy::Stmt::Labeled(legacy::LabeledStmt {
            span: labeled_stmt.span(ast),
            label: transform_ident(ast, labeled_stmt.label(ast)),
            body: Box::new(transform_stmt(ast, labeled_stmt.body(ast))),
        }),
        experimental::Stmt::Break(break_stmt) => legacy::Stmt::Break(legacy::BreakStmt {
            span: break_stmt.span(ast),
            label: break_stmt
                .label(ast)
                .map(|label| transform_ident(ast, label)),
        }),
        experimental::Stmt::Continue(continue_stmt) => {
            legacy::Stmt::Continue(legacy::ContinueStmt {
                span: continue_stmt.span(ast),
                label: continue_stmt
                    .label(ast)
                    .map(|label| transform_ident(ast, label)),
            })
        }
        experimental::Stmt::If(if_stmt) => legacy::Stmt::If(legacy::IfStmt {
            span: if_stmt.span(ast),
            test: Box::new(transform_expr(ast, if_stmt.test(ast))),
            cons: Box::new(transform_stmt(ast, if_stmt.cons(ast))),
            alt: if_stmt
                .alt(ast)
                .map(|alt| Box::new(transform_stmt(ast, alt))),
        }),
        experimental::Stmt::Switch(switch_stmt) => legacy::Stmt::Switch(legacy::SwitchStmt {
            span: switch_stmt.span(ast),
            discriminant: Box::new(transform_expr(ast, switch_stmt.discriminant(ast))),
            cases: transform_type_sub_range(ast, switch_stmt.cases(ast), transform_switch_case),
        }),
        experimental::Stmt::Throw(throw_stmt) => legacy::Stmt::Throw(legacy::ThrowStmt {
            span: throw_stmt.span(ast),
            arg: Box::new(transform_expr(ast, throw_stmt.arg(ast))),
        }),
        experimental::Stmt::Try(try_stmt) => legacy::Stmt::Try(Box::new(legacy::TryStmt {
            span: try_stmt.span(ast),
            block: transform_block_stmt(ast, try_stmt.block(ast)),
            handler: try_stmt
                .handler(ast)
                .map(|handler| transform_catch_clause(ast, handler)),
            finalizer: try_stmt
                .finalizer(ast)
                .map(|finalizer| transform_block_stmt(ast, finalizer)),
        })),
        experimental::Stmt::While(while_stmt) => legacy::Stmt::While(legacy::WhileStmt {
            span: while_stmt.span(ast),
            test: Box::new(transform_expr(ast, while_stmt.test(ast))),
            body: Box::new(transform_stmt(ast, while_stmt.body(ast))),
        }),
        experimental::Stmt::DoWhile(do_while_stmt) => legacy::Stmt::DoWhile(legacy::DoWhileStmt {
            span: do_while_stmt.span(ast),
            test: Box::new(transform_expr(ast, do_while_stmt.test(ast))),
            body: Box::new(transform_stmt(ast, do_while_stmt.body(ast))),
        }),
        experimental::Stmt::For(for_stmt) => legacy::Stmt::For(legacy::ForStmt {
            span: for_stmt.span(ast),
            init: for_stmt
                .init(ast)
                .map(|i| transform_var_decl_or_expr(ast, i)),
            test: for_stmt.test(ast).map(|e| Box::new(transform_expr(ast, e))),
            update: for_stmt
                .update(ast)
                .map(|e| Box::new(transform_expr(ast, e))),
            body: Box::new(transform_stmt(ast, for_stmt.body(ast))),
        }),
        experimental::Stmt::ForIn(for_in_stmt) => legacy::Stmt::ForIn(legacy::ForInStmt {
            span: for_in_stmt.span(ast),
            left: transform_for_head(ast, for_in_stmt.left(ast)),
            right: Box::new(transform_expr(ast, for_in_stmt.right(ast))),
            body: Box::new(transform_stmt(ast, for_in_stmt.body(ast))),
        }),
        experimental::Stmt::ForOf(for_of_stmt) => legacy::Stmt::ForOf(legacy::ForOfStmt {
            span: for_of_stmt.span(ast),
            is_await: for_of_stmt.is_await(ast),
            left: transform_for_head(ast, for_of_stmt.left(ast)),
            right: Box::new(transform_expr(ast, for_of_stmt.right(ast))),
            body: Box::new(transform_stmt(ast, for_of_stmt.body(ast))),
        }),
        experimental::Stmt::Decl(decl) => legacy::Stmt::Decl(transform_decl(ast, decl)),
        experimental::Stmt::Expr(expr_stmt) => legacy::Stmt::Expr(legacy::ExprStmt {
            span: expr_stmt.span(ast),
            expr: Box::new(transform_expr(ast, expr_stmt.expr(ast))),
        }),
    }
}

fn transform_block_stmt(ast: &Ast, block_stmt: experimental::BlockStmt) -> legacy::BlockStmt {
    legacy::BlockStmt {
        span: block_stmt.span(ast),
        stmts: transform_type_sub_range(ast, block_stmt.stmts(ast), transform_stmt),
        ctxt: Default::default(),
    }
}

fn transform_empty_stmt(ast: &Ast, empty_stmt: experimental::EmptyStmt) -> legacy::EmptyStmt {
    legacy::EmptyStmt {
        span: empty_stmt.span(ast),
    }
}

fn transform_expr(ast: &Ast, expr: experimental::Expr) -> legacy::Expr {
    match expr {
        experimental::Expr::This(t) => legacy::Expr::This(legacy::ThisExpr { span: t.span(ast) }),
        experimental::Expr::Array(a) => legacy::Expr::Array(legacy::ArrayLit {
            span: a.span(ast),
            elems: transform_opt_type_sub_range(ast, a.elems(ast), transform_expr_or_spread),
        }),
        experimental::Expr::Object(o) => legacy::Expr::Object(transform_object_lit(ast, o)),
        experimental::Expr::Fn(f) => legacy::Expr::Fn(transform_fn_expr(ast, f)),
        experimental::Expr::Unary(u) => legacy::Expr::Unary(legacy::UnaryExpr {
            span: u.span(ast),
            op: match u.op(ast) {
                experimental::UnaryOp::Minus => legacy::UnaryOp::Minus,
                experimental::UnaryOp::Plus => legacy::UnaryOp::Plus,
                experimental::UnaryOp::Bang => legacy::UnaryOp::Bang,
                experimental::UnaryOp::Tilde => legacy::UnaryOp::Tilde,
                experimental::UnaryOp::TypeOf => legacy::UnaryOp::TypeOf,
                experimental::UnaryOp::Void => legacy::UnaryOp::Void,
                experimental::UnaryOp::Delete => legacy::UnaryOp::Delete,
            },
            arg: Box::new(transform_expr(ast, u.arg(ast))),
        }),
        experimental::Expr::Update(u) => legacy::Expr::Update(legacy::UpdateExpr {
            span: u.span(ast),
            op: match u.op(ast) {
                experimental::UpdateOp::PlusPlus => legacy::UpdateOp::PlusPlus,
                experimental::UpdateOp::MinusMinus => legacy::UpdateOp::MinusMinus,
            },
            prefix: u.prefix(ast),
            arg: Box::new(transform_expr(ast, u.arg(ast))),
        }),
        experimental::Expr::Bin(b) => legacy::Expr::Bin(legacy::BinExpr {
            span: b.span(ast),
            op: match b.op(ast) {
                experimental::BinaryOp::EqEq => legacy::BinaryOp::EqEq,
                experimental::BinaryOp::NotEq => legacy::BinaryOp::NotEq,
                experimental::BinaryOp::EqEqEq => legacy::BinaryOp::EqEqEq,
                experimental::BinaryOp::NotEqEq => legacy::BinaryOp::NotEqEq,
                experimental::BinaryOp::Lt => legacy::BinaryOp::Lt,
                experimental::BinaryOp::LtEq => legacy::BinaryOp::LtEq,
                experimental::BinaryOp::Gt => legacy::BinaryOp::Gt,
                experimental::BinaryOp::GtEq => legacy::BinaryOp::GtEq,
                experimental::BinaryOp::LShift => legacy::BinaryOp::LShift,
                experimental::BinaryOp::RShift => legacy::BinaryOp::RShift,
                experimental::BinaryOp::ZeroFillRShift => legacy::BinaryOp::ZeroFillRShift,
                experimental::BinaryOp::Add => legacy::BinaryOp::Add,
                experimental::BinaryOp::Sub => legacy::BinaryOp::Sub,
                experimental::BinaryOp::Mul => legacy::BinaryOp::Mul,
                experimental::BinaryOp::Div => legacy::BinaryOp::Div,
                experimental::BinaryOp::Mod => legacy::BinaryOp::Mod,
                experimental::BinaryOp::BitOr => legacy::BinaryOp::BitOr,
                experimental::BinaryOp::BitXor => legacy::BinaryOp::BitXor,
                experimental::BinaryOp::BitAnd => legacy::BinaryOp::BitAnd,
                experimental::BinaryOp::LogicalOr => legacy::BinaryOp::LogicalOr,
                experimental::BinaryOp::LogicalAnd => legacy::BinaryOp::LogicalAnd,
                experimental::BinaryOp::In => legacy::BinaryOp::In,
                experimental::BinaryOp::InstanceOf => legacy::BinaryOp::InstanceOf,
                experimental::BinaryOp::Exp => legacy::BinaryOp::Exp,
                experimental::BinaryOp::NullishCoalescing => legacy::BinaryOp::NullishCoalescing,
            },
            left: Box::new(transform_expr(ast, b.left(ast))),
            right: Box::new(transform_expr(ast, b.right(ast))),
        }),
        experimental::Expr::Assign(a) => legacy::Expr::Assign(legacy::AssignExpr {
            span: a.span(ast),
            op: match a.op(ast) {
                experimental::AssignOp::Assign => legacy::AssignOp::Assign,
                experimental::AssignOp::AddAssign => legacy::AssignOp::AddAssign,
                experimental::AssignOp::SubAssign => legacy::AssignOp::SubAssign,
                experimental::AssignOp::MulAssign => legacy::AssignOp::MulAssign,
                experimental::AssignOp::DivAssign => legacy::AssignOp::DivAssign,
                experimental::AssignOp::ModAssign => legacy::AssignOp::ModAssign,
                experimental::AssignOp::LShiftAssign => legacy::AssignOp::LShiftAssign,
                experimental::AssignOp::RShiftAssign => legacy::AssignOp::RShiftAssign,
                experimental::AssignOp::ZeroFillRShiftAssign => {
                    legacy::AssignOp::ZeroFillRShiftAssign
                }
                experimental::AssignOp::BitOrAssign => legacy::AssignOp::BitOrAssign,
                experimental::AssignOp::BitXorAssign => legacy::AssignOp::BitXorAssign,
                experimental::AssignOp::BitAndAssign => legacy::AssignOp::BitAndAssign,
                experimental::AssignOp::ExpAssign => legacy::AssignOp::ExpAssign,
                experimental::AssignOp::AndAssign => legacy::AssignOp::AndAssign,
                experimental::AssignOp::OrAssign => legacy::AssignOp::OrAssign,
                experimental::AssignOp::NullishAssign => legacy::AssignOp::NullishAssign,
            },
            left: transform_assign_target(ast, a.left(ast)),
            right: Box::new(transform_expr(ast, a.right(ast))),
        }),
        experimental::Expr::Member(m) => legacy::Expr::Member(legacy::MemberExpr {
            span: m.span(ast),
            obj: Box::new(transform_expr(ast, m.obj(ast))),
            prop: transform_member_prop(ast, m.prop(ast)),
        }),
        experimental::Expr::SuperProp(s) => legacy::Expr::SuperProp(legacy::SuperPropExpr {
            span: s.span(ast),
            obj: legacy::Super {
                span: s.obj(ast).span(ast),
            },
            prop: match s.prop(ast) {
                experimental::SuperProp::Ident(i) => legacy::SuperProp::Ident(legacy::IdentName {
                    span: i.span(ast),
                    sym: transform_utf8_ref(ast, i.sym(ast)),
                }),
                experimental::SuperProp::Computed(c) => {
                    legacy::SuperProp::Computed(legacy::ComputedPropName {
                        span: c.span(ast),
                        expr: Box::new(transform_expr(ast, c.expr(ast))),
                    })
                }
            },
        }),
        experimental::Expr::Cond(c) => legacy::Expr::Cond(legacy::CondExpr {
            span: c.span(ast),
            test: Box::new(transform_expr(ast, c.test(ast))),
            cons: Box::new(transform_expr(ast, c.cons(ast))),
            alt: Box::new(transform_expr(ast, c.alt(ast))),
        }),
        experimental::Expr::Call(c) => legacy::Expr::Call(legacy::CallExpr {
            span: c.span(ast),
            ctxt: Default::default(),
            callee: match c.callee(ast) {
                experimental::Callee::Super(s) => {
                    legacy::Callee::Super(legacy::Super { span: s.span(ast) })
                }
                experimental::Callee::Import(i) => legacy::Callee::Import(legacy::Import {
                    span: i.span(ast),
                    phase: match i.phase(ast) {
                        experimental::ImportPhase::Evaluation => legacy::ImportPhase::Evaluation,
                        experimental::ImportPhase::Source => legacy::ImportPhase::Source,
                        experimental::ImportPhase::Defer => legacy::ImportPhase::Defer,
                    },
                }),
                experimental::Callee::Expr(e) => {
                    legacy::Callee::Expr(Box::new(transform_expr(ast, e)))
                }
            },
            args: transform_type_sub_range(ast, c.args(ast), transform_expr_or_spread),
            type_args: None,
        }),
        experimental::Expr::New(n) => legacy::Expr::New(legacy::NewExpr {
            span: n.span(ast),
            ctxt: Default::default(),
            callee: Box::new(transform_expr(ast, n.callee(ast))),
            args: Some(transform_type_sub_range(
                ast,
                n.args(ast),
                transform_expr_or_spread,
            )),
            type_args: None,
        }),
        experimental::Expr::Seq(s) => legacy::Expr::Seq(legacy::SeqExpr {
            span: s.span(ast),
            exprs: transform_type_sub_range(ast, s.exprs(ast), |ast, e| {
                Box::new(transform_expr(ast, e))
            }),
        }),
        experimental::Expr::Ident(i) => legacy::Expr::Ident(transform_ident(ast, i)),
        experimental::Expr::Lit(l) => legacy::Expr::Lit(transform_lit(ast, l)),
        experimental::Expr::Tpl(t) => legacy::Expr::Tpl(legacy::Tpl {
            span: t.span(ast),
            exprs: transform_type_sub_range(ast, t.exprs(ast), |ast, e| {
                Box::new(transform_expr(ast, e))
            }),
            quasis: transform_type_sub_range(ast, t.quasis(ast), transform_tpl_element),
        }),
        experimental::Expr::TaggedTpl(tt) => legacy::Expr::TaggedTpl(legacy::TaggedTpl {
            span: tt.span(ast),
            ctxt: Default::default(),
            tag: Box::new(transform_expr(ast, tt.tag(ast))),
            tpl: Box::new(transform_tpl(ast, tt.tpl(ast))),
            type_params: None,
        }),
        experimental::Expr::Arrow(a) => legacy::Expr::Arrow(legacy::ArrowExpr {
            span: a.span(ast),
            ctxt: Default::default(),
            params: transform_type_sub_range(ast, a.params(ast), transform_pat),
            body: Box::new(match a.body(ast) {
                experimental::BlockStmtOrExpr::BlockStmt(b) => {
                    legacy::BlockStmtOrExpr::BlockStmt(transform_block_stmt(ast, b))
                }
                experimental::BlockStmtOrExpr::Expr(e) => {
                    legacy::BlockStmtOrExpr::Expr(Box::new(transform_expr(ast, e)))
                }
            }),
            is_async: a.is_async(ast),
            is_generator: a.is_generator(ast),
            type_params: None,
            return_type: None,
        }),
        experimental::Expr::Class(c) => legacy::Expr::Class(transform_class_expr(ast, c)),
        experimental::Expr::Yield(y) => legacy::Expr::Yield(legacy::YieldExpr {
            span: y.span(ast),
            arg: y.arg(ast).map(|e| Box::new(transform_expr(ast, e))),
            delegate: y.delegate(ast),
        }),
        experimental::Expr::MetaProp(m) => legacy::Expr::MetaProp(legacy::MetaPropExpr {
            span: m.span(ast),
            kind: match m.kind(ast) {
                experimental::MetaPropKind::NewTarget => legacy::MetaPropKind::NewTarget,
                experimental::MetaPropKind::ImportMeta => legacy::MetaPropKind::ImportMeta,
            },
        }),
        experimental::Expr::Await(a) => legacy::Expr::Await(legacy::AwaitExpr {
            span: a.span(ast),
            arg: Box::new(transform_expr(ast, a.arg(ast))),
        }),
        experimental::Expr::Paren(p) => legacy::Expr::Paren(legacy::ParenExpr {
            span: p.span(ast),
            expr: Box::new(transform_expr(ast, p.expr(ast))),
        }),
        experimental::Expr::PrivateName(p) => legacy::Expr::PrivateName(legacy::PrivateName {
            span: p.span(ast),
            name: transform_utf8_ref(ast, p.name(ast)),
        }),
        experimental::Expr::OptChain(o) => legacy::Expr::OptChain(legacy::OptChainExpr {
            span: o.span(ast),
            optional: o.optional(ast),
            base: Box::new(match o.base(ast) {
                experimental::OptChainBase::Member(m) => {
                    legacy::OptChainBase::Member(legacy::MemberExpr {
                        span: m.span(ast),
                        obj: Box::new(transform_expr(ast, m.obj(ast))),
                        prop: transform_member_prop(ast, m.prop(ast)),
                    })
                }
                experimental::OptChainBase::Call(c) => {
                    legacy::OptChainBase::Call(legacy::OptCall {
                        span: c.span(ast),
                        ctxt: Default::default(),
                        callee: Box::new(transform_expr(ast, c.callee(ast))),
                        args: transform_type_sub_range(ast, c.args(ast), transform_expr_or_spread),
                        type_args: None,
                    })
                }
            }),
        }),
        experimental::Expr::Invalid(i) => {
            legacy::Expr::Invalid(legacy::Invalid { span: i.span(ast) })
        }
        _ => todo!(),
    }
}

fn transform_ident(ast: &Ast, ident: experimental::Ident) -> legacy::Ident {
    legacy::Ident {
        span: ident.span(ast),
        ctxt: Default::default(),
        sym: transform_utf8_ref(ast, ident.sym(ast)),
        optional: ident.optional(ast),
    }
}

fn transform_switch_case(ast: &Ast, c: experimental::SwitchCase) -> legacy::SwitchCase {
    legacy::SwitchCase {
        span: c.span(ast),
        test: c.test(ast).map(|e| Box::new(transform_expr(ast, e))),
        cons: transform_type_sub_range(ast, c.cons(ast), transform_stmt),
    }
}

fn transform_catch_clause(ast: &Ast, c: experimental::CatchClause) -> legacy::CatchClause {
    legacy::CatchClause {
        span: c.span(ast),
        param: c.param(ast).map(|p| transform_pat(ast, p)),
        body: transform_block_stmt(ast, c.body(ast)),
    }
}

fn transform_var_decl_or_expr(ast: &Ast, v: experimental::VarDeclOrExpr) -> legacy::VarDeclOrExpr {
    match v {
        experimental::VarDeclOrExpr::VarDecl(d) => {
            legacy::VarDeclOrExpr::VarDecl(Box::new(transform_var_decl(ast, d)))
        }
        experimental::VarDeclOrExpr::Expr(e) => {
            legacy::VarDeclOrExpr::Expr(Box::new(transform_expr(ast, e)))
        }
    }
}

fn transform_var_decl(ast: &Ast, v: experimental::VarDecl) -> legacy::VarDecl {
    legacy::VarDecl {
        span: v.span(ast),
        ctxt: Default::default(),
        kind: match v.kind(ast) {
            experimental::VarDeclKind::Var => legacy::VarDeclKind::Var,
            experimental::VarDeclKind::Let => legacy::VarDeclKind::Let,
            experimental::VarDeclKind::Const => legacy::VarDeclKind::Const,
        },
        declare: v.declare(ast),
        decls: transform_type_sub_range(ast, v.decls(ast), transform_var_declarator),
    }
}

fn transform_for_head(ast: &Ast, h: experimental::ForHead) -> legacy::ForHead {
    match h {
        experimental::ForHead::VarDecl(v) => {
            legacy::ForHead::VarDecl(Box::new(transform_var_decl(ast, v)))
        }
        experimental::ForHead::UsingDecl(u) => {
            legacy::ForHead::UsingDecl(Box::new(legacy::UsingDecl {
                span: u.span(ast),
                is_await: u.is_await(ast),
                decls: transform_type_sub_range(ast, u.decls(ast), transform_var_declarator),
            }))
        }
        experimental::ForHead::Pat(p) => legacy::ForHead::Pat(Box::new(transform_pat(ast, p))),
    }
}

// -------------------------------------------------------------------------------
// Helpers for module declarations and common nodes

fn transform_import_specifier(
    ast: &Ast,
    s: experimental::ImportSpecifier,
) -> legacy::ImportSpecifier {
    match s {
        experimental::ImportSpecifier::Named(n) => {
            legacy::ImportSpecifier::Named(legacy::ImportNamedSpecifier {
                span: n.span(ast),
                local: transform_ident(ast, n.local(ast)),
                imported: n
                    .imported(ast)
                    .map(|me| transform_module_export_name(ast, me)),
                is_type_only: n.is_type_only(ast),
            })
        }
        experimental::ImportSpecifier::Default(d) => {
            legacy::ImportSpecifier::Default(legacy::ImportDefaultSpecifier {
                span: d.span(ast),
                local: transform_ident(ast, d.local(ast)),
            })
        }
        experimental::ImportSpecifier::Namespace(ns) => {
            legacy::ImportSpecifier::Namespace(legacy::ImportStarAsSpecifier {
                span: ns.span(ast),
                local: transform_ident(ast, ns.local(ast)),
            })
        }
    }
}

fn transform_export_specifier(
    ast: &Ast,
    s: experimental::ExportSpecifier,
) -> legacy::ExportSpecifier {
    match s {
        experimental::ExportSpecifier::Namespace(ns) => {
            legacy::ExportSpecifier::Namespace(legacy::ExportNamespaceSpecifier {
                span: ns.span(ast),
                name: transform_module_export_name(ast, ns.name(ast)),
            })
        }
        experimental::ExportSpecifier::Default(d) => {
            legacy::ExportSpecifier::Default(legacy::ExportDefaultSpecifier {
                exported: transform_ident(ast, d.exported(ast)),
            })
        }
        experimental::ExportSpecifier::Named(n) => {
            legacy::ExportSpecifier::Named(legacy::ExportNamedSpecifier {
                span: n.span(ast),
                orig: transform_module_export_name(ast, n.orig(ast)),
                exported: n
                    .exported(ast)
                    .map(|me| transform_module_export_name(ast, me)),
                is_type_only: n.is_type_only(ast),
            })
        }
    }
}

fn transform_module_export_name(
    ast: &Ast,
    n: experimental::ModuleExportName,
) -> legacy::ModuleExportName {
    match n {
        experimental::ModuleExportName::Ident(i) => {
            legacy::ModuleExportName::Ident(transform_ident(ast, i))
        }
        experimental::ModuleExportName::Str(s) => {
            legacy::ModuleExportName::Str(transform_str(ast, s))
        }
    }
}

fn transform_object_lit(ast: &Ast, o: experimental::ObjectLit) -> legacy::ObjectLit {
    legacy::ObjectLit {
        span: o.span(ast),
        props: transform_type_sub_range(ast, o.props(ast), transform_prop_or_spread),
    }
}

fn transform_prop_or_spread(ast: &Ast, p: experimental::PropOrSpread) -> legacy::PropOrSpread {
    match p {
        experimental::PropOrSpread::SpreadElement(s) => {
            legacy::PropOrSpread::Spread(legacy::SpreadElement {
                dot3_token: s.dot_3_token(ast),
                expr: Box::new(transform_expr(ast, s.expr(ast))),
            })
        }
        experimental::PropOrSpread::Prop(prop) => {
            legacy::PropOrSpread::Prop(Box::new(transform_prop(ast, prop)))
        }
    }
}

fn transform_prop(ast: &Ast, p: experimental::Prop) -> legacy::Prop {
    match p {
        experimental::Prop::Shorthand(i) => legacy::Prop::Shorthand(transform_ident(ast, i)),
        experimental::Prop::KeyValue(kv) => legacy::Prop::KeyValue(legacy::KeyValueProp {
            key: transform_prop_name(ast, kv.key(ast)),
            value: Box::new(transform_expr(ast, kv.value(ast))),
        }),
        experimental::Prop::Assign(ap) => legacy::Prop::Assign(legacy::AssignProp {
            span: ap.span(ast),
            key: transform_ident(ast, ap.key(ast)),
            value: Box::new(transform_expr(ast, ap.value(ast))),
        }),
        experimental::Prop::Getter(g) => legacy::Prop::Getter(legacy::GetterProp {
            span: g.span(ast),
            key: transform_prop_name(ast, g.key(ast)),
            type_ann: None,
            body: g.body(ast).map(|b| transform_block_stmt(ast, b)),
        }),
        experimental::Prop::Setter(s) => legacy::Prop::Setter(legacy::SetterProp {
            span: s.span(ast),
            key: transform_prop_name(ast, s.key(ast)),
            this_param: s.this_param(ast).map(|p| transform_pat(ast, p)),
            param: Box::new(transform_pat(ast, s.param(ast))),
            body: s.body(ast).map(|b| transform_block_stmt(ast, b)),
        }),
        experimental::Prop::Method(m) => legacy::Prop::Method(legacy::MethodProp {
            key: transform_prop_name(ast, m.key(ast)),
            function: Box::new(transform_function(ast, m.function(ast))),
        }),
    }
}

fn transform_prop_name(ast: &Ast, n: experimental::PropName) -> legacy::PropName {
    match n {
        experimental::PropName::Ident(i) => legacy::PropName::Ident(legacy::IdentName {
            span: i.span(ast),
            sym: transform_utf8_ref(ast, i.sym(ast)),
        }),
        experimental::PropName::Str(s) => legacy::PropName::Str(transform_str(ast, s)),
        experimental::PropName::Num(n) => legacy::PropName::Num(legacy::Number {
            span: n.span(ast),
            value: n.value(ast),
            raw: transform_opt_utf8_ref(ast, n.raw(ast)),
        }),
        experimental::PropName::Computed(c) => {
            legacy::PropName::Computed(legacy::ComputedPropName {
                span: c.span(ast),
                expr: Box::new(transform_expr(ast, c.expr(ast))),
            })
        }
        experimental::PropName::BigInt(b) => legacy::PropName::Num(legacy::Number {
            span: b.span(ast),
            value: 0.0,
            raw: None,
        }),
    }
}

fn transform_str(ast: &Ast, s: experimental::Str) -> legacy::Str {
    legacy::Str {
        span: s.span(ast),
        value: transform_wtf8_ref(ast, s.value(ast)),
        raw: transform_opt_utf8_ref(ast, s.raw(ast)),
    }
}

// -------------------------------------------------------------------------------
// Function / Class basics used by export default

fn transform_fn_expr(ast: &Ast, f: experimental::FnExpr) -> legacy::FnExpr {
    legacy::FnExpr {
        ident: f.ident(ast).map(|i| transform_ident(ast, i)),
        function: Box::new(transform_function(ast, f.function(ast))),
    }
}

fn transform_class_expr(ast: &Ast, c: experimental::ClassExpr) -> legacy::ClassExpr {
    legacy::ClassExpr {
        ident: c.ident(ast).map(|i| transform_ident(ast, i)),
        class: Box::new(transform_class(ast, c.class(ast))),
    }
}

fn transform_function(ast: &Ast, f: experimental::Function) -> legacy::Function {
    legacy::Function {
        params: transform_type_sub_range(ast, f.params(ast), transform_param),
        decorators: transform_type_sub_range(ast, f.decorators(ast), transform_decorator),
        span: f.span(ast),
        ctxt: Default::default(),
        body: f.body(ast).map(|b| transform_block_stmt(ast, b)),
        is_generator: f.is_generator(ast),
        is_async: f.is_async(ast),
        type_params: None,
        return_type: None,
    }
}

fn transform_param(ast: &Ast, p: experimental::Param) -> legacy::Param {
    legacy::Param {
        span: p.span(ast),
        decorators: transform_type_sub_range(ast, p.decorators(ast), transform_decorator),
        pat: transform_pat(ast, p.pat(ast)),
    }
}

fn transform_decorator(ast: &Ast, d: experimental::Decorator) -> legacy::Decorator {
    legacy::Decorator {
        span: d.span(ast),
        expr: Box::new(transform_expr(ast, d.expr(ast))),
    }
}

fn transform_class(ast: &Ast, c: experimental::Class) -> legacy::Class {
    legacy::Class {
        span: c.span(ast),
        ctxt: Default::default(),
        decorators: transform_type_sub_range(ast, c.decorators(ast), transform_decorator),
        body: transform_type_sub_range(ast, c.body(ast), transform_class_member),
        super_class: c.super_class(ast).map(|e| Box::new(transform_expr(ast, e))),
        is_abstract: c.is_abstract(ast),
        type_params: None,
        super_type_params: None,
        implements: Default::default(),
    }
}

fn transform_class_member(ast: &Ast, m: experimental::ClassMember) -> legacy::ClassMember {
    match m {
        experimental::ClassMember::Constructor(k) => {
            legacy::ClassMember::Constructor(legacy::Constructor {
                span: k.span(ast),
                ctxt: Default::default(),
                key: transform_prop_name(ast, k.key(ast)),
                params: transform_type_sub_range(
                    ast,
                    k.params(ast),
                    transform_param_or_ts_param_prop,
                ),
                body: k.body(ast).map(|b| transform_block_stmt(ast, b)),
                accessibility: None,
                is_optional: false,
            })
        }
        experimental::ClassMember::Method(me) => legacy::ClassMember::Method(legacy::ClassMethod {
            span: me.span(ast),
            key: transform_prop_name(ast, me.key(ast)),
            function: Box::new(transform_function(ast, me.function(ast))),
            kind: match me.kind(ast) {
                experimental::MethodKind::Method => legacy::MethodKind::Method,
                experimental::MethodKind::Getter => legacy::MethodKind::Getter,
                experimental::MethodKind::Setter => legacy::MethodKind::Setter,
            },
            is_static: me.is_static(ast),
            accessibility: None,
            is_abstract: false,
            is_optional: false,
            is_override: false,
        }),
        experimental::ClassMember::PrivateMethod(pm) => {
            legacy::ClassMember::PrivateMethod(legacy::PrivateMethod {
                span: pm.span(ast),
                key: legacy::PrivateName {
                    span: pm.key(ast).span(ast),
                    name: transform_utf8_ref(ast, pm.key(ast).name(ast)),
                },
                function: Box::new(transform_function(ast, pm.function(ast))),
                kind: match pm.kind(ast) {
                    experimental::MethodKind::Method => legacy::MethodKind::Method,
                    experimental::MethodKind::Getter => legacy::MethodKind::Getter,
                    experimental::MethodKind::Setter => legacy::MethodKind::Setter,
                },
                is_static: pm.is_static(ast),
                accessibility: None,
                is_abstract: false,
                is_optional: false,
                is_override: false,
            })
        }
        experimental::ClassMember::ClassProp(cp) => {
            legacy::ClassMember::ClassProp(legacy::ClassProp {
                span: cp.span(ast),
                key: transform_prop_name(ast, cp.key(ast)),
                value: cp.value(ast).map(|e| Box::new(transform_expr(ast, e))),
                type_ann: None,
                is_static: cp.is_static(ast),
                decorators: transform_type_sub_range(ast, cp.decorators(ast), transform_decorator),
                accessibility: None,
                is_abstract: false,
                is_optional: false,
                is_override: false,
                readonly: false,
                declare: false,
                definite: false,
            })
        }
        experimental::ClassMember::PrivateProp(pp) => {
            legacy::ClassMember::PrivateProp(legacy::PrivateProp {
                span: pp.span(ast),
                ctxt: Default::default(),
                key: legacy::PrivateName {
                    span: pp.key(ast).span(ast),
                    name: transform_utf8_ref(ast, pp.key(ast).name(ast)),
                },
                value: pp.value(ast).map(|e| Box::new(transform_expr(ast, e))),
                type_ann: None,
                is_static: pp.is_static(ast),
                decorators: transform_type_sub_range(ast, pp.decorators(ast), transform_decorator),
                accessibility: None,
                is_optional: false,
                is_override: false,
                readonly: false,
                definite: false,
            })
        }
        experimental::ClassMember::Empty(e) => {
            legacy::ClassMember::Empty(transform_empty_stmt(ast, e))
        }
        experimental::ClassMember::StaticBlock(sb) => {
            legacy::ClassMember::StaticBlock(legacy::StaticBlock {
                span: sb.span(ast),
                body: transform_block_stmt(ast, sb.body(ast)),
            })
        }
        experimental::ClassMember::AutoAccessor(a) => {
            legacy::ClassMember::AutoAccessor(legacy::AutoAccessor {
                span: a.span(ast),
                key: match a.key(ast) {
                    experimental::Key::Private(p) => legacy::Key::Private(legacy::PrivateName {
                        span: p.span(ast),
                        name: transform_utf8_ref(ast, p.name(ast)),
                    }),
                    experimental::Key::Public(n) => {
                        legacy::Key::Public(transform_prop_name(ast, n))
                    }
                },
                value: a.value(ast).map(|e| Box::new(transform_expr(ast, e))),
                type_ann: None,
                is_static: a.is_static(ast),
                decorators: transform_type_sub_range(ast, a.decorators(ast), transform_decorator),
                accessibility: None,
                is_abstract: false,
                is_override: false,
                definite: false,
            })
        }
    }
}

fn transform_param_or_ts_param_prop(
    ast: &Ast,
    p: experimental::ParamOrTsParamProp,
) -> legacy::ParamOrTsParamProp {
    match p {
        experimental::ParamOrTsParamProp::Param(pp) => {
            legacy::ParamOrTsParamProp::Param(transform_param(ast, pp))
        }
    }
}

// -------------------------------------------------------------------------------
// Patterns and declarations

fn transform_decl(ast: &Ast, d: experimental::Decl) -> legacy::Decl {
    match d {
        experimental::Decl::Class(c) => legacy::Decl::Class(legacy::ClassDecl {
            ident: transform_ident(ast, c.ident(ast)),
            declare: c.declare(ast),
            class: Box::new(transform_class(ast, c.class(ast))),
        }),
        experimental::Decl::Fn(f) => legacy::Decl::Fn(legacy::FnDecl {
            ident: transform_ident(ast, f.ident(ast)),
            declare: f.declare(ast),
            function: Box::new(transform_function(ast, f.function(ast))),
        }),
        experimental::Decl::Var(v) => legacy::Decl::Var(Box::new(transform_var_decl(ast, v))),
        experimental::Decl::Using(u) => legacy::Decl::Using(Box::new(legacy::UsingDecl {
            span: u.span(ast),
            is_await: u.is_await(ast),
            decls: transform_type_sub_range(ast, u.decls(ast), transform_var_declarator),
        })),
    }
}

fn transform_var_declarator(ast: &Ast, d: experimental::VarDeclarator) -> legacy::VarDeclarator {
    legacy::VarDeclarator {
        span: d.span(ast),
        name: transform_pat(ast, d.name(ast)),
        init: d.init(ast).map(|e| Box::new(transform_expr(ast, e))),
        definite: false,
    }
}

fn transform_pat(ast: &Ast, p: experimental::Pat) -> legacy::Pat {
    match p {
        experimental::Pat::Ident(b) => legacy::Pat::Ident(legacy::BindingIdent {
            id: transform_ident(ast, b.id(ast)),
            type_ann: None,
        }),
        experimental::Pat::Array(a) => legacy::Pat::Array(legacy::ArrayPat {
            span: a.span(ast),
            elems: transform_opt_type_sub_range(ast, a.elems(ast), transform_pat),
            optional: a.optional(ast),
            type_ann: None,
        }),
        experimental::Pat::Rest(r) => legacy::Pat::Rest(legacy::RestPat {
            span: r.span(ast),
            dot3_token: r.dot_3_token(ast),
            arg: Box::new(transform_pat(ast, r.arg(ast))),
            type_ann: None,
        }),
        experimental::Pat::Object(o) => legacy::Pat::Object(legacy::ObjectPat {
            span: o.span(ast),
            props: transform_type_sub_range(ast, o.props(ast), transform_object_pat_prop),
            optional: o.optional(ast),
            type_ann: None,
        }),
        experimental::Pat::Assign(a) => legacy::Pat::Assign(legacy::AssignPat {
            span: a.span(ast),
            left: Box::new(transform_pat(ast, a.left(ast))),
            right: Box::new(transform_expr(ast, a.right(ast))),
        }),
        experimental::Pat::Invalid(i) => {
            legacy::Pat::Invalid(legacy::Invalid { span: i.span(ast) })
        }
        experimental::Pat::Expr(e) => legacy::Pat::Expr(Box::new(transform_expr(ast, e))),
    }
}

fn transform_object_pat_prop(ast: &Ast, p: experimental::ObjectPatProp) -> legacy::ObjectPatProp {
    match p {
        experimental::ObjectPatProp::KeyValue(kv) => {
            legacy::ObjectPatProp::KeyValue(legacy::KeyValuePatProp {
                key: transform_prop_name(ast, kv.key(ast)),
                value: Box::new(transform_pat(ast, kv.value(ast))),
            })
        }
        experimental::ObjectPatProp::Assign(ap) => {
            legacy::ObjectPatProp::Assign(legacy::AssignPatProp {
                span: ap.span(ast),
                key: legacy::BindingIdent {
                    id: transform_ident(ast, ap.key(ast).id(ast)),
                    type_ann: None,
                },
                value: ap.value(ast).map(|e| Box::new(transform_expr(ast, e))),
            })
        }
        experimental::ObjectPatProp::Rest(r) => legacy::ObjectPatProp::Rest(legacy::RestPat {
            span: r.span(ast),
            dot3_token: r.dot_3_token(ast),
            arg: Box::new(transform_pat(ast, r.arg(ast))),
            type_ann: None,
        }),
    }
}
fn transform_expr_or_spread(ast: &Ast, e: experimental::ExprOrSpread) -> legacy::ExprOrSpread {
    legacy::ExprOrSpread {
        spread: e.spread(ast).map(|s| s.span(ast)),
        expr: Box::new(transform_expr(ast, e.expr(ast))),
    }
}

fn transform_member_prop(ast: &Ast, p: experimental::MemberProp) -> legacy::MemberProp {
    match p {
        experimental::MemberProp::Ident(i) => legacy::MemberProp::Ident(legacy::IdentName {
            span: i.span(ast),
            sym: transform_utf8_ref(ast, i.sym(ast)),
        }),
        experimental::MemberProp::PrivateName(pn) => {
            legacy::MemberProp::PrivateName(legacy::PrivateName {
                span: pn.span(ast),
                name: transform_utf8_ref(ast, pn.name(ast)),
            })
        }
        experimental::MemberProp::Computed(c) => {
            legacy::MemberProp::Computed(legacy::ComputedPropName {
                span: c.span(ast),
                expr: Box::new(transform_expr(ast, c.expr(ast))),
            })
        }
    }
}

fn transform_lit(ast: &Ast, l: experimental::Lit) -> legacy::Lit {
    match l {
        experimental::Lit::Str(s) => legacy::Lit::Str(transform_str(ast, s)),
        experimental::Lit::Bool(b) => legacy::Lit::Bool(legacy::Bool {
            span: b.span(ast),
            value: b.value(ast),
        }),
        experimental::Lit::Null(n) => legacy::Lit::Null(legacy::Null { span: n.span(ast) }),
        experimental::Lit::Num(n) => legacy::Lit::Num(legacy::Number {
            span: n.span(ast),
            value: n.value(ast),
            raw: transform_opt_utf8_ref(ast, n.raw(ast)),
        }),
        experimental::Lit::BigInt(b) => legacy::Lit::Num(legacy::Number {
            span: b.span(ast),
            value: 0.0,
            raw: None,
        }),
        experimental::Lit::Regex(r) => legacy::Lit::Regex(legacy::Regex {
            span: r.span(ast),
            exp: transform_utf8_ref(ast, r.exp(ast)),
            flags: transform_utf8_ref(ast, r.flags(ast)),
        }),
    }
}

fn transform_tpl_element(ast: &Ast, e: experimental::TplElement) -> legacy::TplElement {
    legacy::TplElement {
        span: e.span(ast),
        tail: e.tail(ast),
        cooked: transform_opt_wtf8_ref(ast, e.cooked(ast)),
        raw: transform_utf8_ref(ast, e.raw(ast)),
    }
}

fn transform_tpl(ast: &Ast, t: experimental::Tpl) -> legacy::Tpl {
    legacy::Tpl {
        span: t.span(ast),
        exprs: transform_type_sub_range(ast, t.exprs(ast), |ast, e| {
            Box::new(transform_expr(ast, e))
        }),
        quasis: transform_type_sub_range(ast, t.quasis(ast), transform_tpl_element),
    }
}

fn transform_assign_target(ast: &Ast, t: experimental::AssignTarget) -> legacy::AssignTarget {
    match t {
        experimental::AssignTarget::Simple(s) => legacy::AssignTarget::Simple(match s {
            experimental::SimpleAssignTarget::Ident(b) => {
                legacy::SimpleAssignTarget::Ident(legacy::BindingIdent {
                    id: transform_ident(ast, b.id(ast)),
                    type_ann: None,
                })
            }
            experimental::SimpleAssignTarget::Member(m) => {
                legacy::SimpleAssignTarget::Member(legacy::MemberExpr {
                    span: m.span(ast),
                    obj: Box::new(transform_expr(ast, m.obj(ast))),
                    prop: transform_member_prop(ast, m.prop(ast)),
                })
            }
            experimental::SimpleAssignTarget::SuperProp(su) => {
                legacy::SimpleAssignTarget::SuperProp(legacy::SuperPropExpr {
                    span: su.span(ast),
                    obj: legacy::Super {
                        span: su.obj(ast).span(ast),
                    },
                    prop: match su.prop(ast) {
                        experimental::SuperProp::Ident(i) => {
                            legacy::SuperProp::Ident(legacy::IdentName {
                                span: i.span(ast),
                                sym: transform_utf8_ref(ast, i.sym(ast)),
                            })
                        }
                        experimental::SuperProp::Computed(c) => {
                            legacy::SuperProp::Computed(legacy::ComputedPropName {
                                span: c.span(ast),
                                expr: Box::new(transform_expr(ast, c.expr(ast))),
                            })
                        }
                    },
                })
            }
            experimental::SimpleAssignTarget::Paren(p) => {
                legacy::SimpleAssignTarget::Paren(legacy::ParenExpr {
                    span: p.span(ast),
                    expr: Box::new(transform_expr(ast, p.expr(ast))),
                })
            }
            experimental::SimpleAssignTarget::OptChain(o) => {
                legacy::SimpleAssignTarget::OptChain(legacy::OptChainExpr {
                    span: o.span(ast),
                    optional: o.optional(ast),
                    base: Box::new(match o.base(ast) {
                        experimental::OptChainBase::Member(m) => {
                            legacy::OptChainBase::Member(legacy::MemberExpr {
                                span: m.span(ast),
                                obj: Box::new(transform_expr(ast, m.obj(ast))),
                                prop: transform_member_prop(ast, m.prop(ast)),
                            })
                        }
                        experimental::OptChainBase::Call(c) => {
                            legacy::OptChainBase::Call(legacy::OptCall {
                                span: c.span(ast),
                                ctxt: Default::default(),
                                callee: Box::new(transform_expr(ast, c.callee(ast))),
                                args: transform_type_sub_range(
                                    ast,
                                    c.args(ast),
                                    transform_expr_or_spread,
                                ),
                                type_args: None,
                            })
                        }
                    }),
                })
            }
            experimental::SimpleAssignTarget::Invalid(i) => {
                legacy::SimpleAssignTarget::Invalid(legacy::Invalid { span: i.span(ast) })
            }
        }),
        experimental::AssignTarget::Pat(p) => legacy::AssignTarget::Pat(match p {
            experimental::AssignTargetPat::Array(a) => {
                legacy::AssignTargetPat::Array(legacy::ArrayPat {
                    span: a.span(ast),
                    elems: transform_opt_type_sub_range(ast, a.elems(ast), transform_pat),
                    optional: false,
                    type_ann: None,
                })
            }
            experimental::AssignTargetPat::Object(o) => {
                legacy::AssignTargetPat::Object(legacy::ObjectPat {
                    span: o.span(ast),
                    props: transform_type_sub_range(ast, o.props(ast), transform_object_pat_prop),
                    optional: false,
                    type_ann: None,
                })
            }
            experimental::AssignTargetPat::Invalid(i) => {
                legacy::AssignTargetPat::Invalid(legacy::Invalid { span: i.span(ast) })
            }
        }),
    }
}

// ===============================================================================

fn transform_utf8_ref(ast: &Ast, utf8_ref: Utf8Ref) -> Atom {
    Atom::new(ast.get_utf8(utf8_ref))
}

fn transform_opt_utf8_ref(ast: &Ast, utf8_ref: OptionalUtf8Ref) -> Option<Atom> {
    utf8_ref.map(|utf8_ref| transform_utf8_ref(ast, utf8_ref))
}

fn transform_wtf8_ref(ast: &Ast, wtf8_ref: Wtf8Ref) -> Wtf8Atom {
    Wtf8Atom::new(ast.get_wtf8(wtf8_ref))
}

fn transform_opt_wtf8_ref(ast: &Ast, wtf8_ref: OptionalWtf8Ref) -> Option<Wtf8Atom> {
    wtf8_ref
        .to_option()
        .map(|wtf8_ref| transform_wtf8_ref(ast, wtf8_ref))
}

fn transform_type_sub_range<T: FromNodeId, U, F: Fn(&Ast, T) -> U>(
    ast: &Ast,
    typed_range: TypedSubRange<T>,
    transformer: F,
) -> Vec<U> {
    let mut ret = Vec::with_capacity(typed_range.len());
    for item in typed_range.iter() {
        ret.push(transformer(ast, ast.get_node_in_sub_range(item)));
    }
    ret
}

fn transform_opt_type_sub_range<T: FromNodeId, U, F: Fn(&Ast, T) -> U>(
    ast: &Ast,
    typed_range: TypedSubRange<Option<T>>,
    transformer: F,
) -> Vec<Option<U>> {
    let mut ret = Vec::with_capacity(typed_range.len());
    for item in typed_range.iter() {
        let v = ast
            .get_opt_node_in_sub_range(item)
            .map(|n| transformer(ast, n));
        ret.push(v);
    }
    ret
}
