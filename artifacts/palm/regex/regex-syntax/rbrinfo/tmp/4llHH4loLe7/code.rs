fn visit_post(&mut self, ast: &Ast) -> Result<()> {
        match *ast {
            Ast::Empty(_) => {
                self.push(HirFrame::Expr(Hir::empty()));
            }
            Ast::Flags(ref x) => {
                self.set_flags(&x.flags);
            }
            Ast::Literal(ref x) => {
                self.push(HirFrame::Expr(self.hir_literal(x)?));
            }
            Ast::Dot(span) => {
                self.push(HirFrame::Expr(self.hir_dot(span)?));
            }
            Ast::Assertion(ref x) => {
                self.push(HirFrame::Expr(self.hir_assertion(x)?));
            }
            Ast::Class(ast::Class::Perl(ref x)) => {
                if self.flags().unicode() {
                    let cls = self.hir_perl_unicode_class(x);
                    let hcls = hir::Class::Unicode(cls);
                    self.push(HirFrame::Expr(Hir::class(hcls)));
                } else {
                    let cls = self.hir_perl_byte_class(x);
                    let hcls = hir::Class::Bytes(cls);
                    self.push(HirFrame::Expr(Hir::class(hcls)));
                }
            }
            Ast::Class(ast::Class::Unicode(ref x)) => {
                let cls = hir::Class::Unicode(self.hir_unicode_class(x)?);
                self.push(HirFrame::Expr(Hir::class(cls)));
            }
            Ast::Class(ast::Class::Bracketed(ref ast)) => {
                if self.flags().unicode() {
                    let mut cls = self.pop().unwrap().unwrap_class_unicode();
                    self.unicode_fold_and_negate(ast.negated, &mut cls);
                    if cls.iter().next().is_none() {
                        return Err(self.error(
                            ast.span, ErrorKind::EmptyClassNotAllowed));
                    }
                    let expr = Hir::class(hir::Class::Unicode(cls));
                    self.push(HirFrame::Expr(expr));
                } else {
                    let mut cls = self.pop().unwrap().unwrap_class_bytes();
                    self.bytes_fold_and_negate(
                        &ast.span, ast.negated, &mut cls)?;
                    if cls.iter().next().is_none() {
                        return Err(self.error(
                            ast.span, ErrorKind::EmptyClassNotAllowed));
                    }

                    let expr = Hir::class(hir::Class::Bytes(cls));
                    self.push(HirFrame::Expr(expr));
                }
            }
            Ast::Repetition(ref x) => {
                let expr = self.pop().unwrap().unwrap_expr();
                self.push(HirFrame::Expr(self.hir_repetition(x, expr)));
            }
            Ast::Group(ref x) => {
                let expr = self.pop().unwrap().unwrap_expr();
                if let Some(flags) = self.pop().unwrap().unwrap_group() {
                    self.trans().flags.set(flags);
                }
                self.push(HirFrame::Expr(self.hir_group(x, expr)));
            }
            Ast::Concat(_) => {
                let mut exprs = vec![];
                while let Some(HirFrame::Expr(expr)) = self.pop() {
                    if !expr.kind().is_empty() {
                        exprs.push(expr);
                    }
                }
                exprs.reverse();
                self.push(HirFrame::Expr(Hir::concat(exprs)));
            }
            Ast::Alternation(_) => {
                let mut exprs = vec![];
                while let Some(HirFrame::Expr(expr)) = self.pop() {
                    exprs.push(expr);
                }
                exprs.reverse();
                self.push(HirFrame::Expr(Hir::alternation(exprs)));
            }
        }
        Ok(())
    }