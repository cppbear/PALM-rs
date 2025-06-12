// Answer 0

#[test]
fn test_decode_last_utf8_non_empty_ascii() {
    let input: &[u8] = &[0x00, 0x01, 0x7F]; // valid non-empty input with last byte <= 0x7F
    let result = decode_last_utf8(input);
    assert_eq!(result, Some((0x7F as char, 1))); // asserts character is 0x7F and length is 1
}

#[test]
fn test_decode_last_utf8_non_empty_single_byte() {
    let input: &[u8] = &[0x7F]; // valid single byte input
    let result = decode_last_utf8(input);
    assert_eq!(result, Some((0x7F as char, 1))); // asserts character is 0x7F and length is 1
}

#[test]
fn test_decode_last_utf8_non_empty_mixed() {
    let input: &[u8] = &[0xC2, 0xA9, 0x7F]; // valid mixed input with last byte <= 0x7F
    let result = decode_last_utf8(input);
    assert_eq!(result, Some((0x7F as char, 1))); // asserts character is 0x7F and length is 1
}

