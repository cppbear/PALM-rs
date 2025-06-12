// Answer 0

#[test]
fn test_decode_utf8_invalid_two_byte_sequence() {
    let input: &[u8] = &[0b110_00010, 0b01000000]; // This should produce a codepoint that's out of range (0x40)
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_three_byte_sequence() {
    let input: &[u8] = &[0b1110_0001, 0b10111111, 0b00000000]; // Invalid due to b1 being 0b10111111
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_four_byte_sequence() {
    let input: &[u8] = &[0b11110_000, 0b10000000, 0b10000000, 0b10000000]; // This should produce an out of range codepoint (0x200000)
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_too_short_for_valid_sequence() {
    let input: &[u8] = &[0b110_00010]; // One byte, not enough for a valid 2-byte sequence
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

