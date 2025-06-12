// Answer 0

#[test]
fn test_hex_literal_kind_x_digits() {
    let kind = HexLiteralKind::X;
    assert_eq!(kind.digits(), 2);
}

#[test]
fn test_hex_literal_kind_unicode_short_digits() {
    let kind = HexLiteralKind::UnicodeShort;
    assert_eq!(kind.digits(), 4);
}

#[test]
fn test_hex_literal_kind_unicode_long_digits() {
    let kind = HexLiteralKind::UnicodeLong;
    assert_eq!(kind.digits(), 8);
}

