// Answer 0

#[test]
fn test_decode_utf8_invalid_sequence_too_short() {
    let input: &[u8] = &[0b11110_0000]; // Valid start byte for 4-byte sequence, but too short
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_sequence_incorrect_continuations() {
    let input: &[u8] = &[0b11110_0000, 0b10000000, 0b10000000, 0b11111111]; // Last byte incorrect
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_sequence() {
    let input: &[u8] = &[0b11110_0000, 0b10010000, 0b10001000, 0b10000000]; // U+100000
    let result = decode_utf8(input);
    assert_eq!(result, Some(('𐀀', 4))); // Should decode to character U+100000
} 

#[test]
fn test_decode_utf8_valid_sequence_surrogate() {
    let input: &[u8] = &[0b11110_0001, 0b10111111, 0b10011111, 0b10111111]; // U+D7FF (surrogate code point)
    let result = decode_utf8(input);
    assert_eq!(result, None);  // Surrogate code points should return None
} 

#[test]
fn test_decode_utf8_valid_sequence_correct_length() {
    let input: &[u8] = &[0b11110_0010, 0b10000010, 0b10000001, 0b10000000]; // Should successfully decode
    let result = decode_utf8(input);
    assert_eq!(result, Some(('𐂁', 4))); // Should decode correctly
}

