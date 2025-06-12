fn parse_counted_repetition(
        &self,
        mut concat: ast::Concat,
    ) -> Result<ast::Concat> {
        assert!(self.char() == '{');
        let start = self.pos();
        let ast = match concat.asts.pop() {
            Some(ast) => ast,
            None => return Err(self.error(
                self.span(),
                ast::ErrorKind::RepetitionMissing,
            )),
        };
        if !self.bump_and_bump_space() {
            return Err(self.error(
                Span::new(start, self.pos()),
                ast::ErrorKind::RepetitionCountUnclosed,
            ));
        }
        let count_start = self.parse_decimal()?;
        let mut range = ast::RepetitionRange::Exactly(count_start);
        if self.is_eof() {
            return Err(self.error(
                Span::new(start, self.pos()),
                ast::ErrorKind::RepetitionCountUnclosed,
            ));
        }
        if self.char() == ',' {
            if !self.bump_and_bump_space() {
                return Err(self.error(
                    Span::new(start, self.pos()),
                    ast::ErrorKind::RepetitionCountUnclosed,
                ));
            }
            if self.char() != '}' {
                let count_end = self.parse_decimal()?;
                range = ast::RepetitionRange::Bounded(count_start, count_end);
            } else {
                range = ast::RepetitionRange::AtLeast(count_start);
            }
        }
        if self.is_eof() || self.char() != '}' {
            return Err(self.error(
                Span::new(start, self.pos()),
                ast::ErrorKind::RepetitionCountUnclosed,
            ));
        }

        let mut greedy = true;
        if self.bump_and_bump_space() && self.char() == '?' {
            greedy = false;
            self.bump();
        }

        let op_span = Span::new(start, self.pos());
        if !range.is_valid() {
            return Err(self.error(
                op_span,
                ast::ErrorKind::RepetitionCountInvalid,
            ));
        }
        concat.asts.push(Ast::Repetition(ast::Repetition {
            span: ast.span().with_end(self.pos()),
            op: ast::RepetitionOp {
                span: op_span,
                kind: ast::RepetitionKind::Range(range),
            },
            greedy: greedy,
            ast: Box::new(ast),
        }));
        Ok(concat)
    }