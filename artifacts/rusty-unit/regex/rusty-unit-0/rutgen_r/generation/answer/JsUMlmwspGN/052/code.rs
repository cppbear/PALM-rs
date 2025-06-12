// Answer 0

#[test]
fn test_decode_utf8_invalid_sequence_too_long() {
    let input = vec![0b11110_0000, 0b10000000, 0b10000000, 0b10000000]; // This should represent a valid sequence but does not fall into the valid range
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_sequence_out_of_range() {
    let input = vec![0b11110_0000, 0b10000000, 0b10000000, 0b01000000]; // This creates an out of range codepoint
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_valid_sequence() {
    let input = vec![0b11110_1000, 0b10101100, 0b10010000, 0b10101100]; // Represents valid codepoint 0x1000AC
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

