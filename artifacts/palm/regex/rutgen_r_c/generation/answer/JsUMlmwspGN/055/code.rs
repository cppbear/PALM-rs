// Answer 0

#[test]
fn test_decode_utf8_empty_input() {
    let input: &[u8] = &[];
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_single_byte() {
    let input: &[u8] = &[0x7F];
    let result = decode_utf8(input);
    assert_eq!(result, Some(('+', 1)));
}

#[test]
fn test_decode_utf8_too_short_for_two_bytes() {
    let input: &[u8] = &[0xC2]; // Only one byte provided, expects None
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_two_bytes() {
    let input: &[u8] = &[0xC2, 0x25]; // Second byte is invalid, expects None
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_two_bytes() {
    let input: &[u8] = &[0xC2, 0xA2]; // Valid encoding for '¢'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('¢', 2)));
}

#[test]
fn test_decode_utf8_too_short_for_three_bytes() {
    let input: &[u8] = &[0xE2, 0x82]; // Only two bytes provided, expects None
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_three_bytes() {
    let input: &[u8] = &[0xE2, 0x00, 0xA2]; // Second byte is invalid, expects None
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_three_bytes() {
    let input: &[u8] = &[0xE2, 0x82, 0xAC]; // Valid encoding for '€'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('€', 3)));
}

#[test]
fn test_decode_utf8_too_short_for_four_bytes() {
    let input: &[u8] = &[0xF0, 0x9F, 0x98]; // Only three bytes provided, expects None
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_four_bytes() {
    let input: &[u8] = &[0xF0, 0x9F, 0x98, 0x00]; // Last byte should be valid, expects None
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_four_bytes() {
    let input: &[u8] = &[0xF0, 0x9F, 0x98, 0x81]; // Valid encoding for '😂'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('😂', 4)));
}

