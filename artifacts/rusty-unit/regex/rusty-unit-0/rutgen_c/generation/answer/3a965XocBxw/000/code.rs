// Answer 0

#[test]
fn test_char_len_lossy_empty() {
    let bytes: &[u8] = &[];
    let result = char_len_lossy(bytes);
    assert_eq!(result, 0);
}

#[test]
fn test_char_len_lossy_valid_utf8() {
    let bytes: &[u8] = b"Hello, World!";
    let result = char_len_lossy(bytes);
    assert_eq!(result, 13);
}

#[test]
fn test_char_len_lossy_invalid_utf8() {
    let bytes: &[u8] = &[0xff, 0xfe, 0xfd]; 
    let result = char_len_lossy(bytes);
    assert_eq!(result, 3); // Assuming lossless replacement counts as character length
}

#[test]
fn test_char_len_lossy_mixed() {
    let bytes: &[u8] = b"Hello \xff World!";
    let result = char_len_lossy(bytes);
    assert_eq!(result, 13); // The invalid byte is replaced
}

#[test]
fn test_char_len_lossy_surrogate() {
    let bytes: &[u8] = &[0xed, 0xa0, 0xbd]; // UTF-8 encoding of U+D7FF
    let result = char_len_lossy(bytes);
    assert_eq!(result, 1); // Surrogate code point results in a replacement character
}

