// Answer 0

#[test]
fn test_decode_utf8_invalid_sequence() {
    let src: &[u8] = &[0b11110_0000, 0b10000010, 0b10000100, 0b00100000]; // valid first 3 bytes, invalid last byte
    let result = decode_utf8(src);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_correct_four_byte_sequence() {
    let src: &[u8] = &[0b11110_000, 0b10000010, 0b10000100, 0b10000000]; // valid four-byte UTF-8 character (U+20000)
    let result = decode_utf8(src);
    assert_eq!(result, Some(('😄', 4))); // Adjust the expected character as necessary
}

#[test]
fn test_decode_utf8_too_short_sequence() {
    let src: &[u8] = &[0b11110_101]; // insufficient bytes for a four-byte sequence
    let result = decode_utf8(src);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_surrogate_codepoint() {
    let src: &[u8] = &[0b11110_111, 0b10111111, 0b10111111, 0b10111111]; // valid four bytes but results in a surrogate code point
    let result = decode_utf8(src);
    assert_eq!(result, None);
}

