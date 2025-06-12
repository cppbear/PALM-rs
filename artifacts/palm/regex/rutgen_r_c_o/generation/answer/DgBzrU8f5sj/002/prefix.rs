// Answer 0

#[test]
fn test_byte_with_kind_verbatim() {
    let span = Span { start: 0, end: 1 };
    let literal = Literal {
        span,
        kind: LiteralKind::Verbatim,
        c: 'A', // c = 65 which is <= 255
    };
    literal.byte();
}

#[test]
fn test_byte_with_kind_punctuation() {
    let span = Span { start: 0, end: 1 };
    let literal = Literal {
        span,
        kind: LiteralKind::Punctuation,
        c: 'B', // c = 66 which is <= 255
    };
    literal.byte();
}

#[test]
fn test_byte_with_kind_octal() {
    let span = Span { start: 0, end: 1 };
    let literal = Literal {
        span,
        kind: LiteralKind::Octal,
        c: 'C', // c = 67 which is <= 255
    };
    literal.byte();
}

#[test]
fn test_byte_with_kind_hex_brace() {
    let span = Span { start: 0, end: 1 };
    let literal = Literal {
        span,
        kind: LiteralKind::HexBrace(HexLiteralKind::X),
        c: 'D', // c = 68 which is <= 255
    };
    literal.byte();
}

#[test]
fn test_byte_with_kind_special() {
    let span = Span { start: 0, end: 1 };
    let literal = Literal {
        span,
        kind: LiteralKind::Special(SpecialLiteralKind::SomeSpecial),
        c: 'E', // c = 69 which is <= 255
    };
    literal.byte();
}

