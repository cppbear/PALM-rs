// Answer 0

#[test]
fn test_span_literal() {
    struct Span {
        start: usize,
        end: usize,
    }

    struct Literal {
        span: Span,
    }

    enum Ast {
        Empty(Span),
        Flags { span: Span },
        Literal(Literal),
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
                Ast::Literal(ref x) => &x.span,
                Ast::Dot(ref span) => span,
                Ast::Assertion { ref span } => span,
                Ast::Class { ref span } => &span,
                Ast::Repetition { ref span } => span,
                Ast::Group { ref span } => span,
                Ast::Alternation { ref span } => span,
                Ast::Concat { ref span } => span,
            }
        }
    }

    let literal = Literal { span: Span { start: 0, end: 5 } };
    let ast = Ast::Literal(literal);
    
    let result = ast.span();
    
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 5);
}

#[test]
fn test_span_empty() {
    struct Span {
        start: usize,
        end: usize,
    }

    enum Ast {
        Empty(Span),
        Literal(Literal),
    }

    impl Ast {
        pub fn span(&self) -> &Span {
            match *self {
                Ast::Empty(ref span) => span,
                Ast::Literal(ref x) => &x.span,
            }
        }
    }

    let empty_span = Span { start: 0, end: 0 };
    let ast = Ast::Empty(empty_span);
    
    let result = ast.span();
    
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 0);
}

