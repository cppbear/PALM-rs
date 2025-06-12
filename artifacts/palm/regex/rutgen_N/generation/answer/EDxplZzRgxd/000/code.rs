// Answer 0

#[test]
fn test_decode_last_utf8_empty() {
    let result = decode_last_utf8(&[]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_last_utf8_single_ascii() {
    let result = decode_last_utf8(&[0x61]); // 'a'
    assert_eq!(result, Some(('a', 1)));
}

#[test]
fn test_decode_last_utf8_valid_utf8() {
    let result = decode_last_utf8(&[0xE2, 0x9C, 0x94]); // '✓'
    assert_eq!(result, Some(('✓', 3)));
}

#[test]
fn test_decode_last_utf8_invalid_sequence() {
    let result = decode_last_utf8(&[0xE2, 0x9C, 0x94, 0x80]); // Invalid UTF-8
    assert_eq!(result, None);
}

#[test]
fn test_decode_last_utf8_partial_sequence() {
    let result = decode_last_utf8(&[0xE2, 0x9C]); // Incomplete UTF-8 sequence
    assert_eq!(result, None);
}

#[test]
fn test_decode_last_utf8_boundary_condition() {
    let result = decode_last_utf8(&[0xC2, 0xA9]); // '©'
    assert_eq!(result, Some(('©', 2)));
}

