// Answer 0

#[test]
fn test_escape_bytes_empty() {
    let bytes: &[u8] = &[];
    let result = escape_bytes(bytes);
    assert_eq!(result, "");
}

#[test]
fn test_escape_bytes_single_byte_printable() {
    let bytes: &[u8] = &[97]; // 'a'
    let result = escape_bytes(bytes);
    assert_eq!(result, "a");
}

#[test]
fn test_escape_bytes_single_byte_non_printable() {
    let bytes: &[u8] = &[1]; // non-printable byte
    let result = escape_bytes(bytes);
    assert_eq!(result, "\u{1}");
}

#[test]
fn test_escape_bytes_multiple_bytes() {
    let bytes: &[u8] = &[65, 66, 67]; // 'A', 'B', 'C'
    let result = escape_bytes(bytes);
    assert_eq!(result, "ABC");
}

#[test]
fn test_escape_bytes_bytes_with_non_printable() {
    let bytes: &[u8] = &[10, 20]; // newline and non-printable byte
    let result = escape_bytes(bytes);
    assert_eq!(result, "\n\u{14}");
}

#[test]
fn test_escape_bytes_bytes_with_special_characters() {
    let bytes: &[u8] = &[0, 255]; // null and maximum byte value
    let result = escape_bytes(bytes);
    assert_eq!(result, "\u{0}\u{ff}");
}

#[test]
fn test_escape_bytes_large_input() {
    let bytes: Vec<u8> = (0..=255).collect(); // All byte values from 0 to 255
    let result = escape_bytes(&bytes);
    let expected: String = bytes.iter().map(|&b| escape_byte(b)).collect();
    assert_eq!(result, expected);
}

