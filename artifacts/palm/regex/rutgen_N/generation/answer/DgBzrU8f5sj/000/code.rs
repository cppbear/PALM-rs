// Answer 0

#[derive(Debug)]
struct Literal {
    c: char,
    kind: LiteralKind,
}

#[derive(Debug, PartialEq)]
enum LiteralKind {
    HexFixed(HexLiteralKind),
}

#[derive(Debug, PartialEq)]
enum HexLiteralKind {
    X,
}

impl Literal {
    pub fn byte(&self) -> Option<u8> {
        let short_hex = LiteralKind::HexFixed(HexLiteralKind::X);
        if self.c as u32 <= 255 && self.kind == short_hex {
            Some(self.c as u8)
        } else {
            None
        }
    }
}

#[test]
fn test_byte_returns_some_for_valid_hex_escape() {
    let literal = Literal {
        c: '\x7F', // valid ASCII character
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
    };
    assert_eq!(literal.byte(), Some(127));
}

#[test]
fn test_byte_returns_none_for_non_hex_escape() {
    let literal = Literal {
        c: 'A', // not a hex escape
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
    };
    assert_eq!(literal.byte(), None);
}

#[test]
fn test_byte_returns_none_for_out_of_bounds_character() {
    let literal = Literal {
        c: '\u{0100}', // character with a code point > 255
        kind: LiteralKind::HexFixed(HexLiteralKind::X),
    };
    assert_eq!(literal.byte(), None);
}

#[test]
fn test_byte_returns_none_for_different_kind() {
    let literal = Literal {
        c: 'a',
        kind: LiteralKind::HexFixed(HexLiteralKind::X), // valid character but should return None if kind doesn't match the condition
    };
    assert_eq!(literal.byte(), None);
}

#[test]
fn test_byte_with_non_hex_fixed_kind() {
    let literal = Literal {
        c: '\x7F', // valid ASCII character
        kind: LiteralKind::HexFixed(HexLiteralKind::Y), // using a different kind
    };
    assert_eq!(literal.byte(), None);
}

