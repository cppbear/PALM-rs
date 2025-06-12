// Answer 0

#[test]
fn test_decode_utf8_single_byte_valid() {
    let input = [0x7F]; // Maximum valid single-byte UTF-8 character
    let expected = Some(('\u{7F}', 1)); // '\u{7F}' corresponds to the character
    let result = decode_utf8(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_utf8_single_byte_valid_zero() {
    let input = [0x00]; // Minimum valid single-byte UTF-8 character
    let expected = Some(('\u{00}', 1)); // '\u{00}' corresponds to the NULL character
    let result = decode_utf8(&input);
    assert_eq!(result, expected);
} 

#[test]
fn test_decode_utf8_empty_input() {
    let input: &[u8] = &[]; // Empty input
    let expected = None; // Should return None for no input
    let result = decode_utf8(input);
    assert_eq!(result, expected);
} 

#[test]
fn test_decode_utf8_invalid_first_byte() {
    let input = [0xC0]; // Invalid UTF-8 sequence
    let expected = None; // The first byte alone doesn't form a valid character
    let result = decode_utf8(&input);
    assert_eq!(result, expected);
} 

#[test]
fn test_decode_utf8_invalid_last_byte() {
    let input = [0xC2, 0x28]; // Invalid second byte
    let expected = None; // Should return None for invalid sequence
    let result = decode_utf8(&input);
    assert_eq!(result, expected);
} 

#[test]
fn test_decode_utf8_surrogate() {
    let input = [0xED, 0xA0, 0x80]; // Surrogate pair in UTF-8
    let expected = None; // Should return None because surrogate codepoints are out of range
    let result = decode_utf8(&input);
    assert_eq!(result, expected);
}

