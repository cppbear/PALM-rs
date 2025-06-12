// Answer 0

#[test]
fn test_decode_utf8_invalid_sequence_too_short() {
    let src: &[u8] = &[0b11110_0000, 0b10000000, 0b10000000]; // last byte missing
    assert_eq!(decode_utf8(src), None);
}

#[test]
fn test_decode_utf8_invalid_first_byte() {
    let src: &[u8] = &[0b11110_0001, 0b10000000, 0b10000000, 0b10000000]; // invalid first byte
    assert_eq!(decode_utf8(src), None);
}

#[test]
fn test_decode_utf8_invalid_continuation_byte() {
    let src: &[u8] = &[0b11110_0000, 0b10000001, 0b10000000, 0b10000000]; // second byte not valid
    assert_eq!(decode_utf8(src), None);
}

#[test]
fn test_decode_utf8_invalid_continuation_byte_second() {
    let src: &[u8] = &[0b11110_0000, 0b10000000, 0b00000000, 0b10000000]; // third byte not valid
    assert_eq!(decode_utf8(src), None);
}

#[test]
fn test_decode_utf8_invalid_continuation_byte_third() {
    let src: &[u8] = &[0b11110_0000, 0b10000000, 0b10000001, 0b10000000]; // fourth byte not valid
    assert_eq!(decode_utf8(src), None);
}

