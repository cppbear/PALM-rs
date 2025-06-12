fn parse_with_comments(&self) -> Result<ast::WithComments> {
        assert_eq!(self.offset(), 0, "parser can only be used once");
        self.parser().reset();
        let mut concat = ast::Concat {
            span: self.span(),
            asts: vec![],
        };
        loop {
            self.bump_space();
            if self.is_eof() {
                break;
            }
            match self.char() {
                '(' => concat = self.push_group(concat)?,
                ')' => concat = self.pop_group(concat)?,
                '|' => concat = self.push_alternate(concat)?,
                '[' => {
                    let class = self.parse_set_class()?;
                    concat.asts.push(Ast::Class(class));
                }
                '?' => {
                    concat = self.parse_uncounted_repetition(
                        concat, ast::RepetitionKind::ZeroOrOne)?;
                }
                '*' => {
                    concat = self.parse_uncounted_repetition(
                        concat, ast::RepetitionKind::ZeroOrMore)?;
                }
                '+' => {
                    concat = self.parse_uncounted_repetition(
                        concat, ast::RepetitionKind::OneOrMore)?;
                }
                '{' => {
                    concat = self.parse_counted_repetition(concat)?;
                }
                _ => concat.asts.push(self.parse_primitive()?.into_ast()),
            }
        }
        let ast = self.pop_group_end(concat)?;
        NestLimiter::new(self).check(&ast)?;
        Ok(ast::WithComments {
            ast: ast,
            comments: mem::replace(
                &mut *self.parser().comments.borrow_mut(),
                vec![],
            ),
        })
    }