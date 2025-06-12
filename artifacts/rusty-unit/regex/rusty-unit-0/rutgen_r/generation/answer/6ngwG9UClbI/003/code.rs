// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Ast {
    Empty(Span),
    Flags { span: Span },
    Literal { span: Span },
    Dot(Span),
    Assertion { span: Span },
    Class { span: Span },
    Repetition { span: Span },
    Group(Box<Ast>),
    Alternation { span: Span },
    Concat { span: Span },
}

impl Ast {
    pub fn span(&self) -> &Span {
        match *self {
            Ast::Empty(ref span) => span,
            Ast::Flags(ref x) => &x.span,
            Ast::Literal(ref x) => &x.span,
            Ast::Dot(ref span) => span,
            Ast::Assertion(ref x) => &x.span,
            Ast::Class(ref x) => &x.span(),
            Ast::Repetition(ref x) => &x.span,
            Ast::Group(ref x) => &x.span,
            Ast::Alternation(ref x) => &x.span,
            Ast::Concat(ref x) => &x.span,
        }
    }
}

#[test]
fn test_ast_span_group() {
    let inner_ast = Ast::Literal { span: Span { start: 5, end: 10 } };
    let ast = Ast::Group(Box::new(inner_ast));

    let result_span = ast.span();
    assert_eq!(result_span.start, 5);
    assert_eq!(result_span.end, 10);
}

