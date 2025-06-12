// Answer 0

#[test]
fn test_utf8_decode_invalid_two_byte_sequence() {
    let input: &[u8] = &[0b110_00001, 0b000_00001]; // Valid b0, but invalid b1
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_utf8_decode_valid_two_byte_sequence() {
    let input: &[u8] = &[0b110_00010, 0b100_00000]; // Valid UTF-8 sequence
    let result = decode_utf8(input);
    assert_eq!(result, Some(('¢', 2)));
}

#[test]
fn test_utf8_decode_too_short_for_three_byte_sequence() {
    let input: &[u8] = &[0b1110_0001]; // Prefix for a three-byte sequence without enough bytes
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_utf8_decode_invalid_three_byte_sequence() {
    let input: &[u8] = &[0b1110_0001, 0b0010_0000, 0b000_00001]; // Valid prefix and second byte, invalid third byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_utf8_decode_too_short_for_four_byte_sequence() {
    let input: &[u8] = &[0b11110_0000]; // Prefix for a four-byte sequence without enough bytes
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_utf8_decode_invalid_four_byte_sequence() {
    let input: &[u8] = &[0b11110_0001, 0b0010_0000, 0b000_00000, 0b000_00001]; // Valid prefix, but third byte is invalid
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

