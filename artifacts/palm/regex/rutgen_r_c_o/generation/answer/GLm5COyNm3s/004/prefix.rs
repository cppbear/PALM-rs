// Answer 0

#[test]
fn test_span_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = ast::Literal { span, kind: LiteralKind::Unicode, c: 'a' };
    let primitive = Primitive::Literal(literal);
    primitive.span();
}

#[test]
fn test_span_assertion() {
    let span = Span { start: Position(0), end: Position(2) };
    let assertion = Assertion { span, kind: AssertionKind::StartOfLine };
    let primitive = Primitive::Assertion(assertion);
    primitive.span();
}

#[test]
fn test_span_dot() {
    let span = Span { start: Position(5), end: Position(10) };
    let primitive = Primitive::Dot(span);
    primitive.span();
}

#[test]
fn test_span_perl() {
    let span = Span { start: Position(3), end: Position(5) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class);
    primitive.span();
}

#[test]
fn test_span_unicode() {
    let span = Span { start: Position(1), end: Position(4) };
    let unicode_class = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(unicode_class);
    primitive.span();
}

#[test]
fn test_span_assertion_negated() {
    let span = Span { start: Position(0), end: Position(0) };
    let assertion = Assertion { span, kind: AssertionKind::WordBoundary };
    let primitive = Primitive::Assertion(assertion);
    primitive.span();
}

