fn visit_pre(&mut self, ast: &Ast) -> Result<()> {
        let span = match *ast {
            Ast::Empty(_)
            | Ast::Flags(_)
            | Ast::Literal(_)
            | Ast::Dot(_)
            | Ast::Assertion(_)
            | Ast::Class(ast::Class::Unicode(_))
            | Ast::Class(ast::Class::Perl(_)) => {
                // These are all base cases, so we don't increment depth.
                return Ok(());
            }
            Ast::Class(ast::Class::Bracketed(ref x)) => &x.span,
            Ast::Repetition(ref x) => &x.span,
            Ast::Group(ref x) => &x.span,
            Ast::Alternation(ref x) => &x.span,
            Ast::Concat(ref x) => &x.span,
        };
        self.increment_depth(span)
    }