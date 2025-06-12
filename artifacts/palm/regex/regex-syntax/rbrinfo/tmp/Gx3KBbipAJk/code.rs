fn pop_group(&self, mut group_concat: ast::Concat) -> Result<ast::Concat> {
        use self::GroupState::*;

        assert_eq!(self.char(), ')');
        let mut stack = self.parser().stack_group.borrow_mut();
        let (mut prior_concat, mut group, ignore_whitespace, alt) =
            match stack.pop() {
                Some(Group { concat, group, ignore_whitespace }) => {
                    (concat, group, ignore_whitespace, None)
                }
                Some(Alternation(alt)) => {
                    match stack.pop() {
                        Some(Group { concat, group, ignore_whitespace }) => {
                            (concat, group, ignore_whitespace, Some(alt))
                        }
                        None | Some(Alternation(_)) => {
                            return Err(self.error(
                                self.span_char(),
                                ast::ErrorKind::GroupUnopened,
                            ));
                        }
                    }
                }
                None => {
                    return Err(self.error(
                        self.span_char(),
                        ast::ErrorKind::GroupUnopened,
                    ));
                }
            };
        self.parser().ignore_whitespace.set(ignore_whitespace);
        group_concat.span.end = self.pos();
        self.bump();
        group.span.end = self.pos();
        match alt {
            Some(mut alt) => {
                alt.span.end = group_concat.span.end;
                alt.asts.push(group_concat.into_ast());
                group.ast = Box::new(alt.into_ast());
            }
            None => {
                group.ast = Box::new(group_concat.into_ast());
            }
        }
        prior_concat.asts.push(Ast::Group(group));
        Ok(prior_concat)
    }