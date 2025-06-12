fn visit_post(&mut self, ast: &Ast) -> fmt::Result {
        use ast::Class;

        match *ast {
            Ast::Empty(_) => Ok(()),
            Ast::Flags(ref x) => self.fmt_set_flags(x),
            Ast::Literal(ref x) => self.fmt_literal(x),
            Ast::Dot(_) => self.wtr.write_str("."),
            Ast::Assertion(ref x) => self.fmt_assertion(x),
            Ast::Class(Class::Perl(ref x)) => self.fmt_class_perl(x),
            Ast::Class(Class::Unicode(ref x)) => self.fmt_class_unicode(x),
            Ast::Class(Class::Bracketed(ref x)) => {
                self.fmt_class_bracketed_post(x)
            }
            Ast::Repetition(ref x) => self.fmt_repetition(x),
            Ast::Group(ref x) => self.fmt_group_post(x),
            Ast::Alternation(_) => Ok(()),
            Ast::Concat(_) => Ok(()),
        }
    }