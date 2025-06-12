// Answer 0

#[test]
fn test_ast_dot_span() {
    struct Span {
        start: usize,
        end: usize,
    }

    enum Ast {
        Empty(Span),
        Flags { span: Span },
        Literal { span: Span },
        Dot(Span),
        Assertion { span: Span },
        Class { span: Span },
        Repetition { span: Span },
        Group { span: Span },
        Alternation { span: Span },
        Concat { span: Span },
    }

    impl Ast {
        pub fn span(&self) -> &Span {
            match *self {
                Ast::Empty(ref span) => span,
                Ast::Flags { ref span } => span,
                Ast::Literal { ref span } => span,
                Ast::Dot(ref span) => span,
                Ast::Assertion { ref span } => span,
                Ast::Class { ref span } => span,
                Ast::Repetition { ref span } => span,
                Ast::Group { ref span } => span,
                Ast::Alternation { ref span } => span,
                Ast::Concat { ref span } => span,
            }
        }
    }

    let span = Span { start: 1, end: 2 };
    let ast_dot = Ast::Dot(span);

    let returned_span = ast_dot.span();
    assert_eq!(returned_span.start, 1);
    assert_eq!(returned_span.end, 2);
}

