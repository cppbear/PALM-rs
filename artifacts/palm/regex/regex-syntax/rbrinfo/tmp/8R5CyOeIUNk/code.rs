fn induct<V: Visitor>(
        &mut self,
        ast: &'a Ast,
        visitor: &mut V,
    ) -> Result<Option<Frame<'a>>, V::Err> {
        Ok(match *ast {
            Ast::Class(ast::Class::Bracketed(ref x)) => {
                self.visit_class(x, visitor)?;
                None
            }
            Ast::Repetition(ref x) => Some(Frame::Repetition(x)),
            Ast::Group(ref x) => Some(Frame::Group(x)),
            Ast::Concat(ref x) if x.asts.is_empty() => None,
            Ast::Concat(ref x) => {
                Some(Frame::Concat {
                    head: &x.asts[0],
                    tail: &x.asts[1..],
                })
            }
            Ast::Alternation(ref x) if x.asts.is_empty() => None,
            Ast::Alternation(ref x) => {
                Some(Frame::Alternation {
                    head: &x.asts[0],
                    tail: &x.asts[1..],
                })
            }
            _ => None,
        })
    }