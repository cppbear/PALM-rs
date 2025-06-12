// Answer 0

#[test]
fn test_char_len_lossy_empty() {
    let bytes: &[u8] = b"";
    let result = char_len_lossy(bytes);
    assert_eq!(result, 0);
}

#[test]
fn test_char_len_lossy_valid_utf8() {
    let bytes: &[u8] = b"hello";
    let result = char_len_lossy(bytes);
    assert_eq!(result, 5);
}

#[test]
fn test_char_len_lossy_invalid_utf8() {
    let bytes: &[u8] = &[0xFF, 0xFE, 0xFD];
    let result = char_len_lossy(bytes);
    assert!(result > 0); // Valid counts should still return a positive count due to lossiness
}

#[test]
fn test_char_len_lossy_mixed_content() {
    let bytes: &[u8] = b"hello \xF0\x9F\x98\x80world"; // valid string with a valid UTF-8 emoji
    let result = char_len_lossy(bytes);
    assert_eq!(result, 12); // 5 + 1 + 5 = 11 (5 + 1 for the emoji and 'world')
}

#[test]
fn test_char_len_lossy_single_invalid_byte() {
    let bytes: &[u8] = &[0x80]; // lone invalid byte
    let result = char_len_lossy(bytes);
    assert_eq!(result, 1); // the invalid byte counts as one character
}

#[test]
fn test_char_len_lossy_continuous_invalid_bytes() {
    let bytes: &[u8] = &[0x80, 0x81, 0x82]; // multiple invalid bytes
    let result = char_len_lossy(bytes);
    assert_eq!(result, 3); // each invalid byte is counted as one character
}

