// Answer 0

#[test]
fn test_byte_with_valid_hex_escape() {
    // Given a Literal representing a hex escape with a valid character
    let literal = Literal {
        span: Span { start: Position(0), end: Position(1) },
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: 'ÿ', // Unicode character that corresponds to u8 value 255
    };

    // When calling byte method
    let result = literal.byte();

    // Then it should return Some(255)
    assert_eq!(result, Some(255));
}

#[test]
fn test_byte_with_valid_character_not_hex_escape() {
    // Given a Literal representing a non-hex escape character
    let literal = Literal {
        span: Span { start: Position(0), end: Position(1) },
        kind: LiteralKind::Verbatim,
        c: 'A', // Unicode character that corresponds to u8 value 65
    };

    // When calling byte method
    let result = literal.byte();

    // Then it should return None
    assert_eq!(result, None);
}

#[test]
fn test_byte_with_valid_hex_escape_above_max_u8() {
    // Given a Literal representing a hex escape character above the u8 limit
    let literal = Literal {
        span: Span { start: Position(0), end: Position(1) },
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
        c: 'ȿ', // Unicode character that corresponds to u8 value 255
    };

    // When calling byte method
    let result = literal.byte();

    // Then it should return None since it does not satisfy the `c as u32 <= 255` constraint
    assert_eq!(result, None);
}

