// Answer 0

#[test]
fn test_decode_utf8_invalid_first_byte() {
    let src: &[u8] = &[0b11111000, 0b10000000, 0b10000000, 0b10000000]; // Valid prefix but invalid continuation
    assert_eq!(decode_utf8(src), None);
}

#[test]
fn test_decode_utf8_invalid_continuation_byte() {
    let src: &[u8] = &[0b11110000, 0b11000000, 0b10000000, 0b00000000]; // Valid prefix, invalid continuation in second byte
    assert_eq!(decode_utf8(src), None);
}

#[test]
fn test_decode_utf8_valid_four_byte_sequence() {
    let src: &[u8] = &[0b11110000, 0b10010000, 0b10010000, 0b10000000]; // Valid 4-byte sequence (U+10000)
    assert_eq!(decode_utf8(src), Some(('𐀀', 4)));
}

#[test]
fn test_decode_utf8_too_short_for_four_byte() {
    let src: &[u8] = &[0b11110000, 0b10010000, 0b10010000]; // Valid prefix but too short (needs 4 bytes)
    assert_eq!(decode_utf8(src), None);
}

#[test]
fn test_decode_utf8_invalid_length_few_bytes() {
    let src: &[u8] = &[0b11110000, 0b10000000]; // Valid start but not enough bytes
    assert_eq!(decode_utf8(src), None);
}

