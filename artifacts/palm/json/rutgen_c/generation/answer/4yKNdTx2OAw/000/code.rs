// Answer 0

#[test]
fn test_decode_four_hex_digits_valid() {
    assert_eq!(decode_four_hex_digits(0x30, 0x30, 0x30, 0x30), Some(0x0030)); // '0'
    assert_eq!(decode_four_hex_digits(0x61, 0x62, 0x63, 0x64), Some(0x6162)); // 'abcd'
    assert_eq!(decode_four_hex_digits(0x39, 0x39, 0x39, 0x39), Some(0x3939)); // '9999'
}

#[test]
fn test_decode_four_hex_digits_invalid() {
    assert_eq!(decode_four_hex_digits(0xFF, 0x00, 0x00, 0x00), None); // Invalid hex
    assert_eq!(decode_four_hex_digits(0x00, 0x00, 0xFF, 0xFF), None); // Invalid hex
}

#[test]
fn test_decode_four_hex_digits_edge_cases() {
    assert_eq!(decode_four_hex_digits(0x30, 0x30, 0x30, 0x31), Some(0x0031)); // '0' -> '1'
    assert_eq!(decode_four_hex_digits(0xFF, 0xFF, 0xFF, 0xFF), None); // All invalid
}

