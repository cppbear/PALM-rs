// Answer 0

#[test]
fn test_decode_last_utf8_empty_input() {
    let src: &[u8] = &[];
    let result = decode_last_utf8(src);
    assert_eq!(result, None);
}

#[test]
fn test_decode_last_utf8_single_byte() {
    let src: &[u8] = &[0x61]; // 'a'
    let result = decode_last_utf8(src);
    assert_eq!(result, Some(('a', 1)));
}

#[test]
fn test_decode_last_utf8_two_bytes() {
    let src: &[u8] = &[0xC2, 0xA9]; // '©'
    let result = decode_last_utf8(src);
    assert_eq!(result, Some(('©', 2)));
}

#[test]
fn test_decode_last_utf8_three_bytes() {
    let src: &[u8] = &[0xE2, 0x9C, 0x94]; // '✓'
    let result = decode_last_utf8(src);
    assert_eq!(result, Some(('✓', 3)));
}

#[test]
fn test_decode_last_utf8_four_bytes() {
    let src: &[u8] = &[0xF0, 0x9F, 0x98, 0x81]; // '😀'
    let result = decode_last_utf8(src);
    assert_eq!(result, Some(('😀', 4)));
}

#[test]
fn test_decode_last_utf8_incomplete_sequence() {
    let src: &[u8] = &[0xE2, 0x9C]; // Incomplete sequence for '✓'
    let result = decode_last_utf8(src);
    assert_eq!(result, None);
}

#[test]
fn test_decode_last_utf8_invalid_sequence() {
    let src: &[u8] = &[0xC3, 0x28]; // Invalid UTF-8 sequence
    let result = decode_last_utf8(src);
    assert_eq!(result, None);
}

