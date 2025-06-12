fn push_alternate(&self, mut concat: ast::Concat) -> Result<ast::Concat> {
        assert_eq!(self.char(), '|');
        concat.span.end = self.pos();
        self.push_or_add_alternation(concat);
        self.bump();
        Ok(ast::Concat {
            span: self.span(),
            asts: vec![],
        })
    }