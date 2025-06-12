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
    let span = Span { start: 0, end: 0 };
    let ast = Ast::Empty(span);
    let result = ast.span();
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 0);
}

#[test]
fn test_span_flags() {
    let span = Span { start: 1, end: 5 };
    let ast = Ast::Flags { span };
    let result = ast.span();
    assert_eq!(result.start, 1);
    assert_eq!(result.end, 5);
}

#[test]
fn test_span_literal() {
    let span = Span { start: 2, end: 3 };
    let ast = Ast::Literal { span };
    let result = ast.span();
    assert_eq!(result.start, 2);
    assert_eq!(result.end, 3);
}

#[test]
fn test_span_dot() {
    let span = Span { start: 3, end: 4 };
    let ast = Ast::Dot(span);
    let result = ast.span();
    assert_eq!(result.start, 3);
    assert_eq!(result.end, 4);
}

#[test]
fn test_span_assertion() {
    let span = Span { start: 4, end: 5 };
    let ast = Ast::Assertion { span };
    let result = ast.span();
    assert_eq!(result.start, 4);
    assert_eq!(result.end, 5);
}

#[test]
fn test_span_class() {
    let span = Span { start: 5, end: 6 };
    let ast = Ast::Class { span };
    let result = ast.span();
    assert_eq!(result.start, 5);
    assert_eq!(result.end, 6);
}

#[test]
fn test_span_repetition() {
    let span = Span { start: 6, end: 7 };
    let ast = Ast::Repetition { span };
    let result = ast.span();
    assert_eq!(result.start, 6);
    assert_eq!(result.end, 7);
}

#[test]
fn test_span_group() {
    let span = Span { start: 7, end: 8 };
    let ast = Ast::Group { span };
    let result = ast.span();
    assert_eq!(result.start, 7);
    assert_eq!(result.end, 8);
}

#[test]
fn test_span_alternation() {
    let span = Span { start: 8, end: 9 };
    let ast = Ast::Alternation { span };
    let result = ast.span();
    assert_eq!(result.start, 8);
    assert_eq!(result.end, 9);
}

#[test]
fn test_span_concat() {
    let span = Span { start: 9, end: 10 };
    let ast = Ast::Concat { span };
    let result = ast.span();
    assert_eq!(result.start, 9);
    assert_eq!(result.end, 10);
}

