fn push_group(&self, mut concat: ast::Concat) -> Result<ast::Concat> {
        assert_eq!(self.char(), '(');
        match self.parse_group()? {
            Either::Left(set) => {
                let ignore = set.flags.flag_state(ast::Flag::IgnoreWhitespace);
                if let Some(v) = ignore {
                    self.parser().ignore_whitespace.set(v);
                }

                concat.asts.push(Ast::Flags(set));
                Ok(concat)
            }
            Either::Right(group) => {
                let old_ignore_whitespace = self.ignore_whitespace();
                let new_ignore_whitespace = group
                    .flags()
                    .and_then(|f| f.flag_state(ast::Flag::IgnoreWhitespace))
                    .unwrap_or(old_ignore_whitespace);
                self.parser().stack_group.borrow_mut().push(GroupState::Group {
                    concat: concat,
                    group: group,
                    ignore_whitespace: old_ignore_whitespace,
                });
                self.parser().ignore_whitespace.set(new_ignore_whitespace);
                Ok(ast::Concat {
                    span: self.span(),
                    asts: vec![],
                })
            }
        }
    }