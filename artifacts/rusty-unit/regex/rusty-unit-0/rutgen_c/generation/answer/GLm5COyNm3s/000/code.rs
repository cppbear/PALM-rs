// Answer 0

#[test]
fn test_span_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let primitive = Primitive::Literal(literal);
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_assertion() {
    let span = Span { start: Position(2), end: Position(3) };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::StartOfLine };
    let primitive = Primitive::Assertion(assertion);
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_dot() {
    let span = Span { start: Position(4), end: Position(5) };
    let primitive = Primitive::Dot(span.clone());
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_perl() {
    let span = Span { start: Position(6), end: Position(7) };
    let class_perl = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(class_perl);
    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_unicode() {
    let span = Span { start: Position(8), end: Position(9) };
    let class_unicode = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(class_unicode);
    assert_eq!(primitive.span(), &span);
}

