// Answer 0

#[test]
fn test_span_literal_char() {
    let span = Span { start: Position { value: 0 }, end: Position { value: 1 } };
    let literal = Primitive::Literal(Literal { span, kind: LiteralKind::Unicode, c: 'a' });
    let result = literal.span();
}

#[test]
fn test_span_literal_byte() {
    let span = Span { start: Position { value: 100 }, end: Position { value: 101 } };
    let literal = Primitive::Literal(Literal { span, kind: LiteralKind::Byte, c: 'b' });
    let result = literal.span();
}

#[test]
fn test_span_literal_max() {
    let span = Span { start: Position { value: 255 }, end: Position { value: 256 } };
    let literal = Primitive::Literal(Literal { span, kind: LiteralKind::Unicode, c: 'z' });
    let result = literal.span();
}

#[test]
fn test_span_literal_zero_start() {
    let span = Span { start: Position { value: 0 }, end: Position { value: 10 } };
    let literal = Primitive::Literal(Literal { span, kind: LiteralKind::Byte, c: 'c' });
    let result = literal.span();
}

#[test]
fn test_span_literal_end_boundary() {
    let span = Span { start: Position { value: 0 }, end: Position { value: 255 } };
    let literal = Primitive::Literal(Literal { span, kind: LiteralKind::Unicode, c: 'd' });
    let result = literal.span();
}

