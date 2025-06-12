// Answer 0

#[test]
fn test_decode_utf8_four_byte_valid() {
    let input: &[u8] = &[0b11110_000, 0b100_00010, 0b100_00010, 0b100_00001]; // U+100002
    let expected = Some(('𐀁', 4));
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_utf8_four_byte_invalid_too_short() {
    let input: &[u8] = &[0b11110_000, 0b100_00010, 0b100_00010]; // Missing one byte
    let expected = None;
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_utf8_four_byte_invalid_continuation_byte() {
    let input: &[u8] = &[0b11110_000, 0b100_00010, 0b100_00010, 0b000_00001]; // Last byte invalid
    let expected = None;
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_utf8_four_byte_invalid_codepoint_out_of_range() {
    let input: &[u8] = &[0b11110_000, 0b100_00010, 0b100_00010, 0b11111111]; // Out of range codepoint
    let expected = None;
    let result = decode_utf8(input);
    assert_eq!(result, expected);
}

