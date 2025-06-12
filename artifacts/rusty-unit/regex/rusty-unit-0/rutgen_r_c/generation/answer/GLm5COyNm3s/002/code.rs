// Answer 0

#[test]
fn test_span_primitive_literal() {
    let span = Span { start: Position::from(0), end: Position::from(5) };
    let literal = Literal { span, kind: LiteralKind::Unicode, c: 'a' };
    let primitive = Primitive::Literal(literal);
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_primitive_assertion() {
    let span = Span { start: Position::from(0), end: Position::from(3) };
    let assertion = Assertion { span, kind: AssertionKind::WordBoundary };
    let primitive = Primitive::Assertion(assertion);
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_primitive_dot() {
    let span = Span { start: Position::from(2), end: Position::from(4) };
    let primitive = Primitive::Dot(span);
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_primitive_perl() {
    let span = Span { start: Position::from(1), end: Position::from(6) };
    let class_perl = ClassPerl { span, kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(class_perl);
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_primitive_unicode() {
    let span = Span { start: Position::from(0), end: Position::from(100) };
    let class_unicode = ClassUnicode { span, negated: true, kind: ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(class_unicode);
    assert_eq!(primitive.span(), &span);
}

