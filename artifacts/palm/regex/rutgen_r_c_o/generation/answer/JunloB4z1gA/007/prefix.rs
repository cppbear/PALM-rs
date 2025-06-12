// Answer 0

#[test]
fn test_span_literal_a() {
    let span = Span { start: Position { .. }, end: Position { .. } };
    let literal = Literal { span, kind: LiteralKind::Unicode, c: 'a' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_literal_b() {
    let span = Span { start: Position { .. }, end: Position { .. } };
    let literal = Literal { span, kind: LiteralKind::Unicode, c: 'b' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_literal_z() {
    let span = Span { start: Position { .. }, end: Position { .. } };
    let literal = Literal { span, kind: LiteralKind::Unicode, c: 'z' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_literal_upper_a() {
    let span = Span { start: Position { .. }, end: Position { .. } };
    let literal = Literal { span, kind: LiteralKind::Unicode, c: 'A' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_literal_special_char() {
    let span = Span { start: Position { .. }, end: Position { .. } };
    let literal = Literal { span, kind: LiteralKind::Byte, c: '!' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_literal_bounds() {
    let span = Span { start: Position { .. }, end: Position { .. } };
    let literal = Literal { span, kind: LiteralKind::Unicode, c: 'y' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

