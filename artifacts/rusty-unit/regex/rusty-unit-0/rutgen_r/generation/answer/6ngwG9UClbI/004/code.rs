// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Repetition {
    span: Span,
}

#[derive(Debug)]
enum Ast {
    Empty(Span),
    Flags { span: Span },
    Literal { span: Span },
    Dot(Span),
    Assertion { span: Span },
    Class { span: Span },
    Repetition(Repetition),
    Group { span: Span },
    Alternation { span: Span },
    Concat { span: Span },
}

#[test]
fn test_span_repetition() {
    let span = Span { start: 0, end: 5 };
    let repetition = Repetition { span };
    let ast = Ast::Repetition(repetition);

    assert_eq!(ast.span(), &Span { start: 0, end: 5 });
}

