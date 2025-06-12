fn into_ast(self) -> Ast {
        match self {
            Primitive::Literal(lit) => Ast::Literal(lit),
            Primitive::Assertion(assert) => Ast::Assertion(assert),
            Primitive::Dot(span) => Ast::Dot(span),
            Primitive::Perl(cls) => Ast::Class(ast::Class::Perl(cls)),
            Primitive::Unicode(cls) => Ast::Class(ast::Class::Unicode(cls)),
        }
    }