// Answer 0

#[test]
fn test_byte_char_above_ascii() {
    let literal = Literal {
        span: Span { start: Position(0), end: Position(1) },
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: '\u{0100}',
    };
    literal.byte();
}

#[test]
fn test_byte_char_unicode() {
    let literal = Literal {
        span: Span { start: Position(0), end: Position(1) },
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: '\u{FFFF}',
    };
    literal.byte();
}

#[test]
fn test_byte_char_high_unicode() {
    let literal = Literal {
        span: Span { start: Position(0), end: Position(1) },
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: '\u{1234}',
    };
    literal.byte();
}

