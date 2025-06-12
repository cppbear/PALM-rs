// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Primitive {
    Literal { span: Span },
    Assertion { span: Span },
    Dot(Span),
    Perl { span: Span },
    Unicode { span: Span },
}

#[test]
fn test_span_dot() {
    let span = Span { start: 0, end: 1 };
    let primitive = Primitive::Dot(span);
    let result = primitive.span();
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 1);
}

#[test]
fn test_span_literal() {
    let span = Span { start: 2, end: 3 };
    let primitive = Primitive::Literal { span };
    let result = primitive.span();
    assert_eq!(result.start, 2);
    assert_eq!(result.end, 3);
}

#[test]
fn test_span_assertion() {
    let span = Span { start: 4, end: 5 };
    let primitive = Primitive::Assertion { span };
    let result = primitive.span();
    assert_eq!(result.start, 4);
    assert_eq!(result.end, 5);
}

#[test]
fn test_span_perl() {
    let span = Span { start: 6, end: 7 };
    let primitive = Primitive::Perl { span };
    let result = primitive.span();
    assert_eq!(result.start, 6);
    assert_eq!(result.end, 7);
}

#[test]
fn test_span_unicode() {
    let span = Span { start: 8, end: 9 };
    let primitive = Primitive::Unicode { span };
    let result = primitive.span();
    assert_eq!(result.start, 8);
    assert_eq!(result.end, 9);
}

