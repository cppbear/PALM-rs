fn parse_uncounted_repetition(
        &self,
        mut concat: ast::Concat,
        kind: ast::RepetitionKind,
    ) -> Result<ast::Concat> {
        assert!(
            self.char() == '?' || self.char() == '*' || self.char() == '+');
        let op_start = self.pos();
        let ast = match concat.asts.pop() {
            Some(ast) => ast,
            None => return Err(self.error(
                self.span(),
                ast::ErrorKind::RepetitionMissing,
            )),
        };
        match ast {
            Ast::Empty(_) | Ast::Flags(_) => return Err(self.error(
                self.span(),
                ast::ErrorKind::RepetitionMissing,
            )),
            _ => {}
        }
        let mut greedy = true;
        if self.bump() && self.char() == '?' {
            greedy = false;
            self.bump();
        }
        concat.asts.push(Ast::Repetition(ast::Repetition {
            span: ast.span().with_end(self.pos()),
            op: ast::RepetitionOp {
                span: Span::new(op_start, self.pos()),
                kind: kind,
            },
            greedy: greedy,
            ast: Box::new(ast),
        }));
        Ok(concat)
    }