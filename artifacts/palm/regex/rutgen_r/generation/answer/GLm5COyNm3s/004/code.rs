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

impl Primitive {
    fn span(&self) -> &Span {
        match *self {
            Primitive::Literal(ref x) => &x.span,
            Primitive::Assertion(ref x) => &x.span,
            Primitive::Dot(ref span) => span,
            Primitive::Perl(ref x) => &x.span,
            Primitive::Unicode(ref x) => &x.span,
        }
    }
}

#[test]
fn test_span_literal() {
    let literal = Primitive::Literal(Literal {
        span: Span { start: 0, end: 5 },
    });
    assert_eq!(literal.span(), &Span { start: 0, end: 5 });
}

#[test]
fn test_span_assertion() {
    let assertion = Primitive::Assertion(Assertion {
        span: Span { start: 1, end: 3 },
    });
    assert_eq!(assertion.span(), &Span { start: 1, end: 3 });
}

#[test]
fn test_span_dot() {
    let dot = Primitive::Dot(Span { start: 2, end: 4 });
    assert_eq!(dot.span(), &Span { start: 2, end: 4 });
}

#[test]
fn test_span_perl() {
    let perl = Primitive::Perl(Perl {
        span: Span { start: 5, end: 7 },
    });
    assert_eq!(perl.span(), &Span { start: 5, end: 7 });
}

#[test]
fn test_span_unicode() {
    let unicode = Primitive::Unicode(Unicode {
        span: Span { start: 6, end: 8 },
    });
    assert_eq!(unicode.span(), &Span { start: 6, end: 8 });
}

