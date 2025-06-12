// Answer 0

#[test]
fn test_escape_bytes_empty() {
    let bytes: &[u8] = &[];
    let result = escape_bytes(bytes);
    assert_eq!(result, "");
}

#[test]
fn test_escape_bytes_single_byte() {
    let bytes: &[u8] = &[0];
    let result = escape_bytes(bytes);
    assert_eq!(result, "\\0");
}

#[test]
fn test_escape_bytes_multiple_bytes() {
    let bytes: &[u8] = &[1, 2, 3];
    let result = escape_bytes(bytes);
    assert_eq!(result, "\\x01\\x02\\x03");
}

#[test]
fn test_escape_bytes_control_characters() {
    let bytes: &[u8] = &[7, 8, 9, 10, 11, 12, 13];
    let result = escape_bytes(bytes);
    assert_eq!(result, "\\x07\\x08\\t\\n\\x0b\\x0c\\r");
}

#[test]
fn test_escape_bytes_high_byte() {
    let bytes: &[u8] = &[255, 128, 64];
    let result = escape_bytes(bytes);
    assert_eq!(result, "\\xff\\x80@");
}

#[test]
fn test_escape_bytes_non_ascii() {
    let bytes: &[u8] = &[230, 136, 152]; // Represents UTF-8 bytes for a non-ASCII character
    let result = escape_bytes(bytes);
    assert_eq!(result, "\\xe6\\x88\\x98");
}

