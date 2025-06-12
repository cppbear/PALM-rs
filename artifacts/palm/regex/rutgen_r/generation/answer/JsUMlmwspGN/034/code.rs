// Answer 0

#[test]
fn test_decode_utf8_valid_three_byte_sequence() {
    let input: &[u8] = &[0b1110_0000, 0b1010_0001, 0b1010_0001]; // U+00A1 (¡)
    let expected = Some(('¡', 3));
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_utf8_valid_three_byte_sequence_edge() {
    let input: &[u8] = &[0b1110_0000, 0b1011_1111, 0b1011_1111]; // U+00FF (ÿ)
    let expected = Some(('ÿ', 3));
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_utf8_invalid_three_byte_sequence() {
    let input: &[u8] = &[0b1110_0000, 0b1111_1111, 0b1010_0001]; // Invalid continuation byte
    let expected = None;
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_utf8_too_short_three_byte_sequence() {
    let input: &[u8] = &[0b1110_0000, 0b1010_0001]; // Too short
    let expected = None;
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

