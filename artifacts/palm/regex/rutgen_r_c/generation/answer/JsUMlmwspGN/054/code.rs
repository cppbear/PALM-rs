// Answer 0

#[test]
fn test_decode_utf8_invalid_byte() {
    let result = decode_utf8(&[0xFF]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_empty_input() {
    let result = decode_utf8(&[]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_incomplete_sequence_two_bytes() {
    let result = decode_utf8(&[0b11000010]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_incomplete_sequence_three_bytes() {
    let result = decode_utf8(&[0b11100010]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_following_byte_two() {
    let result = decode_utf8(&[0b11000010, 0b00111111]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_following_byte_three() {
    let result = decode_utf8(&[0b11100010, 0b00111111, 0b00111111]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_following_byte_four() {
    let result = decode_utf8(&[0b11110000, 0b00111111, 0b00111111, 0b00111111]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_codepoint() {
    let result = decode_utf8(&[0b11011111, 0b11111111]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_surrogate_codepoint() {
    let result = decode_utf8(&[0b11110000, 0b10011111, 0b10111111, 0b10000000]);
    assert_eq!(result, None);
}

