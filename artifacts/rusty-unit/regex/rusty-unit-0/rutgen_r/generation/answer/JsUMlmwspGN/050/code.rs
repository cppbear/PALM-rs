// Answer 0

#[test]
fn test_decode_valid_utf8_sequence_four_bytes() {
    // Test valid UTF-8 sequence for character U+10000 (a valid character in the supplementary planes).
    let input: &[u8] = &[0b11110_000, 0b1000_0001, 0b1000_0000, 0b1000_0000]; // Represents U+10000
    let expected = Some(('\u{10000}', 4));
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_valid_utf8_sequence_four_bytes_boundary() {
    // Test valid UTF-8 sequence for character U+10FFFF (the highest valid Unicode codepoint).
    let input: &[u8] = &[0b11110_000, 0b1011_1111, 0b1011_1111, 0b1011_1111]; // Represents U+10FFFF
    let expected = Some(('\u{10FFFF}', 4));
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_invalid_utf8_length_too_short() {
    // Test with a sequence that's too short to decode as four bytes.
    let input: &[u8] = &[0b11110_000, 0b1000_0001, 0b1000_0000]; // 3 bytes, should return None
    let expected = None;
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_invalid_utf8_first_byte() {
    // Test an invalid first byte, which is not valid UTF-8.
    let input: &[u8] = &[0b11111_111, 0b1000_0001, 0b1000_0000, 0b1000_0000]; // Invalid first byte
    let expected = None;
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

