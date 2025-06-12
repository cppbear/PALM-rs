// Answer 0

#[test]
fn test_decode_utf8_invalid_sequence_too_few_bytes() {
    let input = [0b1110_0000]; // Only one byte, should return None
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_sequence_unexpected_continuation_byte() {
    let input = [0b1110_0000, 0b10_000000, 0b10_000000]; // Invalid second byte, should return None
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_surrogate_codepoint() {
    let input = [0b1110_0000, 0b1000_0000, 0b1000_0000]; // Valid three-byte sequence for U+800
    // but shouldn't have valid cp (surrogate), should return None
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_first_byte() {
    let input = [0b1100_0001, 0b1000_0000]; // Valid first byte but invalid continuation byte, should return None
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_fourth_byte() {
    let input = [0b11110_000, 0b1000_0000, 0b1000_0000, 0b10_000000]; // Invalid fourth byte, should return None
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

