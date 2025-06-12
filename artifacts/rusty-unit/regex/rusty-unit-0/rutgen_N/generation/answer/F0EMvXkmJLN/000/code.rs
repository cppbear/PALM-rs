// Answer 0

#[test]
fn test_is_start_byte_valid_utf8() {
    assert_eq!(is_start_byte(0b0_11111111), true); // 0x7F, valid start byte
    assert_eq!(is_start_byte(0b0_10000000), true); // 0x80, valid start byte
    assert_eq!(is_start_byte(0b0_11000000), true); // 0xC0, valid start byte for 2-byte sequence
    assert_eq!(is_start_byte(0b0_11100000), true); // 0xE0, valid start byte for 3-byte sequence
    assert_eq!(is_start_byte(0b0_11110000), true); // 0xF0, valid start byte for 4-byte sequence
}

#[test]
fn test_is_start_byte_invalid_utf8() {
    assert_eq!(is_start_byte(0b0_00000000), false); // 0x00, invalid start byte
    assert_eq!(is_start_byte(0b0_11111110), false); // 0xFE, invalid start byte
    assert_eq!(is_start_byte(0b0_11111101), false); // 0xFD, invalid start byte
    assert_eq!(is_start_byte(0b1_00000000), false); // 0x80, invalid start byte for UTF-8
}

