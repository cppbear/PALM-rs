// Answer 0

#[test]
fn test_decode_last_utf8_empty() {
    let result = decode_last_utf8(&[]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_last_utf8_one_byte() {
    let src = &[0x7F]; // Single ASCII character, the highest one.
    let result = decode_last_utf8(src);
    assert_eq!(result, Some(('\u{7F}', 1)));
}

#[test]
fn test_decode_last_utf8_two_byte() {
    let src = &[0x80, 0xBF]; // Invalid UTF-8 sequence.
    let result = decode_last_utf8(src);
    assert_eq!(result, None);
}

#[test]
fn test_decode_last_utf8_valid_utf8() {
    let src = &[0xE2, 0x82, 0xAC]; // UTF-8 encoding of the Euro sign (€).
    let result = decode_last_utf8(src);
    assert_eq!(result, Some(('\u{20AC}', 3)));
}

#[test]
fn test_decode_last_utf8_valid_utf8_with_ascii_end() {
    let src = &[0x31, 0x32, 0x33, 0x7F]; // Ends with a valid ASCII character.
    let result = decode_last_utf8(src);
    assert_eq!(result, Some(('\u{7F}', 1)));
}

#[test]
fn test_decode_last_utf8_three_byte_sequence() {
    let src = &[0xE0, 0xA0, 0x80]; // UTF-8 encoding for character U+2000.
    let result = decode_last_utf8(src);
    assert_eq!(result, Some(('\u{2000}', 3)));
}

#[test]
fn test_decode_last_utf8_four_byte_sequence() {
    let src = &[0xF0, 0x9F, 0x98, 0x80]; // UTF-8 encoding for U+1F600.
    let result = decode_last_utf8(src);
    assert_eq!(result, Some(('\u{1F600}', 4)));
}

#[test]
fn test_decode_last_utf8_invalid_four_byte() {
    let src = &[0xF0, 0x9F, 0x80]; // Incomplete four-byte sequence.
    let result = decode_last_utf8(src);
    assert_eq!(result, None);
}

