// Answer 0

#[test]
fn test_primitive_span_literal() {
    let literal = Literal {
        span: Span { start: Position(0), end: Position(1) },
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    };
    
    let primitive = Primitive::Literal(literal);
    assert_eq!(primitive.span(), &Span { start: Position(0), end: Position(1) });
}

#[test]
fn test_primitive_span_assertion() {
    let assertion = Assertion {
        span: Span { start: Position(2), end: Position(3) },
        kind: AssertionKind::StartOfInput,
    };
    
    let primitive = Primitive::Assertion(assertion);
    assert_eq!(primitive.span(), &Span { start: Position(2), end: Position(3) });
}

#[test]
fn test_primitive_span_dot() {
    let span = Span { start: Position(4), end: Position(5) };
    let primitive = Primitive::Dot(span);
    assert_eq!(primitive.span(), &Span { start: Position(4), end: Position(5) });
}

#[test]
fn test_primitive_span_perl() {
    let perl_class = ClassPerl {
        span: Span { start: Position(6), end: Position(7) },
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    
    let primitive = Primitive::Perl(perl_class);
    assert_eq!(primitive.span(), &Span { start: Position(6), end: Position(7) });
}

#[test]
fn test_primitive_span_unicode() {
    let unicode_class = ClassUnicode {
        span: Span { start: Position(8), end: Position(9) },
        negated: false,
        kind: ClassUnicodeKind::Letter,
    };
    
    let primitive = Primitive::Unicode(unicode_class);
    assert_eq!(primitive.span(), &Span { start: Position(8), end: Position(9) });
}

