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
fn test_unicode_span() {
    let unicode_span = Span { start: 5, end: 10 };
    let primitive = Primitive::Unicode { span: unicode_span };

    let result = primitive.span();
    assert_eq!(result.start, 5);
    assert_eq!(result.end, 10);
}

#[test]
fn test_literal_span() {
    let literal_span = Span { start: 1, end: 2 };
    let primitive = Primitive::Literal { span: literal_span };

    let result = primitive.span();
    assert_eq!(result.start, 1);
    assert_eq!(result.end, 2);
}

#[test]
fn test_assertion_span() {
    let assertion_span = Span { start: 3, end: 4 };
    let primitive = Primitive::Assertion { span: assertion_span };

    let result = primitive.span();
    assert_eq!(result.start, 3);
    assert_eq!(result.end, 4);
}

#[test]
fn test_dot_span() {
    let dot_span = Span { start: 2, end: 3 };
    let primitive = Primitive::Dot(dot_span);

    let result = primitive.span();
    assert_eq!(result.start, 2);
    assert_eq!(result.end, 3);
}

#[test]
fn test_perl_span() {
    let perl_span = Span { start: 6, end: 7 };
    let primitive = Primitive::Perl { span: perl_span };

    let result = primitive.span();
    assert_eq!(result.start, 6);
    assert_eq!(result.end, 7);
}

