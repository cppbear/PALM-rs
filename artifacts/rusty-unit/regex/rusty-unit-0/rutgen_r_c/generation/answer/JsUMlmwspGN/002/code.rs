// Answer 0

#[test]
fn test_decode_utf8_valid_ascii() {
    let input = &[0x61];  // 'a'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('a', 1)));
}

#[test]
fn test_decode_utf8_valid_two_byte_sequence() {
    let input = &[0xC2, 0xA1];  // '\u00A1' (inverted exclamation mark)
    let result = decode_utf8(input);
    assert_eq!(result, Some(('\u{00A1}', 2)));
}

#[test]
fn test_decode_utf8_valid_three_byte_sequence() {
    let input = &[0xE2, 0x82, 0xAC];  // '\u20AC' (Euro sign)
    let result = decode_utf8(input);
    assert_eq!(result, Some(('\u{20AC}', 3)));
}

#[test]
fn test_decode_utf8_valid_four_byte_sequence() {
    let input = &[0xF0, 0x9F, 0x92, 0xA9];  // '\u{1F12A}' (banknote)
    let result = decode_utf8(input);
    assert_eq!(result, Some(('\u{1F12A}', 4)));
}

#[test]
fn test_decode_utf8_invalid_too_short() {
    let input = &[0xC2];  // Expected to be followed by another byte for a valid two-byte sequence
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_not_continuation_byte() {
    let input = &[0xC2, 0x20];  // Second byte '0x20' is not a continuation byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_fourth_byte_not_continuation() {
    let input = &[0xF0, 0x9F, 0x92, 0x20];  // Last byte '0x20' is not a continuation byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_out_of_range() {
    let input = &[0xF0, 0x9F, 0xBF, 0xBF];  // Invalid range, should return None
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

