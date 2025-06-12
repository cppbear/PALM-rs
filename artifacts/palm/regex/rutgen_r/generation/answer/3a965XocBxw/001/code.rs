// Answer 0

#[test]
fn test_char_len_lossy_valid_utf8() {
    let valid_bytes: &[u8] = b"Hello, world!";
    let length = char_len_lossy(valid_bytes);
    assert_eq!(length, 13);
}

#[test]
fn test_char_len_lossy_empty() {
    let empty_bytes: &[u8] = b"";
    let length = char_len_lossy(empty_bytes);
    assert_eq!(length, 0);
}

#[test]
fn test_char_len_lossy_non_utf8_bytes() {
    let non_utf8_bytes: &[u8] = &[0xff, 0xfe, 0xfd];
    let length = char_len_lossy(non_utf8_bytes);
    assert!(length > 0); // Expecting some replacement characters to be present
}

#[test]
fn test_char_len_lossy_combined_valid_and_invalid() {
    let combined_bytes: &[u8] = b"Hello, \xFFworld!";
    let length = char_len_lossy(combined_bytes);
    assert_eq!(length, 14); // '�' for the invalid byte
}

#[test]
fn test_char_len_lossy_only_invalid_bytes() {
    let only_invalid_bytes: &[u8] = &[0x80, 0xFF, 0xFE];
    let length = char_len_lossy(only_invalid_bytes);
    assert!(length > 0); // Expecting some replacement characters to be present
}

#[test]
fn test_char_len_lossy_boundary_condition() {
    let boundary_bytes: &[u8] = b"\xF0\x9F\x92\xA9"; // Valid UTF-8 for 💩
    let length = char_len_lossy(boundary_bytes);
    assert_eq!(length, 2); // 2 characters in the string
}

