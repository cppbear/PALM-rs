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
    Group { span: Span },
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
            Ast::Class(ref x) => &x.span,
            Ast::Repetition(ref x) => &x.span,
            Ast::Group(ref x) => &x.span,
            Ast::Alternation(ref x) => &x.span,
            Ast::Concat(ref x) => &x.span,
        }
    }
}

#[test]
fn test_span_empty() {
    let ast = Ast::Empty(Span { start: 0, end: 0 });
    let span = ast.span();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 0);
}

#[test]
fn test_span_flags() {
    let ast = Ast::Flags { span: Span { start: 1, end: 2 } };
    let span = ast.span();
    assert_eq!(span.start, 1);
    assert_eq!(span.end, 2);
}

#[test]
fn test_span_literal() {
    let ast = Ast::Literal { span: Span { start: 3, end: 4 } };
    let span = ast.span();
    assert_eq!(span.start, 3);
    assert_eq!(span.end, 4);
}

#[test]
fn test_span_dot() {
    let ast = Ast::Dot(Span { start: 5, end: 6 });
    let span = ast.span();
    assert_eq!(span.start, 5);
    assert_eq!(span.end, 6);
}

#[test]
fn test_span_assertion() {
    let ast = Ast::Assertion { span: Span { start: 7, end: 8 } };
    let span = ast.span();
    assert_eq!(span.start, 7);
    assert_eq!(span.end, 8);
}

#[test]
fn test_span_class() {
    let ast = Ast::Class { span: Span { start: 9, end: 10 } };
    let span = ast.span();
    assert_eq!(span.start, 9);
    assert_eq!(span.end, 10);
}

#[test]
fn test_span_repetition() {
    let ast = Ast::Repetition { span: Span { start: 11, end: 12 } };
    let span = ast.span();
    assert_eq!(span.start, 11);
    assert_eq!(span.end, 12);
}

#[test]
fn test_span_group() {
    let ast = Ast::Group { span: Span { start: 13, end: 14 } };
    let span = ast.span();
    assert_eq!(span.start, 13);
    assert_eq!(span.end, 14);
}

#[test]
fn test_span_alternation() {
    let ast = Ast::Alternation { span: Span { start: 15, end: 16 } };
    let span = ast.span();
    assert_eq!(span.start, 15);
    assert_eq!(span.end, 16);
}

#[test]
fn test_span_concat() {
    let ast = Ast::Concat { span: Span { start: 17, end: 18 } };
    let span = ast.span();
    assert_eq!(span.start, 17);
    assert_eq!(span.end, 18);
}

