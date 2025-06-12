// Answer 0

#[test]
fn test_escape_bytes_empty() {
    let bytes: &[u8] = &[];
    let result = escape_bytes(bytes);
    assert_eq!(result, "");
}

#[test]
fn test_escape_bytes_single_byte() {
    let bytes: &[u8] = &[0u8]; // testing with a null byte
    let result = escape_bytes(bytes);
    assert_eq!(result, "\\0");
}

#[test]
fn test_escape_bytes_multiple_bytes() {
    let bytes: &[u8] = &[65, 66, 67]; // ASCII for 'A', 'B', 'C'
    let result = escape_bytes(bytes);
    assert_eq!(result, "A\\x42B\\x43");
}

#[test]
fn test_escape_bytes_special_characters() {
    let bytes: &[u8] = &[9, 10, 13, 32, 33]; // tab, newline, carriage return, space, exclamation
    let result = escape_bytes(bytes);
    assert_eq!(result, "\\t\\n\\r A!");
}

#[test]
fn test_escape_bytes_non_ascii() {
    let bytes: &[u8] = &[128, 255]; // testing with non-ASCII values
    let result = escape_bytes(bytes);
    assert_eq!(result, "\\x80\\xff");
}

