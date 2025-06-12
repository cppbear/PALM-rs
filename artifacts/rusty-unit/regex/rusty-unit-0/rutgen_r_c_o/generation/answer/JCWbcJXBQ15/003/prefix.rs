// Answer 0

#[test]
fn test_hex_literal_kind_x() {
    let hex_literal = HexLiteralKind::X;
    let result = hex_literal.digits();
}

#[test]
fn test_hex_literal_kind_unicode_short() {
    let hex_literal = HexLiteralKind::UnicodeShort;
    let result = hex_literal.digits();
}

#[test]
fn test_hex_literal_kind_unicode_long() {
    let hex_literal = HexLiteralKind::UnicodeLong;
    let result = hex_literal.digits();
}

