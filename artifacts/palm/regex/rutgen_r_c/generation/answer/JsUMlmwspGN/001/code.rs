// Answer 0

#[test]
fn test_decode_utf8_ascii_value() {
    let input: &[u8] = &[0x7F];
    let result = decode_utf8(input);
    assert_eq!(result, Some(('\u{7F}', 1)));
}

#[test]
fn test_decode_utf8_valid_1_byte() {
    let input: &[u8] = &[0x41]; // 'A'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('A', 1)));
}

#[test]
fn test_decode_utf8_valid_2_bytes() {
    let input: &[u8] = &[0xC2, 0xA0]; // ' ' (non-breaking space)
    let result = decode_utf8(input);
    assert_eq!(result, Some(('\u{00A0}', 2)));
}

#[test]
fn test_decode_utf8_valid_3_bytes() {
    let input: &[u8] = &[0xE2, 0x9C, 0x94]; // '✔'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('\u{2714}', 3)));
}

#[test]
fn test_decode_utf8_valid_4_bytes() {
    let input: &[u8] = &[0xF0, 0x9F, 0x98, 0x80]; // '😀'
    let result = decode_utf8(input);
    assert_eq!(result, Some(('\u{1F600}', 4)));
}

#[test]
fn test_decode_utf8_invalid_too_short() {
    let input: &[u8] = &[0xC2]; // expecting at least 2 bytes for 2-byte character
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_first_byte() {
    let input: &[u8] = &[0xFF]; // Invalid first byte for any UTF-8 character
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_second_byte() {
    let input: &[u8] = &[0xC2, 0xFF]; // Valid first byte but invalid second byte
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_out_of_range() {
    let input: &[u8] = &[0xF0, 0x9F, 0xBF, 0xBF]; // out of Unicode range
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_surrogate_codepoint() {
    let input: &[u8] = &[0xED, 0x9E, 0xA0]; // surrogate codepoint
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

