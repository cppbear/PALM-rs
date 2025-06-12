// Answer 0

#[test]
fn test_decode_utf8_two_byte_sequence() {
    let src = &[0b110_00010, 0b10000010]; // Valid 2-byte UTF-8 sequence for '¢' (U+00A2)
    let result = decode_utf8(src);
    assert_eq!(result, Some(('¢', 2)));
}

#[test]
fn test_decode_utf8_three_byte_sequence() {
    let src = &[0b1110_0010, 0b10000010, 0b10101100]; // Valid 3-byte UTF-8 sequence for '¢' (U+20AC)
    let result = decode_utf8(src);
    assert_eq!(result, Some(('€', 3)));
}

#[test]
fn test_decode_utf8_four_byte_sequence() {
    let src = &[0b11110_000, 0b10000001, 0b10000010, 0b10000000]; // Valid 4-byte UTF-8 sequence for '𐀀' (U+10000)
    let result = decode_utf8(src);
    assert_eq!(result, Some(('𐀀', 4)));
}

#[test]
fn test_decode_utf8_invalid_two_byte_sequence() {
    let src = &[0b110_00010, 0b10111000]; // Invalid continuation byte
    let result = decode_utf8(src);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_too_short_two_byte_sequence() {
    let src = &[0b110_00010]; // Only one byte provided
    let result = decode_utf8(src);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_non_utf8_sequence() {
    let src = &[0b1111_1111, 0b1111_1111]; // Invalid sequence
    let result = decode_utf8(src);
    assert_eq!(result, None);
}

