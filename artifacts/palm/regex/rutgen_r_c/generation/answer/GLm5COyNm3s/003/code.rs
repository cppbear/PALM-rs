// Answer 0

#[test]
fn test_span_with_literal() {
    let span = Span { start: Position::new(0), end: Position::new(1) };
    let primitive = Primitive::Literal(Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' });
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_with_assertion() {
    let span = Span { start: Position::new(1), end: Position::new(2) };
    let primitive = Primitive::Assertion(Assertion { span, kind: AssertionKind::WordBoundary });
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_with_dot() {
    let span = Span { start: Position::new(2), end: Position::new(3) };
    let primitive = Primitive::Dot(span);
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_with_perl_class() {
    let span = Span { start: Position::new(3), end: Position::new(4) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class);
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_with_unicode_class() {
    let span = Span { start: Position::new(4), end: Position::new(5) };
    let unicode_class = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(unicode_class);
    assert_eq!(primitive.span(), &span);
}

