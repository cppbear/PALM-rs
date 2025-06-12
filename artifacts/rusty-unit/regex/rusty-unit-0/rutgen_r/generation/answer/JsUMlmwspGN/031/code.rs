// Answer 0

#[test]
fn test_decode_utf8_invalid_sequence_too_short() {
    let input: &[u8] = &[0b1110_0000]; // Start of a 3-byte sequence, but only 1 byte is provided
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_second_byte() {
    let input: &[u8] = &[0b1110_0001, 0b10_000000]; // Valid starting byte and a proper continuation byte
    let result = decode_utf8(input);
    assert_eq!(result, None); // No third byte, thus it should return None
}

#[test]
fn test_decode_utf8_invalid_following_byte_too_few() {
    let input: &[u8] = &[0b1110_0000, 0b10_000000]; // Valid starting byte, but only one continuation byte provided
    let result = decode_utf8(input);
    assert_eq!(result, None); // Missing the third byte, thus should return None
}

