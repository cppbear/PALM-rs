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
    let bytes: &[u8] = &[0, 159, 146, 150]; // Invalid UTF-8 sequence
    let result = char_len_lossy(bytes);
    assert_eq!(result, 1); // Counts as one replacement character
}

#[test]
fn test_char_len_lossy_mixed() {
    let bytes: &[u8] = b"hello\xFFworld"; // 'hello' is valid, '\xFF' is invalid
    let result = char_len_lossy(bytes);
    assert_eq!(result, 11); // Counts valid characters and the replacement character for '\xFF'
}

#[test]
fn test_char_len_lossy_multibyte_char() {
    let bytes: &[u8] = "测试".as_bytes(); // Chinese characters (valid UTF-8)
    let result = char_len_lossy(bytes);
    assert_eq!(result, 2); // "测试" has 2 characters
}

