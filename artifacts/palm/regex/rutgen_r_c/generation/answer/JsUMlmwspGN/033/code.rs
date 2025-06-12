// Answer 0

#[test]
fn test_decode_utf8_valid_3_byte_sequence() {
    let input = [0b1110_0000, 0b1000_0000, 0b1000_0001]; // U+0801
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('ࠀ', 3)));
}

#[test]
fn test_decode_utf8_invalid_3_byte_sequence() {
    let input = [0b1110_0000, 0b1000_0000, 0b0000_0001]; // 0b0000_0001 is not TAG_CONT
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_length_3_byte_sequence() {
    let input = [0b1110_0000, 0b1000_0000]; // Length is less than 3
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_first_byte() {
    let input = [0b110_00000, 0b1000_0000]; // Invalid as it should be 3 bytes
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_zero_length() {
    let input: &[u8] = &[];
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

