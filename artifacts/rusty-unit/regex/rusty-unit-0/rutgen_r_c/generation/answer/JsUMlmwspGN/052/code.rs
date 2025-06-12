// Answer 0

#[test]
fn test_decode_utf8_invalid_four_bytes() {
    let input: &[u8] = &[0b11110_0001, 0b10000000, 0b10000000, 0b10000000]; // Invalid UTF-8 sequence
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_surrogate_code_point() {
    let input: &[u8] = &[0b11110_0000, 0b10000000, 0b10000000, 0b10000000]; // Valid UTF-8 starting but out of range
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_too_few_bytes() {
    let input: &[u8] = &[0b11110_0000, 0b10000000]; // Only two bytes provided for a four-byte sequence
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

