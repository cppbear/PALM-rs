// Answer 0

#[test]
fn test_decode_utf8_invalid_length() {
    let input: &[u8] = &[0b1110_0000]; // Only one byte, should return None
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_second_byte() {
    let input: &[u8] = &[0b1110_0000, 0b0000_0001]; // Valid first byte, invalid second byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_third_byte() {
    let input: &[u8] = &[0b1110_0000, 0b1000_0000, 0b0000_0001]; // Valid first and second bytes, invalid third byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_three_byte_sequence() {
    let input: &[u8] = &[0b1110_0000, 0b1000_0000, 0b1000_0000]; // Valid UTF-8 sequence (U+800)
    let result = decode_utf8(input);
    assert_eq!(result, Some(('', 3)));
}

