// Answer 0

#[test]
fn test_decode_utf8_invalid_too_short() {
    let input = &[0b1110_0000, 0b10]; // Invalid: less than 3 bytes
    let result = decode_utf8(input);
}

#[test]
fn test_decode_utf8_invalid_too_short_beyond_limit() {
    let input = &[0b1110_1111, 0b10]; // Invalid: less than 3 bytes
    let result = decode_utf8(input);
}

#[test]
fn test_decode_utf8_invalid_continuation_byte() {
    let input = &[0b1110_0111, 0b11, 0b10]; // Invalid: no trailing continuation byte
    let result = decode_utf8(input);
}

#[test]
fn test_decode_utf8_invalid_continuation_byte_case_variation() {
    let input = &[0b1110_0001, 0b01, 0b10]; // Invalid: still less than 3 bytes
    let result = decode_utf8(input);
}

