// Answer 0

#[test]
fn test_decode_utf8_ascii() {
    let input = [0x41]; // 'A'
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('A', 1)));
}

#[test]
fn test_decode_utf8_two_bytes() {
    let input = [0xC2, 0xA9]; // '©'
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('©', 2)));
}

#[test]
fn test_decode_utf8_three_bytes() {
    let input = [0xE2, 0x82, 0xAC]; // '€'
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('€', 3)));
}

#[test]
fn test_decode_utf8_four_bytes() {
    let input = [0xF0, 0x9F, 0x98, 0x81]; // '😀'
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('😀', 4)));
}

#[test]
fn test_decode_utf8_invalid_too_short() {
    let input = [0xC2]; // Incomplete 2-byte sequence
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_tag() {
    let input = [0xC2, 0x20]; // Incorrect continuation byte
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_surrogate() {
    let input = [0xED, 0x9F, 0xBF]; // Invalid surrogate
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_out_of_range() {
    let input = [0xF4, 0x90, 0x80, 0x80]; // Out of range (0x110000)
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

