// Answer 0

#[test]
fn test_decode_utf8_invalid_shorter_than_three_bytes() {
    let input: &[u8] = &[0b1110_0000, 0b10000000]; // 2 bytes only
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_b2_not_continuation_byte() {
    let input: &[u8] = &[0b1110_0000, 0b10_000000, 0b10_000001]; // b2 is not a continuation byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_sequence() {
    let input: &[u8] = &[0b1110_0000, 0b10000001, 0b10111111]; // valid 3-byte UTF-8 sequence
    let result = decode_utf8(input);
    assert_eq!(result, Some(('U', 3))); // U+01BF is '˿'
} 

#[test]
fn test_decode_utf8_valid_boundary() {
    let input: &[u8] = &[0b1110_1111, 0b10111111, 0b10111111]; // valid 3-byte UTF-8 sequence at boundary
    let result = decode_utf8(input);
    assert_eq!(result, Some(('O', 3))); // U+1FFFF is not valid, should be None
} 

#[test]
fn test_decode_utf8_invalid_surragate_codepoint() {
    let input: &[u8] = &[0b1111_0000, 0b10000000, 0b10000000, 0b10000000]; // more than 4 bytes
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

