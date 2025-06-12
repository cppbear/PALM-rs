// Answer 0

#[test]
fn test_byte_with_short_hex() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: 'a', // ASCII character, within byte range
    };
    assert_eq!(literal.byte(), Some(97)); // ASCII value of 'a'
}

#[test]
fn test_byte_with_non_hex_kind() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::Verbatim,
        c: 'a', // ASCII character, within byte range
    };
    assert_eq!(literal.byte(), None);
}

#[test]
fn test_byte_with_out_of_range_char() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: 'ÿ', // Unicode character (ASCII 255), on the edge of byte range
    };
    assert_eq!(literal.byte(), Some(255));
}

#[test]
fn test_byte_with_invalid_hex() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal {
        span,
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: '€', // Not within the byte range
    };
    assert_eq!(literal.byte(), None);
}

