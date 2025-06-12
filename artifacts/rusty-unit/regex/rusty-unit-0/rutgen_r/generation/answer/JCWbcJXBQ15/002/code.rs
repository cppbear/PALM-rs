// Answer 0

#[derive(Clone, Copy)]
enum HexLiteralKind {
    X,
    UnicodeShort,
    UnicodeLong,
}

impl HexLiteralKind {
    pub fn digits(&self) -> u32 {
        match *self {
            HexLiteralKind::X => 2,
            HexLiteralKind::UnicodeShort => 4,
            HexLiteralKind::UnicodeLong => 8,
        }
    }
}

#[test]
fn test_digits_unicode_short() {
    let kind = HexLiteralKind::UnicodeShort;
    assert_eq!(kind.digits(), 4);
}

#[test]
fn test_digits_hex_literal() {
    let kind = HexLiteralKind::X;
    assert_eq!(kind.digits(), 2);
}

#[test]
fn test_digits_unicode_long() {
    let kind = HexLiteralKind::UnicodeLong;
    assert_eq!(kind.digits(), 8);
}

