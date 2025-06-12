// Answer 0

#[test]
fn test_decode_utf8_invalid_utf8_sequence() {
    let input: &[u8] = &[0b1110_0000, 0b10_000000]; // Invalid, missing third byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_surrogate_codepoint() {
    let input: &[u8] = &[0b1110_0000, 0b10_000000, 0b10_000000]; // Valid byte sequence but leads to surrogate codepoint
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_too_short() {
    let input: &[u8] = &[0b1110_0001]; // Only one byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_start_byte() {
    let input: &[u8] = &[0b1110_1111, 0b10_000000]; // Valid first byte but still invalid due to not enough bytes
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

