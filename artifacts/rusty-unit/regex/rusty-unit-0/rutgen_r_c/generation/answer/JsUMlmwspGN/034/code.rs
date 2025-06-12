// Answer 0

#[test]
fn test_decode_utf8_valid_3_byte_sequence() {
    let input: &[u8] = &[0b1110_0000, 0b1000_0001, 0b1010_0001]; // U+0081
    assert_eq!(decode_utf8(input), Some(('¢', 3))); // expected char is '¢'
}

#[test]
fn test_decode_utf8_valid_3_byte_sequence_edge() {
    let input: &[u8] = &[0b1110_0000, 0b1011_1111, 0b1011_1111]; // U+00FF
    assert_eq!(decode_utf8(input), Some(('ÿ', 3))); // expected char is 'ÿ'
}

#[test]
fn test_decode_utf8_invalid_first_byte() {
    let input: &[u8] = &[0b1100_0000, 0b0100_0001]; // invalid first byte
    assert_eq!(decode_utf8(input), None);
}

#[test]
fn test_decode_utf8_valid_2_byte_sequence() {
    let input: &[u8] = &[0b1100_0001, 0b1000_0001]; // U+0101
    assert_eq!(decode_utf8(input), Some(('Ā', 2))); // expected char is 'Ā'
}

#[test]
fn test_decode_utf8_invalid_too_short() {
    let input: &[u8] = &[0b1110_0000]; // incomplete sequence
    assert_eq!(decode_utf8(input), None);
}

#[test]
fn test_decode_utf8_valid_4_byte_sequence() {
    let input: &[u8] = &[0b11110_000, 0b1000_0001, 0b1010_0001, 0b1010_0001]; // U+010101
    assert_eq!(decode_utf8(input), Some(('𐁁', 4))); // expected char is '𐁁'
}

#[test]
fn test_decode_utf8_invalid_control_byte() {
    let input: &[u8] = &[0b1110_0000, 0b0100_0000, 0b1100_0000]; // invalid byte in sequence
    assert_eq!(decode_utf8(input), None);
}

