// Answer 0

#[test]
fn test_decode_utf8_invalid_sequence_with_three_bytes() {
    let input: &[u8] = &[0b1110_0010, 0b0000_0001, 0b1000_0001]; // Invalid because b1 is not a valid continuation byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_sequence_with_three_bytes() {
    let input: &[u8] = &[0b1110_0001, 0b1000_0000, 0b1000_0000]; // Valid sequence for the character U+0400 (Ѐ)
    let result = decode_utf8(input);
    assert_eq!(result, Some(('Ѐ', 3)));
}

#[test]
fn test_decode_utf8_invalid_first_byte_not_shortest() {
    let input: &[u8] = &[0b1100_0011, 0b0000_0010]; // This is valid, but the first byte is not the shortest representation for the codepoint
    let result = decode_utf8(input);
    assert_eq!(result, Some(('2', 2))); // The expected character is '2'
}

#[test]
fn test_decode_utf8_incomplete_sequence() {
    let input: &[u8] = &[0b1110_0001, 0b1000_0000]; // Incomplete, should return None
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

