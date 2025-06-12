// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Primitive {
    Literal(Literal),
    Assertion(Assertion),
    Dot(Span),
    Perl(Perl),
    Unicode(Unicode),
}

#[derive(Debug)]
struct Literal {
    span: Span,
}

#[derive(Debug)]
struct Assertion {
    span: Span,
}

#[derive(Debug)]
struct Perl {
    span: Span,
}

#[derive(Debug)]
struct Unicode {
    span: Span,
}

#[test]
fn test_span_literal() {
    let span = Span { start: 0, end: 5 };
    let literal = Literal { span };
    let primitive = Primitive::Literal(literal);
    
    assert_eq!(primitive.span().start, 0);
    assert_eq!(primitive.span().end, 5);
}

#[test]
fn test_span_assertion() {
    let span = Span { start: 1, end: 3 };
    let assertion = Assertion { span };
    let primitive = Primitive::Assertion(assertion);
    
    assert_eq!(primitive.span().start, 1);
    assert_eq!(primitive.span().end, 3);
}

#[test]
fn test_span_dot() {
    let span = Span { start: 2, end: 4 };
    let primitive = Primitive::Dot(span);
    
    assert_eq!(primitive.span().start, 2);
    assert_eq!(primitive.span().end, 4);
}

#[test]
fn test_span_perl() {
    let span = Span { start: 3, end: 6 };
    let perl = Perl { span };
    let primitive = Primitive::Perl(perl);
    
    assert_eq!(primitive.span().start, 3);
    assert_eq!(primitive.span().end, 6);
}

#[test]
fn test_span_unicode() {
    let span = Span { start: 4, end: 7 };
    let unicode = Unicode { span };
    let primitive = Primitive::Unicode(unicode);
    
    assert_eq!(primitive.span().start, 4);
    assert_eq!(primitive.span().end, 7);
}

