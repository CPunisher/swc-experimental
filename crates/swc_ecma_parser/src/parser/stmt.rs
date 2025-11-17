use super::*;
use crate::parser::{Parser, pat::PatType, util::FromStmt};

#[allow(clippy::enum_variant_names)]
enum TempForHead {
    For {
        init: Option<VarDeclOrExpr>,
        test: Option<Expr>,
        update: Option<Expr>,
    },
    ForIn {
        left: ForHead,
        right: Expr,
    },
    ForOf {
        left: ForHead,
        right: Expr,
    },
}

impl<I: Tokens> Parser<I> {
    fn parse_normal_for_head(&mut self, init: Option<VarDeclOrExpr>) -> PResult<TempForHead> {
        let test = if self.input_mut().eat(Token::Semi) {
            None
        } else {
            let test = self.allow_in_expr(|p| p.parse_expr()).map(Some)?;
            self.input_mut().eat(Token::Semi);
            test
        };

        let update = if self.input().is(Token::RParen) {
            None
        } else {
            self.allow_in_expr(|p| p.parse_expr()).map(Some)?
        };

        Ok(TempForHead::For { init, test, update })
    }

    fn parse_for_each_head(&mut self, left: ForHead) -> PResult<TempForHead> {
        let is_of = self.input().cur() == Token::Of;
        self.bump();
        if is_of {
            let right = self.allow_in_expr(Self::parse_assignment_expr)?;
            Ok(TempForHead::ForOf { left, right })
        } else {
            if let ForHead::UsingDecl(d) = &left {
                self.emit_err(
                    d.span(&self.ast),
                    SyntaxError::UsingDeclNotAllowedForForInLoop,
                )
            }
            let right = self.allow_in_expr(|p| p.parse_expr())?;
            Ok(TempForHead::ForIn { left, right })
        }
    }

    fn parse_return_stmt(&mut self) -> PResult<Stmt> {
        let start = self.cur_pos();

        self.assert_and_bump(Token::Return);

        let arg = if self.is_general_semi() {
            None
        } else {
            self.allow_in_expr(|p| p.parse_expr()).map(Some)?
        };
        self.expect_general_semi()?;
        let stmt = self.ast.stmt_return_stmt(self.span(start), arg);

        if !self.ctx().contains(Context::InFunction)
            && !self.input().syntax().allow_return_outside_function()
        {
            self.emit_err(self.span(start), SyntaxError::ReturnNotAllowed);
        }

        Ok(stmt)
    }

    fn parse_var_declarator(
        &mut self,
        for_loop: bool,
        kind: VarDeclKind,
    ) -> PResult<VarDeclarator> {
        let start = self.cur_pos();

        let is_let_or_const = matches!(kind, VarDeclKind::Let | VarDeclKind::Const);

        let name = self.parse_binding_pat_or_ident(is_let_or_const)?;

        // let definite = if self.input().syntax().typescript() {
        //     match name {
        //         Pat::Ident(..) => self.input_mut().eat(Token::Bang),
        //         _ => false,
        //     }
        // } else {
        //     false
        // };

        // Typescript extension
        // if self.input().syntax().typescript() && self.input().is(Token::Colon) {
        //     let type_annotation = self.try_parse_ts_type_ann()?;
        //     match name {
        //         Pat::Array(ArrayPat {
        //             ref mut type_ann, ..
        //         })
        //         | Pat::Ident(BindingIdent {
        //             ref mut type_ann, ..
        //         })
        //         | Pat::Object(ObjectPat {
        //             ref mut type_ann, ..
        //         })
        //         | Pat::Rest(RestPat {
        //             ref mut type_ann, ..
        //         }) => {
        //             *type_ann = type_annotation;
        //         }
        //         _ => unreachable!("invalid syntax: Pat: {:?}", name),
        //     }
        // }

        //FIXME: This is wrong. Should check in/of only on first looself.
        let cur = self.input().cur();
        let init = if !for_loop || !(cur == Token::In || cur == Token::Of) {
            if self.input_mut().eat(Token::Eq) {
                let expr = self.parse_assignment_expr()?;
                let expr = self.verify_expr(expr)?;

                Some(expr)
            } else {
                // Destructuring bindings require initializers, but
                // typescript allows `declare` vars not to have initializers.
                if self.ctx().contains(Context::InDeclare) {
                    None
                } else if kind == VarDeclKind::Const
                    && !for_loop
                    && !self.ctx().contains(Context::InDeclare)
                {
                    self.emit_err(
                        self.span(start),
                        SyntaxError::ConstDeclarationsRequireInitialization,
                    );

                    None
                } else {
                    match name {
                        Pat::Ident(..) => None,
                        _ => {
                            syntax_error!(self, self.span(start), SyntaxError::PatVarWithoutInit)
                        }
                    }
                }
            }
        } else {
            // e.g. for(let a;;)
            None
        };

        Ok(self.ast.var_declarator(self.span(start), name, init))
    }

    pub(crate) fn parse_var_stmt(&mut self, for_loop: bool) -> PResult<VarDecl> {
        let start = self.cur_pos();
        let t = self.input().cur();
        let kind = if t == Token::Const {
            VarDeclKind::Const
        } else if t == Token::Let {
            VarDeclKind::Let
        } else if t == Token::Var {
            VarDeclKind::Var
        } else {
            unreachable!()
        };
        self.bump();
        let var_span = self.span(start);
        let should_include_in = kind != VarDeclKind::Var || !for_loop;

        // if self.syntax().typescript() && for_loop {
        //     let cur = self.input().cur();
        //     let res: PResult<bool> = if cur == Token::In || cur == Token::Of {
        //         self.ts_look_ahead(|p| {
        //             //
        //             if !p.input_mut().eat(Token::Of) && !p.input_mut().eat(Token::In) {
        //                 return Ok(false);
        //             }

        //             p.parse_assignment_expr()?;
        //             expect!(p, Token::RParen);

        //             Ok(true)
        //         })
        //     } else {
        //         Ok(false)
        //     };

        //     match res {
        //         Ok(true) => {
        //             let pos = var_span.hi();
        //             let span = Span::new_with_checked(pos, pos);
        //             self.emit_err(span, SyntaxError::TS1123);

        //             return Ok(Box::new(VarDecl {
        //                 span: self.span(start),
        //                 kind,
        //                 declare: false,
        //                 decls: Vec::new(),
        //                 ..Default::default()
        //             }));
        //         }
        //         Err(..) => {}
        //         _ => {}
        //     }
        // }

        let mut decls = Vec::with_capacity(4);
        loop {
            // Handle
            //      var a,;
            //
            // NewLine is ok
            if self.input().is(Token::Semi) {
                let prev_span = self.input().prev_span();
                let span = if prev_span == var_span {
                    Span::new_with_checked(prev_span.hi, prev_span.hi)
                } else {
                    prev_span
                };
                self.emit_err(span, SyntaxError::TS1009);
                break;
            }

            let decl = if should_include_in {
                self.do_inside_of_context(Context::IncludeInExpr, |p| {
                    p.parse_var_declarator(for_loop, kind)
                })
            } else {
                self.parse_var_declarator(for_loop, kind)
            }?;

            decls.push(decl);

            if !self.input_mut().eat(Token::Comma) {
                break;
            }
        }

        if !for_loop && !self.eat_general_semi() {
            self.emit_err(self.input().cur_span(), SyntaxError::TS1005);

            let _ = self.parse_expr();

            while !self.eat_general_semi() {
                self.bump();

                if self.input().cur() == Token::Error {
                    break;
                }
            }
        }

        let decls = self.ast.add_typed_sub_range(&decls);
        Ok(self.ast.var_decl(self.span(start), kind, false, decls))
    }

    fn parse_using_decl(&mut self, start: BytePos, is_await: bool) -> PResult<Option<UsingDecl>> {
        // using
        // reader = init()

        // is two statements
        if self.input_mut().has_linebreak_between_cur_and_peeked() {
            return Ok(None);
        }

        if !self.peek_is_ident_ref() {
            return Ok(None);
        }

        self.assert_and_bump(Token::Using);

        let mut decls = Vec::new();
        loop {
            // Handle
            //      var a,;
            //
            // NewLine is ok
            if self.input().is(Token::Semi) {
                let span = self.input().prev_span();
                self.emit_err(span, SyntaxError::TS1009);
                break;
            }

            decls.push(self.parse_var_declarator(false, VarDeclKind::Var)?);
            if !self.input_mut().eat(Token::Comma) {
                break;
            }
        }

        if !self.syntax().explicit_resource_management() {
            self.emit_err(self.span(start), SyntaxError::UsingDeclNotEnabled);
        }

        if !self.ctx().contains(Context::AllowUsingDecl) {
            self.emit_err(self.span(start), SyntaxError::UsingDeclNotAllowed);
        }

        for decl in &decls {
            match decl.name(&self.ast) {
                Pat::Ident(..) => {}
                _ => {
                    self.emit_err(self.span(start), SyntaxError::InvalidNameInUsingDecl);
                }
            }

            if decl.init(&self.ast).is_none() {
                self.emit_err(self.span(start), SyntaxError::InitRequiredForUsingDecl);
            }
        }

        self.expect_general_semi()?;

        let decls = self.ast.add_typed_sub_range(&decls);
        Ok(Some(self.ast.using_decl(self.span(start), is_await, decls)))
    }

    fn parse_for_head(&mut self) -> PResult<TempForHead> {
        // let strict = self.ctx().contains(Context::Strict);

        let cur = self.input().cur();
        if cur == Token::Const
            || cur == Token::Var
            || (self.input().is(Token::Let)
                && peek!(self).map_or(false, |v| v.follows_keyword_let()))
        {
            let decl = self.parse_var_stmt(true)?;

            let cur = self.input().cur();
            if cur == Token::Of || cur == Token::In {
                if decl.decls(&self.ast).len() != 1 {
                    for d in decl.decls(&self.ast).iter().skip(1) {
                        let d = self.ast.get_node(d);
                        self.emit_err(
                            d.name(&self.ast).span(&self.ast),
                            SyntaxError::TooManyVarInForInHead,
                        );
                    }
                } else {
                    // if (self.ctx().contains(Context::Strict) || self.input().is(Token::Of))
                    //     && decl.decls[0].init.is_some()
                    // {
                    //     self.emit_err(
                    //         decl.decls[0].name.span(),
                    //         SyntaxError::VarInitializerInForInHead,
                    //     );
                    // }

                    // if self.syntax().typescript() {
                    //     let type_ann = match decl.decls[0].name {
                    //         Pat::Ident(ref v) => Some(&v.type_ann),
                    //         Pat::Array(ref v) => Some(&v.type_ann),
                    //         Pat::Rest(ref v) => Some(&v.type_ann),
                    //         Pat::Object(ref v) => Some(&v.type_ann),
                    //         _ => None,
                    //     };

                    //     if let Some(type_ann) = type_ann {
                    //         if type_ann.is_some() {
                    //             self.emit_err(decl.decls[0].name.span(), SyntaxError::TS2483);
                    //         }
                    //     }
                    // }
                }

                return self.parse_for_each_head(ForHead::VarDecl(decl));
            }

            expect!(self, Token::Semi);
            return self.parse_normal_for_head(Some(VarDeclOrExpr::VarDecl(decl)));
        }

        if self.input_mut().eat(Token::Semi) {
            return self.parse_normal_for_head(None);
        }

        let start = self.cur_pos();
        let init = self.disallow_in_expr(Self::parse_for_head_prefix)?;

        let mut is_using_decl = false;
        let mut is_await_using_decl = false;

        if self.input().syntax().explicit_resource_management() {
            // using foo
            let mut maybe_using_decl = init.is_ident_ref_to(&self.ast, "using");
            let mut maybe_await_using_decl = false;

            // await using foo
            if !maybe_using_decl
                && init
                    .as_await()
                    .filter(|e| e.arg(&self.ast).is_ident_ref_to(&self.ast, "using"))
                    .is_some()
            {
                maybe_using_decl = true;
                maybe_await_using_decl = true;
            }

            if maybe_using_decl
                && !self.input().is(Token::Of)
                && (peek!(self).is_some_and(|peek| peek == Token::Of || peek == Token::In))
            {
                is_using_decl = maybe_using_decl;
                is_await_using_decl = maybe_await_using_decl;
            }
        }

        if is_using_decl {
            let name = self.parse_binding_ident(false)?;
            let decl = self
                .ast
                .var_declarator(self.span(start), Pat::Ident(name), Some(init));

            let decls = self.ast.add_typed_sub_range(&[decl]);
            let pat = self
                .ast
                .using_decl(self.span(start), is_await_using_decl, decls);

            let cur = self.input().cur();
            if cur == Token::Error {
                let err = self.input_mut().expect_error_token_and_bump();
                return Err(err);
            } else if cur == Token::Eof {
                return Err(self.eof_error());
            }

            return self.parse_for_each_head(ForHead::UsingDecl(pat));
        }

        // for (a of b)
        let cur = self.input().cur();
        if cur == Token::Of || cur == Token::In {
            let is_in = self.input().is(Token::In);

            let pat = self.reparse_expr_as_pat(PatType::AssignPat, init)?;

            // for ({} in foo) is invalid
            if self.input().syntax().typescript() && is_in {
                match pat {
                    Pat::Ident(..) => {}
                    Pat::Expr(..) => {}
                    ref v => self.emit_err(v.span(&self.ast), SyntaxError::TS2491),
                }
            }

            return self.parse_for_each_head(ForHead::Pat(pat));
        }

        expect!(self, Token::Semi);

        let init = self.verify_expr(init)?;
        self.parse_normal_for_head(Some(VarDeclOrExpr::Expr(init)))
    }

    fn parse_for_stmt(&mut self) -> PResult<Stmt> {
        let start = self.cur_pos();

        self.assert_and_bump(Token::For);
        let await_start = self.cur_pos();
        let await_token = if self.input_mut().eat(Token::Await) {
            Some(self.span(await_start))
        } else {
            None
        };
        expect!(self, Token::LParen);

        let head = self.do_inside_of_context(Context::ForLoopInit, |p| {
            if await_token.is_some() {
                p.do_inside_of_context(Context::ForAwaitLoopInit, Self::parse_for_head)
            } else {
                p.do_outside_of_context(Context::ForAwaitLoopInit, Self::parse_for_head)
            }
        })?;

        expect!(self, Token::RParen);

        let body = self.do_inside_of_context(
            Context::IsBreakAllowed.union(Context::IsContinueAllowed),
            |p| p.do_outside_of_context(Context::TopLevel, Self::parse_stmt),
        )?;

        let span = self.span(start);
        Ok(match head {
            TempForHead::For { init, test, update } => {
                if let Some(await_token) = await_token {
                    syntax_error!(self, await_token, SyntaxError::AwaitForStmt);
                }

                self.ast.stmt_for_stmt(span, init, test, update, body)
            }
            TempForHead::ForIn { left, right } => {
                if let Some(await_token) = await_token {
                    syntax_error!(self, await_token, SyntaxError::AwaitForStmt);
                }

                self.ast.stmt_for_in_stmt(span, left, right, body)
            }
            TempForHead::ForOf { left, right } => {
                self.ast
                    .stmt_for_of_stmt(span, await_token.is_some(), left, right, body)
            }
        })
    }

    pub fn parse_stmt(&mut self) -> PResult<Stmt> {
        trace_cur!(self, parse_stmt);
        self.parse_stmt_like(false, handle_import_export)
    }

    /// Utility function used to parse large if else statements iteratively.
    ///
    /// THis function is recursive, but it is very cheap so stack overflow will
    /// not occur.
    fn adjust_if_else_clause(&mut self, cur: IfStmt, alt: Stmt) {
        let span = self.span(cur.span_lo(&self.ast));
        cur.set_span(&mut self.ast, span);

        if let Some(Stmt::If(prev_alt)) = cur.alt(&self.ast) {
            self.adjust_if_else_clause(prev_alt, alt)
        } else {
            debug_assert!(cur.alt(&self.ast).is_none());
            cur.set_alt(&mut self.ast, Some(alt));
        }
    }

    fn parse_if_stmt(&mut self) -> PResult<IfStmt> {
        let start = self.cur_pos();

        self.assert_and_bump(Token::If);
        let if_token = self.input().prev_span();

        expect!(self, Token::LParen);

        let test = self
            .do_outside_of_context(Context::IgnoreElseClause, |p| {
                p.allow_in_expr(|p| p.parse_expr())
            })
            .map_err(|err| {
                Error::new(
                    err.span(),
                    SyntaxError::WithLabel {
                        inner: Box::new(err),
                        span: if_token,
                        note: "Tried to parse the condition for an if statement",
                    },
                )
            })?;

        expect!(self, Token::RParen);

        let cons = {
            // Prevent stack overflow
            crate::maybe_grow(256 * 1024, 1024 * 1024, || {
                // // Annex B
                // if !self.ctx().contains(Context::Strict) && self.input().is(Token::FUNCTION)
                // {     // TODO: report error?
                // }
                self.do_outside_of_context(
                    Context::IgnoreElseClause.union(Context::TopLevel),
                    Self::parse_stmt,
                )
            })?
        };

        // We parse `else` branch iteratively, to avoid stack overflow
        // See https://github.com/swc-project/swc/pull/3961

        let alt = if self.ctx().contains(Context::IgnoreElseClause) {
            None
        } else {
            let mut cur = None;

            let last = loop {
                if !self.input_mut().eat(Token::Else) {
                    break None;
                }

                if !self.input().is(Token::If) {
                    // As we eat `else` above, we need to parse statement once.
                    let last = self.do_outside_of_context(
                        Context::IgnoreElseClause.union(Context::TopLevel),
                        Self::parse_stmt,
                    )?;
                    break Some(last);
                }

                // We encountered `else if`

                let alt =
                    self.do_inside_of_context(Context::IgnoreElseClause, Self::parse_if_stmt)?;

                match cur {
                    Some(cur) => {
                        self.adjust_if_else_clause(cur, Stmt::If(alt));
                    }
                    _ => {
                        cur = Some(alt);
                    }
                }
            };

            match cur {
                Some(cur) => {
                    if let Some(last) = last {
                        self.adjust_if_else_clause(cur, last);
                    }
                    Some(Stmt::If(cur))
                }
                _ => last,
            }
        };

        let span = self.span(start);
        Ok(self.ast.if_stmt(span, test, cons, alt))
    }

    fn parse_throw_stmt(&mut self) -> PResult<Stmt> {
        let start = self.cur_pos();

        self.assert_and_bump(Token::Throw);

        if self.input().had_line_break_before_cur() {
            // TODO: Suggest throw arg;
            syntax_error!(self, SyntaxError::LineBreakInThrow);
        }

        let arg = self.allow_in_expr(|p| p.parse_expr())?;
        self.expect_general_semi()?;

        let span = self.span(start);
        Ok(self.ast.stmt_throw_stmt(span, arg))
    }

    fn parse_with_stmt(&mut self) -> PResult<Stmt> {
        if self.syntax().typescript() {
            let span = self.input().cur_span();
            self.emit_err(span, SyntaxError::TS2410);
        }

        {
            let span = self.input().cur_span();
            self.emit_strict_mode_err(span, SyntaxError::WithInStrict);
        }

        let start = self.cur_pos();

        self.assert_and_bump(Token::With);

        expect!(self, Token::LParen);
        let obj = self.allow_in_expr(|p| p.parse_expr())?;
        expect!(self, Token::RParen);

        let body = self.do_inside_of_context(Context::InFunction, |p| {
            p.do_outside_of_context(Context::TopLevel, Self::parse_stmt)
        })?;

        let span = self.span(start);
        Ok(self.ast.stmt_with_stmt(span, obj, body))
    }

    fn parse_while_stmt(&mut self) -> PResult<Stmt> {
        let start = self.cur_pos();

        self.assert_and_bump(Token::While);

        expect!(self, Token::LParen);
        let test = self.allow_in_expr(|p| p.parse_expr())?;
        expect!(self, Token::RParen);

        let body = self.do_inside_of_context(
            Context::IsBreakAllowed.union(Context::IsContinueAllowed),
            |p| p.do_outside_of_context(Context::TopLevel, Self::parse_stmt),
        )?;

        let span = self.span(start);
        Ok(self.ast.stmt_while_stmt(span, test, body))
    }

    /// It's optional since es2019
    fn parse_catch_param(&mut self) -> PResult<Option<Pat>> {
        if self.input_mut().eat(Token::LParen) {
            let pat = self.parse_binding_pat_or_ident(false)?;

            // let type_ann_start = self.cur_pos();
            // if self.syntax().typescript() && self.input_mut().eat(Token::Colon) {
            //     let ty = self.do_inside_of_context(Context::InType, Self::parse_ts_type)?;
            //     // self.emit_err(ty.span(), SyntaxError::TS1196);

            //     match &mut pat {
            //         Pat::Ident(BindingIdent { type_ann, .. })
            //         | Pat::Array(ArrayPat { type_ann, .. })
            //         | Pat::Rest(RestPat { type_ann, .. })
            //         | Pat::Object(ObjectPat { type_ann, .. }) => {
            //             *type_ann = Some(Box::new(TsTypeAnn {
            //                 span: self.span(type_ann_start),
            //                 type_ann: ty,
            //             }));
            //         }
            //         Pat::Assign(..) => {}
            //         Pat::Invalid(_) => {}
            //         Pat::Expr(_) => {}
            //         #[cfg(swc_ast_unknown)]
            //         _ => unreachable!(),
            //     }
            // }
            expect!(self, Token::RParen);
            Ok(Some(pat))
        } else {
            Ok(None)
        }
    }

    fn parse_do_stmt(&mut self) -> PResult<Stmt> {
        let start = self.cur_pos();

        self.assert_and_bump(Token::Do);

        let body = self.do_inside_of_context(
            Context::IsBreakAllowed.union(Context::IsContinueAllowed),
            |p| p.do_outside_of_context(Context::TopLevel, Self::parse_stmt),
        )?;

        expect!(self, Token::While);
        expect!(self, Token::LParen);

        let test = self.allow_in_expr(|p| p.parse_expr())?;

        expect!(self, Token::RParen);

        // We *may* eat semicolon.
        let _ = self.eat_general_semi();

        let span = self.span(start);

        Ok(self.ast.stmt_do_while_stmt(span, test, body))
    }

    fn parse_labelled_stmt(&mut self, l: Ident) -> PResult<Stmt> {
        self.do_inside_of_context(Context::IsBreakAllowed, |p| {
            p.do_outside_of_context(Context::AllowUsingDecl, |p| {
                let start = l.span_lo(&p.ast);
                let atom = p.ast.get_atom(l.sym(&p.ast)).clone();

                let mut errors = Vec::new();
                for lb in &p.state().labels {
                    if &atom == lb {
                        errors.push(Error::new(
                            l.span(&p.ast),
                            SyntaxError::DuplicateLabel(atom.clone()),
                        ));
                    }
                }
                p.state_mut().labels.push(atom.clone());

                let body = if p.input().is(Token::Function) {
                    let f = p.parse_fn_decl(Vec::new())?;
                    if let Decl::Fn(fn_decl) = &f {
                        let function = fn_decl.function(&p.ast);
                        if p.ctx().contains(Context::Strict) {
                            p.emit_err(function.span(&p.ast), SyntaxError::LabelledFunctionInStrict)
                        }
                        if function.is_generator(&p.ast) || function.is_async(&p.ast) {
                            p.emit_err(function.span(&p.ast), SyntaxError::LabelledGeneratorOrAsync)
                        }
                    }

                    Stmt::Decl(f)
                } else {
                    p.do_outside_of_context(Context::TopLevel, Self::parse_stmt)?
                };

                for err in errors {
                    p.emit_error(err);
                }

                {
                    let pos = p.state().labels.iter().position(|v| v == &atom);
                    if let Some(pos) = pos {
                        p.state_mut().labels.remove(pos);
                    }
                }

                Ok(p.ast.stmt_labeled_stmt(p.span(start), l, body))
            })
        })
    }

    pub(crate) fn parse_block(&mut self, allow_directives: bool) -> PResult<BlockStmt> {
        let start = self.cur_pos();

        expect!(self, Token::LBrace);

        let stmts = self.do_outside_of_context(Context::TopLevel, |p| {
            p.parse_stmt_block_body(allow_directives, Some(Token::RBrace))
        })?;

        let span = self.span(start);
        Ok(self.ast.block_stmt(span, stmts))
    }

    fn parse_finally_block(&mut self) -> PResult<Option<BlockStmt>> {
        Ok(if self.input_mut().eat(Token::Finally) {
            self.parse_block(false).map(Some)?
        } else {
            None
        })
    }

    fn parse_catch_clause(&mut self) -> PResult<Option<CatchClause>> {
        let start = self.cur_pos();
        Ok(if self.input_mut().eat(Token::Catch) {
            let param = self.parse_catch_param()?;
            self.parse_block(false)
                .map(|body| self.ast.catch_clause(self.span(start), param, body))
                .map(Some)?
        } else {
            None
        })
    }

    fn parse_try_stmt(&mut self) -> PResult<Stmt> {
        let start = self.cur_pos();
        self.assert_and_bump(Token::Try);

        let block = self.parse_block(false)?;

        let catch_start = self.cur_pos();
        let handler = self.parse_catch_clause()?;
        let finalizer = self.parse_finally_block()?;

        if handler.is_none() && finalizer.is_none() {
            self.emit_err(
                Span::new_with_checked(catch_start, catch_start),
                SyntaxError::TS1005,
            );
        }

        let span = self.span(start);
        Ok(self.ast.stmt_try_stmt(span, block, handler, finalizer))
    }

    fn parse_switch_stmt(&mut self) -> PResult<Stmt> {
        let switch_start = self.cur_pos();

        self.assert_and_bump(Token::Switch);

        expect!(self, Token::LParen);
        let discriminant = self.allow_in_expr(|p| p.parse_expr())?;
        expect!(self, Token::RParen);

        let mut cases = Vec::new();
        let mut span_of_previous_default = None;

        expect!(self, Token::LBrace);

        self.do_inside_of_context(Context::IsBreakAllowed, |p| {
            while {
                let cur = p.input().cur();
                cur == Token::Case || cur == Token::Default
            } {
                let mut cons = Vec::new();
                let is_case = p.input().is(Token::Case);
                let case_start = p.cur_pos();
                p.bump();
                let test = if is_case {
                    p.allow_in_expr(|p| p.parse_expr()).map(Some)?
                } else {
                    if let Some(previous) = span_of_previous_default {
                        syntax_error!(p, SyntaxError::MultipleDefault { previous });
                    }
                    span_of_previous_default = Some(p.span(case_start));

                    None
                };
                expect!(p, Token::Colon);

                while {
                    let cur = p.input().cur();
                    !(cur == Token::Case || cur == Token::Default || cur == Token::RBrace)
                } {
                    cons.push(
                        p.do_outside_of_context(Context::TopLevel, Self::parse_stmt_list_item)?,
                    );
                }

                let cons = p.ast.add_typed_sub_range(&cons);
                cases.push(p.ast.switch_case(
                    Span::new_with_checked(case_start, p.input().prev_span().hi),
                    test,
                    cons,
                ));
            }

            Ok(())
        })?;

        // eof or rbrace
        expect!(self, Token::RBrace);

        let cases = self.ast.add_typed_sub_range(&cases);
        Ok(self
            .ast
            .stmt_switch_stmt(self.span(switch_start), discriminant, cases))
    }

    /// Parse a statement and maybe a declaration.
    pub fn parse_stmt_list_item(&mut self) -> PResult<Stmt> {
        trace_cur!(self, parse_stmt_list_item);
        self.parse_stmt_like(true, handle_import_export)
    }

    /// Parse a statement, declaration or module item.
    #[inline(always)]
    pub(crate) fn parse_stmt_like<Type: FromStmt>(
        &mut self,
        include_decl: bool,
        handle_import_export: impl Fn(&mut Self, Vec<Decorator>) -> PResult<Type>,
    ) -> PResult<Type> {
        trace_cur!(self, parse_stmt_like);

        debug_tracing!(self, "parse_stmt_like");

        let start = self.cur_pos();
        let decorators = if self.input().get_cur().token == Token::At {
            self.parse_decorators(true)?
        } else {
            vec![]
        };

        let cur = self.input().cur();
        if cur == Token::Import || cur == Token::Export {
            return handle_import_export(self, decorators);
        }

        self.do_outside_of_context(Context::WillExpectColonForCond, |p| {
            p.do_inside_of_context(Context::AllowUsingDecl, |p| {
                p.parse_stmt_internal(start, include_decl, decorators)
            })
        })
        .map(Type::from_stmt)
    }

    /// `parseStatementContent`
    fn parse_stmt_internal(
        &mut self,
        start: BytePos,
        include_decl: bool,
        decorators: Vec<Decorator>,
    ) -> PResult<Stmt> {
        trace_cur!(self, parse_stmt_internal);

        // let is_typescript = self.input().syntax().typescript();
        // if is_typescript
        //     && self.input().is(Token::Const)
        //     && peek!(self).is_some_and(|peek| peek == Token::Enum)
        // {
        //     self.assert_and_bump(Token::Const);
        //     self.assert_and_bump(Token::Enum);
        //     return self
        //         .parse_ts_enum_decl(start, true)
        //         .map(Decl::from)
        //         .map(Stmt::from);
        // }

        let top_level = self.ctx().contains(Context::TopLevel);

        let cur = self.input().cur();

        if cur == Token::Await && (include_decl || top_level) {
            if top_level {
                self.mark_found_module_item();
                if !self.ctx().contains(Context::CanBeModule) {
                    self.emit_err(self.input().cur_span(), SyntaxError::TopLevelAwaitInScript);
                }
            }

            if peek!(self).is_some_and(|peek| peek == Token::Using) {
                let eaten_await = Some(self.input().cur_pos());
                self.assert_and_bump(Token::Await);
                let v = self.parse_using_decl(start, true)?;
                if let Some(v) = v {
                    return Ok(Stmt::Decl(Decl::Using(v)));
                }

                let expr = self.parse_await_expr(eaten_await)?;
                let expr = self.allow_in_expr(|p| p.parse_bin_op_recursively(expr, 0))?;
                self.eat_general_semi();

                let span = self.span(start);
                return Ok(self.ast.stmt_expr_stmt(span, expr));
            }
        } else if cur == Token::Break || cur == Token::Continue {
            let is_break = self.input().is(Token::Break);
            self.bump();
            let label = if self.eat_general_semi() {
                None
            } else {
                let i = self.parse_label_ident().map(Some)?;
                self.expect_general_semi()?;
                i
            };
            let span = self.span(start);
            if is_break {
                if label.is_some()
                    && !self
                        .state()
                        .labels
                        .contains(self.ast.get_atom(label.as_ref().unwrap().sym(&self.ast)))
                {
                    self.emit_err(span, SyntaxError::TS1116);
                } else if !self.ctx().contains(Context::IsBreakAllowed) {
                    self.emit_err(span, SyntaxError::TS1105);
                }
            } else if !self.ctx().contains(Context::IsContinueAllowed) {
                self.emit_err(span, SyntaxError::TS1115);
            } else if label.is_some()
                && !self
                    .state()
                    .labels
                    .contains(self.ast.get_atom(label.as_ref().unwrap().sym(&self.ast)))
            {
                self.emit_err(span, SyntaxError::TS1107);
            }
            return Ok(if is_break {
                self.ast.stmt_break_stmt(span, label)
            } else {
                self.ast.stmt_continue_stmt(span, label)
            });
        } else if cur == Token::Debugger {
            self.bump();
            self.expect_general_semi()?;
            return Ok(self.ast.stmt_debugger_stmt(self.span(start)));
        } else if cur == Token::Do {
            return self.parse_do_stmt();
        } else if cur == Token::For {
            return self.parse_for_stmt();
        } else if cur == Token::Function {
            if !include_decl {
                self.emit_err(self.input().cur_span(), SyntaxError::DeclNotAllowed);
            }
            return self.parse_fn_decl(decorators).map(Stmt::Decl);
        } else if cur == Token::Class {
            if !include_decl {
                self.emit_err(self.input().cur_span(), SyntaxError::DeclNotAllowed);
            }
            return self
                .parse_class_decl(start, start, decorators, false)
                .map(Stmt::Decl);
        } else if cur == Token::If {
            return self.parse_if_stmt().map(Stmt::If);
        } else if cur == Token::Return {
            return self.parse_return_stmt();
        } else if cur == Token::Switch {
            return self.parse_switch_stmt();
        } else if cur == Token::Throw {
            return self.parse_throw_stmt();
        } else if cur == Token::Catch {
            // Error recovery
            let span = self.input().cur_span();
            self.emit_err(span, SyntaxError::TS1005);

            let _ = self.parse_catch_clause();
            let _ = self.parse_finally_block();

            let invalid = self.ast.expr_invalid(span);
            return Ok(self.ast.stmt_expr_stmt(span, invalid));
        } else if cur == Token::Finally {
            // Error recovery
            let span = self.input().cur_span();
            self.emit_err(span, SyntaxError::TS1005);

            let _ = self.parse_finally_block();

            let invalud = self.ast.expr_invalid(span);
            return Ok(self.ast.stmt_expr_stmt(span, invalud));
        } else if cur == Token::Try {
            return self.parse_try_stmt();
        } else if cur == Token::With {
            return self.parse_with_stmt();
        } else if cur == Token::While {
            return self.parse_while_stmt();
        } else if cur == Token::Var || (cur == Token::Const && include_decl) {
            let v = self.parse_var_stmt(false)?;
            return Ok(Stmt::Decl(Decl::Var(v)));
        } else if cur == Token::Let && include_decl {
            // 'let' can start an identifier reference.
            let is_keyword = match peek!(self) {
                Some(t) => t.follows_keyword_let(),
                _ => false,
            };

            if is_keyword {
                let v = self.parse_var_stmt(false)?;
                return Ok(Stmt::Decl(Decl::Var(v)));
            }
        } else if cur == Token::Using && include_decl {
            let v = self.parse_using_decl(start, false)?;
            if let Some(v) = v {
                return Ok(Stmt::Decl(Decl::Using(v)));
            }
        // } else if cur == Token::Interface
        //     && is_typescript
        //     && peek!(self).is_some_and(|peek| peek.is_word())
        //     && !self.input_mut().has_linebreak_between_cur_and_peeked()
        // {
        //     let start = self.input().cur_pos();
        //     self.bump();
        //     return Ok(self.parse_ts_interface_decl(start)?.into());
        // } else if cur == Token::Type
        //     && is_typescript
        //     && peek!(self).is_some_and(|peek| peek.is_word())
        //     && !self.input_mut().has_linebreak_between_cur_and_peeked()
        // {
        //     let start = self.input().cur_pos();
        //     self.bump();
        //     return Ok(self.parse_ts_type_alias_decl(start)?.into());
        // } else if cur == Token::Enum
        //     && is_typescript
        //     && peek!(self).is_some_and(|peek| peek.is_word())
        //     && !self.input_mut().has_linebreak_between_cur_and_peeked()
        // {
        //     let start = self.input().cur_pos();
        //     self.bump();
        //     return Ok(self.parse_ts_enum_decl(start, false)?.into());
        } else if cur == Token::LBrace {
            return self
                .do_inside_of_context(Context::AllowUsingDecl, |p| p.parse_block(false))
                .map(Stmt::Block);
        } else if cur == Token::Semi {
            self.bump();
            return Ok(self.ast.stmt_empty_stmt(self.span(start)));
        }

        // Handle async function foo() {}
        if self.input().is(Token::Async)
            && peek!(self).is_some_and(|peek| peek == Token::Function)
            && !self.input_mut().has_linebreak_between_cur_and_peeked()
        {
            return self.parse_async_fn_decl(decorators).map(Stmt::Decl);
        }

        // If the statement does not start with a statement keyword or a
        // brace, it's an ExpressionStatement or LabeledStatement. We
        // simply start parsing an expression, and afterwards, if the
        // next token is a colon and the expression was a simple
        // Identifier node, we switch to interpreting it as a label.
        let expr = self.allow_in_expr(|p| p.parse_expr())?;

        let expr = match expr {
            Expr::Ident(ident) => {
                if self.input_mut().eat(Token::Colon) {
                    return self.parse_labelled_stmt(ident);
                }
                Expr::Ident(ident)
            }
            _ => self.verify_expr(expr)?,
        };

        if let Expr::Ident(ref ident) = expr {
            let ident_sym = self.ast.get_atom(ident.sym(&self.ast));
            if ident_sym.as_str() == "interface" && self.input().had_line_break_before_cur() {
                self.emit_strict_mode_err(
                    ident.span(&self.ast),
                    SyntaxError::InvalidIdentInStrict(ident_sym.clone()),
                );

                self.eat_general_semi();

                return Ok(self.ast.stmt_expr_stmt(self.span(start), expr));
            }

            // if self.input().syntax().typescript() {
            //     if let Some(decl) = self.parse_ts_expr_stmt(decorators, ident.clone())? {
            //         return Ok(decl.into());
            //     }
            // }
        }

        // if self.syntax().typescript() {
        //     if let Expr::Ident(ref i) = expr {
        //         match &*i.sym {
        //             "public" | "static" | "abstract" => {
        //                 if self.input_mut().eat(Token::Interface) {
        //                     self.emit_err(i.span, SyntaxError::TS2427);
        //                     return self
        //                         .parse_ts_interface_decl(start)
        //                         .map(Decl::from)
        //                         .map(Stmt::from);
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
        // }

        if self.eat_general_semi() {
            Ok(self.ast.stmt_expr_stmt(self.span(start), expr))
        } else {
            let cur = self.input().cur();
            if cur.is_bin_op() {
                self.emit_err(self.input().cur_span(), SyntaxError::TS1005);
                let expr = self.parse_bin_op_recursively(expr, 0)?;
                return Ok(self.ast.stmt_expr_stmt(self.span(start), expr));
            }

            syntax_error!(
                self,
                SyntaxError::ExpectedSemiForExprStmt {
                    expr: expr.span(&self.ast)
                }
            );
        }
    }

    pub(crate) fn parse_stmt_block_body(
        &mut self,
        allow_directives: bool,
        end: Option<Token>,
    ) -> PResult<TypedSubRange<Stmt>> {
        self.parse_block_body(allow_directives, end, handle_import_export)
    }

    pub(crate) fn parse_block_body<Type: FromStmt>(
        &mut self,
        allow_directives: bool,
        end: Option<Token>,
        handle_import_export: impl Fn(&mut Self, Vec<Decorator>) -> PResult<Type>,
    ) -> PResult<TypedSubRange<Type>> {
        trace_cur!(self, parse_block_body);

        let mut stmts = Vec::with_capacity(8);

        let has_strict_directive = allow_directives
            && (self
                .input()
                .cur()
                .is_str_raw_content("\"use strict\"", self.input())
                || self
                    .input()
                    .cur()
                    .is_str_raw_content("'use strict'", self.input()));

        let parse_stmts = |p: &mut Self, stmts: &mut Vec<Type>| -> PResult<()> {
            let is_stmt_start = |p: &mut Self| {
                let cur = p.input().cur();
                match end {
                    Some(end) => {
                        if cur == Token::Eof {
                            let eof_text = p.input_mut().dump_cur();
                            p.emit_err(
                                p.input().cur_span(),
                                SyntaxError::Expected(format!("{end:?}"), eof_text),
                            );
                            false
                        } else {
                            cur != end
                        }
                    }
                    None => cur != Token::Eof,
                }
            };
            while is_stmt_start(p) {
                let stmt = p.parse_stmt_like(true, &handle_import_export)?;
                stmts.push(stmt);
            }
            Ok(())
        };

        if has_strict_directive {
            self.do_inside_of_context(Context::Strict, |p| parse_stmts(p, &mut stmts))?;
        } else {
            parse_stmts(self, &mut stmts)?;
        };

        if self.input().cur() != Token::Eof && end.is_some() {
            self.bump();
        }

        let stmts = self.ast.add_typed_sub_range(&stmts);
        Ok(stmts)
    }
}

fn handle_import_export<I: Tokens>(p: &mut Parser<I>, _: Vec<Decorator>) -> PResult<Stmt> {
    let start = p.cur_pos();
    if p.input().is(Token::Import) && peek!(p).is_some_and(|peek| peek == Token::LParen) {
        let expr = p.parse_expr()?;

        p.eat_general_semi();

        return Ok(p.ast.stmt_expr_stmt(p.span(start), expr));
    }

    if p.input().is(Token::Import) && peek!(p).is_some_and(|peek| peek == Token::Dot) {
        let expr = p.parse_expr()?;

        p.eat_general_semi();

        return Ok(p.ast.stmt_expr_stmt(p.span(start), expr));
    }

    syntax_error!(p, SyntaxError::ImportExportInScript);
}

// #[cfg(test)]
// mod tests {
//     use swc_atoms::atom;
//     use swc_common::{DUMMY_SP as span, comments::SingleThreadedComments};
//     use swc_ecma_visit::assert_eq_ignore_span;

//     use super::*;
//     use crate::{EsSyntax, TsSyntax};

//     fn stmt(s: &'static str) -> Stmt {
//         test_parser(s, Syntax::default(), |p| p.parse_stmt())
//     }

//     fn module_item(s: &'static str) -> ModuleItem {
//         test_parser(s, Syntax::default(), |p| p.parse_module_item())
//     }
//     fn expr(s: &'static str) -> Box<Expr> {
//         test_parser(s, Syntax::default(), |p| p.parse_expr())
//     }

//     #[test]
//     fn expr_stmt() {
//         assert_eq_ignore_span!(
//             stmt("a + b + c"),
//             Stmt::Expr(ExprStmt {
//                 span,
//                 expr: expr("a + b + c")
//             })
//         )
//     }

//     #[test]
//     fn catch_rest_pat() {
//         assert_eq_ignore_span!(
//             stmt("try {} catch({ ...a34 }) {}"),
//             Stmt::Try(Box::new(TryStmt {
//                 span,
//                 block: BlockStmt {
//                     span,
//                     ..Default::default()
//                 },
//                 handler: Some(CatchClause {
//                     span,
//                     param: Pat::Object(ObjectPat {
//                         span,
//                         optional: false,
//                         props: vec![ObjectPatProp::Rest(RestPat {
//                             span,
//                             dot3_token: span,
//                             arg: Box::new(Pat::Ident(
//                                 Ident::new_no_ctxt(atom!("a34"), span).into()
//                             )),
//                             type_ann: None
//                         })],
//                         type_ann: None,
//                     })
//                     .into(),
//                     body: BlockStmt {
//                         span,
//                         ..Default::default()
//                     }
//                 }),
//                 finalizer: None
//             }))
//         );
//     }

//     #[test]
//     fn throw_this() {
//         assert_eq_ignore_span!(
//             stmt("throw this"),
//             Stmt::Throw(ThrowStmt {
//                 span,
//                 arg: expr("this"),
//             })
//         )
//     }

//     #[test]
//     fn await_for_of() {
//         assert_eq_ignore_span!(
//             stmt("for await (const a of b) ;"),
//             Stmt::ForOf(ForOfStmt {
//                 span,
//                 is_await: true,
//                 left: ForHead::VarDecl(Box::new(VarDecl {
//                     span,
//                     kind: VarDeclKind::Const,
//                     decls: vec![VarDeclarator {
//                         span,
//                         init: None,
//                         name: Pat::Ident(Ident::new_no_ctxt(atom!("a"), span).into()),
//                         definite: false,
//                     }],
//                     ..Default::default()
//                 })),
//                 right: Box::new(Expr::Ident(Ident::new_no_ctxt(atom!("b"), span))),

//                 body: Box::new(Stmt::Empty(EmptyStmt { span })),
//             })
//         )
//     }

//     #[test]
//     fn no_empty_without_semi() {
//         assert_eq_ignore_span!(
//             stmt("(function foo() { return 1 })"),
//             stmt(
//                 "(function foo () {
//                 return 1
//             })"
//             )
//         );

//         assert_eq_ignore_span!(
//             stmt("{ 1; }"),
//             Stmt::Block(BlockStmt {
//                 span,
//                 stmts: vec![stmt("1")],
//                 ..Default::default()
//             })
//         );
//     }

//     #[test]
//     fn if_else() {
//         assert_eq_ignore_span!(
//             stmt("if (a) b; else c"),
//             Stmt::If(IfStmt {
//                 span,
//                 test: expr("a"),
//                 cons: Box::new(stmt("b;")),
//                 alt: Some(Box::new(stmt("c"))),
//             })
//         );
//     }

//     #[test]
//     fn class_decorator() {
//         assert_eq_ignore_span!(
//             test_parser(
//                 "
//             @decorator
//             @dec2
//             class Foo {}
//             ",
//                 Syntax::Es(EsSyntax {
//                     decorators: true,
//                     ..Default::default()
//                 }),
//                 |p| p.parse_stmt_list_item(),
//             ),
//             Stmt::Decl(Decl::Class(ClassDecl {
//                 ident: Ident::new_no_ctxt(atom!("Foo"), span),
//                 class: Box::new(Class {
//                     span,
//                     decorators: vec![
//                         Decorator {
//                             span,
//                             expr: expr("decorator")
//                         },
//                         Decorator {
//                             span,
//                             expr: expr("dec2")
//                         }
//                     ],
//                     super_class: None,
//                     body: Vec::new(),
//                     is_abstract: false,
//                     ..Default::default()
//                 }),
//                 declare: false,
//             }))
//         );
//     }

//     #[test]
//     fn example() {
//         let src = r#"
// import React from 'react'
// import ReactDOM from 'react-dom'

// function App() {
//   return <h1>JSX is working!</h1>
// }

// ReactDOM.render(<App />, document.getElementById('root'))

// "#;
//         test_parser(
//             src,
//             Syntax::Es(EsSyntax {
//                 jsx: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn ice() {
//         let src = r#"import React from "react"

// function App() {
//   return <h1>works</h1>
// }

// export default App"#;
//         test_parser(
//             src,
//             Syntax::Es(EsSyntax {
//                 jsx: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn export_default() {
//         let src = "export v, { x, y as w } from 'mod';";
//         test_parser(
//             src,
//             Syntax::Es(EsSyntax {
//                 export_default_from: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn export_default_2() {
//         let src = "export foo from 'bar';";
//         test_parser(
//             src,
//             Syntax::Es(EsSyntax {
//                 export_default_from: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn export_default_3() {
//         let src = "export default from 'bar';";
//         test_parser(
//             src,
//             Syntax::Es(EsSyntax {
//                 export_default_from: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn export_default_4() {
//         let src = "export default, {foo} from 'bar';";
//         test_parser(
//             src,
//             Syntax::Es(EsSyntax {
//                 export_default_from: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn shebang_01() {
//         let src = "#!/usr/bin/env node";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     fn shebang_02() {
//         let src = "#!/usr/bin/env node
// let x = 4";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     fn empty() {
//         test_parser("", Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     fn issue_226() {
//         test_parser(
//             "export * as Foo from 'bar';",
//             Syntax::Es(EsSyntax {
//                 export_default_from: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     #[should_panic(expected = "Expected 'from', got ','")]
//     fn issue_4369_1() {
//         test_parser(
//             r#"export * as foo, { bar } from "mod""#,
//             Syntax::Es(EsSyntax {
//                 export_default_from: false,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_4369_2() {
//         test_parser(
//             r#"export foo, * as bar, { baz } from "mod""#,
//             Syntax::Es(EsSyntax {
//                 export_default_from: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_4369_3() {
//         test_parser(
//             r#"export foo, * as bar from "mod""#,
//             Syntax::Es(EsSyntax {
//                 export_default_from: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_4369_4() {
//         test_parser(
//             r#"export * as bar, { baz } from "mod""#,
//             Syntax::Es(EsSyntax {
//                 export_default_from: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_4369_5() {
//         test_parser(
//             r#"export foo, { baz } from "mod""#,
//             Syntax::Es(EsSyntax {
//                 export_default_from: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_257_var() {
//         test_parser(
//             "
// export default function waitUntil(callback, options = {}) {
//   var timeout = 'timeout' in options ? options.timeout : 1000;
// }",
//             Default::default(),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_257_let() {
//         test_parser(
//             "
// export default function waitUntil(callback, options = {}) {
//   let timeout = 'timeout' in options ? options.timeout : 1000;
// }",
//             Default::default(),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_269() {
//         test_parser(
//             ";(function() {})(window, window.lib || (window.lib = {}))",
//             Default::default(),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_319_2() {
//         module_item(
//             "export default obj({
//     async f() {
//         await g();
//     }
// });",
//         );
//     }

//     #[test]
//     fn issue_340_fn() {
//         test_parser("export default function(){};", Default::default(), |p| {
//             p.parse_module()
//         });
//     }

//     #[test]
//     fn issue_340_async_fn() {
//         test_parser(
//             "export default async function(){};",
//             Default::default(),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_340_generator_fn() {
//         test_parser("export default function*(){};", Default::default(), |p| {
//             p.parse_module()
//         });
//     }

//     #[test]
//     fn issue_340_class() {
//         test_parser("export default class {};", Default::default(), |p| {
//             p.parse_module()
//         });
//     }

//     #[test]
//     fn issue_360() {
//         test_parser(
//             "var IS_IE11 = !global.ActiveXObject && 'ActiveXObject' in global;",
//             Default::default(),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_380_1() {
//         test_parser(
//             "import(filePath).then(bar => {})",
//             Syntax::Es(Default::default()),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_380_2() {
//         test_parser(
//             "class Foo {
//                 componentDidMount() {
//                     const filePath = '../foo/bar'
//                     import(filePath).then(bar => {})
//                 }
//             }",
//             Syntax::Es(Default::default()),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn issue_411() {
//         test_parser(
//             "try {
// } catch {}",
//             Syntax::Es(Default::default()),
//             |p| p.parse_module(),
//         );
//     }

//     #[test]
//     fn top_level_await() {
//         test_parser("await foo", Syntax::Es(Default::default()), |p| {
//             p.parse_module()
//         });
//     }

//     #[test]
//     fn issue_856() {
//         let c = SingleThreadedComments::default();
//         let s = "class Foo {
//     static _extensions: {
//         // eslint-disable-next-line @typescript-eslint/no-explicit-any
//         [key: string]: (module: Module, filename: string) => any;
//     } = Object.create(null);
// }
// ";
//         let _ = test_parser_comment(&c, s, Syntax::Typescript(Default::default()), |p| {
//             p.parse_typescript_module()
//         });

//         let (leading, trailing) = c.take_all();
//         assert!(trailing.borrow().is_empty());
//         assert_eq!(leading.borrow().len(), 1);
//     }

//     #[test]
//     fn issue_856_2() {
//         let c = SingleThreadedComments::default();
//         let s = "type ConsoleExamineFunc = (
//   // eslint-disable-next-line @typescript-eslint/no-explicit-any
//   csl: any,
//   out: StringBuffer,
//   err?: StringBuffer,
//   both?: StringBuffer
// ) => void;";

//         let _ = test_parser_comment(&c, s, Syntax::Typescript(Default::default()), |p| {
//             p.parse_typescript_module()
//         });

//         let (leading, trailing) = c.take_all();
//         assert!(trailing.borrow().is_empty());
//         assert_eq!(leading.borrow().len(), 1);
//     }

//     #[test]
//     fn issue_856_3() {
//         let c = SingleThreadedComments::default();
//         let s = "type RequireWrapper = (
//   // eslint-disable-next-line @typescript-eslint/no-explicit-any
//   exports: any,
//   // eslint-disable-next-line @typescript-eslint/no-explicit-any
//   require: any,
//   module: Module,
//   __filename: string,
//   __dirname: string
// ) => void;";

//         let _ = test_parser_comment(&c, s, Syntax::Typescript(Default::default()), |p| {
//             p.parse_typescript_module()
//         });

//         let (leading, trailing) = c.take_all();
//         assert!(trailing.borrow().is_empty());
//         assert_eq!(leading.borrow().len(), 2);
//     }

//     #[test]
//     fn issue_856_4() {
//         let c = SingleThreadedComments::default();
//         let s = "const _extensions: {
//     // eslint-disable-next-line @typescript-eslint/no-explicit-any
//     [key: string]: (module: Module, filename: string) => any;
//   } = Object.create(null);";

//         let _ = test_parser_comment(&c, s, Syntax::Typescript(Default::default()), |p| {
//             p.parse_typescript_module()
//         });

//         let (leading, trailing) = c.take_all();
//         assert!(trailing.borrow().is_empty());
//         assert_eq!(leading.borrow().len(), 1);
//     }

//     fn parse_for_head(str: &'static str) -> TempForHead {
//         test_parser(str, Syntax::default(), |p| p.parse_for_head())
//     }

//     #[test]
//     fn for_array_binding_pattern() {
//         match parse_for_head("let [, , t] = simple_array; t < 10; t++") {
//             TempForHead::For { init: Some(v), .. } => assert_eq_ignore_span!(
//                 v,
//                 VarDeclOrExpr::VarDecl(Box::new(VarDecl {
//                     span,
//                     declare: false,
//                     kind: VarDeclKind::Let,
//                     decls: vec![VarDeclarator {
//                         span,
//                         name: Pat::Array(ArrayPat {
//                             span,
//                             type_ann: None,
//                             optional: false,
//                             elems: vec![
//                                 None,
//                                 None,
//                                 Some(Pat::Ident(Ident::new_no_ctxt(atom!("t"), span).into()))
//                             ]
//                         }),
//                         init: Some(Box::new(Expr::Ident(Ident::new_no_ctxt(
//                             atom!("simple_array"),
//                             span
//                         )))),
//                         definite: false
//                     }],
//                     ..Default::default()
//                 }))
//             ),
//             _ => unreachable!(),
//         }
//     }
//     #[test]
//     fn for_object_binding_pattern() {
//         match parse_for_head("let {num} = obj; num < 11; num++") {
//             TempForHead::For { init: Some(v), .. } => assert_eq_ignore_span!(
//                 v,
//                 VarDeclOrExpr::VarDecl(Box::new(VarDecl {
//                     span,
//                     kind: VarDeclKind::Let,
//                     decls: vec![VarDeclarator {
//                         span,
//                         name: Pat::Object(ObjectPat {
//                             optional: false,
//                             type_ann: None,
//                             span,
//                             props: vec![ObjectPatProp::Assign(AssignPatProp {
//                                 span,
//                                 key: Ident::new_no_ctxt(atom!("num"), span).into(),
//                                 value: None
//                             })]
//                         }),
//                         init: Some(Box::new(Expr::Ident(Ident::new_no_ctxt(
//                             atom!("obj"),
//                             span
//                         )))),
//                         definite: false
//                     }],
//                     ..Default::default()
//                 }))
//             ),
//             _ => unreachable!(),
//         }
//     }

//     #[test]
//     #[should_panic(expected = "'import.meta' cannot be used outside of module code.")]
//     fn import_meta_in_script() {
//         let src = "const foo = import.meta.url;";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_script());
//     }

//     #[test]
//     fn import_meta_in_program() {
//         let src = "const foo = import.meta.url;";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_program());
//     }

//     #[test]
//     #[should_panic(expected = "'import', and 'export' cannot be used outside of module code")]
//     fn import_statement_in_script() {
//         let src = "import 'foo';";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_script());
//     }

//     #[test]
//     #[should_panic(expected = "top level await is only allowed in module")]
//     fn top_level_await_in_script() {
//         let src = "await promise";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_script());
//     }

//     #[test]
//     fn top_level_await_in_program() {
//         let src = "await promise";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_program());
//     }

//     #[test]
//     fn for_of_head_lhs_async_dot() {
//         let src = "for (async.x of [1]) ;";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     fn for_head_init_async_of() {
//         let src = "for (async of => {}; i < 10; ++i) { ++counter; }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     #[should_panic(expected = "await isn't allowed in non-async function")]
//     fn await_in_function_in_module() {
//         let src = "function foo (p) { await p; }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     #[should_panic(expected = "await isn't allowed in non-async function")]
//     fn await_in_function_in_script() {
//         let src = "function foo (p) { await p; }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_script());
//     }

//     #[test]
//     #[should_panic(expected = "await isn't allowed in non-async function")]
//     fn await_in_function_in_program() {
//         let src = "function foo (p) { await p; }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_program());
//     }

//     #[test]
//     #[should_panic(expected = "`await` cannot be used as an identifier in an async context")]
//     fn await_in_nested_async_function_in_module() {
//         let src = "async function foo () { function bar(x = await) {} }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     fn await_in_nested_async_function_in_script() {
//         let src = "async function foo () { function bar(x = await) {} }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_script());
//     }

//     #[test]
//     fn await_in_nested_async_function_in_program() {
//         let src = "async function foo () { function bar(x = await) {} }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_program());
//     }

//     #[test]
//     #[should_panic(expected = "`await` cannot be used as an identifier in an async context")]
//     fn await_as_param_ident_in_module() {
//         let src = "function foo (x = await) { }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     fn await_as_param_ident_in_script() {
//         let src = "function foo (x = await) { }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_script());
//     }

//     #[test]
//     #[should_panic(expected = "`await` cannot be used as an identifier in an async context")]
//     fn await_as_ident_in_module() {
//         let src = "let await = 1";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     fn await_as_ident_in_script() {
//         let src = "let await = 1";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_script());
//     }

//     #[test]
//     #[should_panic(expected = "`await` cannot be used as an identifier in an async context")]
//     fn await_as_ident_in_async() {
//         let src = "async function foo() { let await = 1; }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_script());
//     }

//     #[test]
//     fn top_level_await_in_block() {
//         let src = "if (true) { await promise; }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     fn top_level_await_in_decl() {
//         test_parser(r#"const t = await test();"#, Default::default(), |p| {
//             let program = p.parse_program()?;
//             assert!(program.is_module());
//             Ok(program)
//         });
//     }

//     #[test]
//     fn class_static_blocks() {
//         let src = "class Foo { static { 1 + 1; } }";
//         assert_eq_ignore_span!(
//             test_parser(src, Syntax::Es(Default::default()), |p| p.parse_expr()),
//             Box::new(Expr::Class(ClassExpr {
//                 ident: Some(Ident {
//                     span,
//                     sym: atom!("Foo"),
//                     ..Default::default()
//                 }),
//                 class: Box::new(Class {
//                     span,
//                     decorators: Vec::new(),
//                     super_class: None,
//                     type_params: None,
//                     super_type_params: None,
//                     is_abstract: false,
//                     implements: Vec::new(),
//                     body: vec!(ClassMember::StaticBlock(StaticBlock {
//                         span,
//                         body: BlockStmt {
//                             span,
//                             stmts: vec!(stmt("1 + 1;")),
//                             ..Default::default()
//                         }
//                     })),
//                     ..Default::default()
//                 })
//             }))
//         );
//     }

//     #[test]
//     fn multiple_class_static_blocks() {
//         let src = "class Foo { static { 1 + 1; } static { 1 + 1; } }";
//         assert_eq_ignore_span!(
//             test_parser(src, Syntax::Es(Default::default()), |p| p.parse_expr()),
//             Box::new(Expr::Class(ClassExpr {
//                 ident: Some(Ident {
//                     span,
//                     sym: atom!("Foo"),
//                     ..Default::default()
//                 }),
//                 class: Box::new(Class {
//                     span,
//                     decorators: Vec::new(),
//                     super_class: None,
//                     is_abstract: false,
//                     body: vec!(
//                         ClassMember::StaticBlock(StaticBlock {
//                             span,
//                             body: BlockStmt {
//                                 span,
//                                 stmts: vec!(stmt("1 + 1;")),
//                                 ..Default::default()
//                             },
//                         }),
//                         ClassMember::StaticBlock(StaticBlock {
//                             span,
//                             body: BlockStmt {
//                                 span,
//                                 stmts: vec!(stmt("1 + 1;")),
//                                 ..Default::default()
//                             },
//                         })
//                     ),
//                     ..Default::default()
//                 })
//             }))
//         );
//     }

//     #[test]
//     fn class_static_blocks_with_line_breaks_01() {
//         let src = "class Foo {
//             static
//             {
//                 1 + 1;
//             }
//         }";
//         assert_eq_ignore_span!(
//             test_parser(src, Syntax::Es(Default::default()), |p| p.parse_expr()),
//             Box::new(Expr::Class(ClassExpr {
//                 ident: Some(Ident {
//                     span,
//                     sym: atom!("Foo"),
//                     ..Default::default()
//                 }),
//                 class: Box::new(Class {
//                     span,
//                     is_abstract: false,
//                     body: vec!(ClassMember::StaticBlock(StaticBlock {
//                         span,
//                         body: BlockStmt {
//                             span,
//                             stmts: vec!(stmt("1 + 1;")),
//                             ..Default::default()
//                         }
//                     })),
//                     ..Default::default()
//                 })
//             }))
//         );
//     }

//     #[test]
//     fn class_static_blocks_with_line_breaks_02() {
//         let src = "class Foo {
//             static
//             {}
//         }";
//         assert_eq_ignore_span!(
//             test_parser(src, Syntax::Es(Default::default()), |p| p.parse_expr()),
//             Box::new(Expr::Class(ClassExpr {
//                 ident: Some(Ident {
//                     span,
//                     sym: atom!("Foo"),
//                     ..Default::default()
//                 }),
//                 class: Box::new(Class {
//                     span,
//                     is_abstract: false,
//                     body: vec!(ClassMember::StaticBlock(StaticBlock {
//                         span,
//                         body: BlockStmt {
//                             span,
//                             stmts: Vec::new(),
//                             ..Default::default()
//                         }
//                     })),
//                     ..Default::default()
//                 })
//             }))
//         );
//     }

//     #[test]
//     fn class_static_blocks_in_ts() {
//         let src = "class Foo { static { 1 + 1 }; }";
//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_expr()
//         });
//     }

//     #[test]
//     fn class_static_blocks_with_line_breaks_in_ts_01() {
//         let src = "class Foo {
//             static
//             {
//                 1 + 1;
//             }
//         }";
//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_expr()
//         });
//     }

//     #[test]
//     fn class_static_blocks_with_line_breaks_in_ts_02() {
//         let src = "class Foo {
//             static
//             {}
//         }";
//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_expr()
//         });
//     }

//     #[test]
//     #[should_panic(expected = "Expected ident")]
//     fn class_static_blocks_with_await() {
//         let src = "class Foo{
//             static {
//                 var await = 'bar';
//             }
//         }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_expr());
//     }

//     #[test]
//     #[should_panic(expected = "Expected ident")]
//     fn class_static_blocks_with_await_in_nested_class() {
//         let src = "class Foo{
//             static {
//                 function foo() {
//                     class Foo {
//                         static {
//                             var await = 'bar';
//                         }
//                     }
//                 }
//             }
//         }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_expr());
//     }

//     #[test]
//     fn class_static_blocks_with_await_in_fn() {
//         let src = "class Foo{
//             static {
//                 function foo() {
//                     var await = 'bar';
//                 }
//             }
//         }";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_expr());
//     }

//     #[test]
//     #[should_panic(expected = "Modifiers cannot appear here")]
//     fn class_static_blocks_in_ts_with_invalid_modifier_01() {
//         let src = "class Foo { abstract static { 1 + 1 }; }";
//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_expr()
//         });
//     }

//     #[test]
//     #[should_panic(expected = "Modifiers cannot appear here")]
//     fn class_static_blocks_in_ts_with_invalid_modifier_02() {
//         let src = "class Foo { static static { 1 + 1 }; }";
//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_expr()
//         });
//     }

//     #[test]
//     #[should_panic(expected = "Modifiers cannot appear here")]
//     fn class_static_blocks_in_ts_with_invalid_modifier_03() {
//         let src = "class Foo { declare static { 1 + 1 }; }";
//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_expr()
//         });
//     }

//     #[test]
//     #[should_panic(expected = "Modifiers cannot appear here")]
//     fn class_static_blocks_in_ts_with_invalid_modifier_04() {
//         let src = "class Foo { private static { 1 + 1 }; }";
//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_expr()
//         });
//     }

//     #[ignore = "see https://github.com/swc-project/swc/pull/3018#issuecomment-991947907"]
//     #[test]
//     #[should_panic(expected = "Trailing comma is disallowed inside import(...) arguments")]
//     fn error_for_trailing_comma_inside_dynamic_import() {
//         let src = "import('foo',)";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_expr());
//     }

//     #[test]
//     fn no_error_for_trailing_comma_inside_dynamic_import_with_import_assertions() {
//         let src = "import('foo',)";
//         test_parser(
//             src,
//             Syntax::Es(EsSyntax {
//                 import_attributes: true,
//                 ..Default::default()
//             }),
//             |p| p.parse_expr(),
//         );
//     }

//     #[test]
//     fn type_only_star_exports_with_name() {
//         let src = "export type * as bar from 'mod'";
//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_module()
//         });
//     }

//     #[test]
//     fn type_only_star_exports_without_name() {
//         let src = "export type * from 'mod'";
//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_module()
//         });
//     }

//     #[test]
//     #[should_panic(expected = "A string literal cannot be used as an imported binding.")]
//     fn error_for_string_literal_is_import_binding() {
//         let src = "import { \"str\" } from \"mod\"";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     #[should_panic(
//         expected = "A string literal cannot be used as an exported binding without `from`."
//     )]
//     fn error_for_string_literal_is_export_binding() {
//         let src = "export { 'foo' };";
//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_module());
//     }

//     #[test]
//     #[should_panic(expected = "'const' declarations must be initialized")]
//     fn ts_error_for_const_declaration_not_initialized() {
//         let src = r#"
// "use strict";
// const foo;"#;

//         test_parser(src, Syntax::Typescript(Default::default()), |p| {
//             p.parse_script()
//         });
//     }

//     #[test]
//     #[should_panic(expected = "'const' declarations must be initialized")]
//     fn es_error_for_const_declaration_not_initialized() {
//         let src = r#"
// "use strict";
// const foo;"#;

//         test_parser(src, Syntax::Es(Default::default()), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_5557_expr_follow_class() {
//         let src = "foo * class {} / bar;";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_5722_class_keyword_in_tpl() {
//         let src = "console.log(`${toStr({class: fn})}`)";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_6301_await_expr_stmt() {
//         let src = "try { await; } catch { console.log('caught'); }";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_6301_await_expr_stmt_1() {
//         let src = "try { await, await; } catch { console.log('caught'); }";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_6301_await_expr_stmt_2() {
//         let src = "function test() { await; }";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_6301_await_expr_stmt_3() {
//         let src = "function test() { await, await; }";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_6301_await_expr_stmt_4() {
//         let src = "function test() { [await]; }";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_6301_await_expr_stmt_5() {
//         let src = "function test() { (await); }";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_6322() {
//         let src = "for ( ; { } / 1 ; ) ;";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_6323() {
//         let src = "let x = 0 < { } / 0 ;";

//         test_parser(src, Default::default(), |p| p.parse_script());
//     }

//     #[test]
//     fn issue_10797_0() {
//         let src = "
// // var b = delete ANY1;

// module A {
//     export const a = 1
// }";
//         test_parser(src, Syntax::Typescript(TsSyntax::default()), |p| {
//             p.parse_script()
//         });
//     }

//     #[test]
//     fn issue_10797_1() {
//         let src = "
// module A {
//     export const a = 1
// }

// var b = delete ANY1;";
//         test_parser(src, Syntax::Typescript(TsSyntax::default()), |p| {
//             p.parse_script()
//         });
//     }
// }
