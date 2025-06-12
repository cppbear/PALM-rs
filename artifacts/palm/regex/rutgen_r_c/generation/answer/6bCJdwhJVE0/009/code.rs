// Answer 0

#[test]
fn test_escape_unicode_valid_utf8_with_no_whitespace() {
    let input: &[u8] = b"Hello World"; // Valid UTF-8 with no whitespace
    let result = escape_unicode(input);
    assert_eq!(result, "Hello World"); // Should return the same string
}

#[test]
fn test_escape_unicode_valid_utf8_with_whitespace() {
    let input: &[u8] = b"Hello \tWorld"; // Valid UTF-8 with a tab (whitespace)
    let result = escape_unicode(input);
    assert_eq!(result, "Hello \u{0009}World"); // Tab should be escaped
}

#[test]
fn test_escape_unicode_valid_utf8_with_newline() {
    let input: &[u8] = b"Hello\nWorld"; // Valid UTF-8 with a newline (whitespace)
    let result = escape_unicode(input);
    assert_eq!(result, "Hello\nWorld"); // Newline should remain unescaped
}

#[test]
fn test_escape_unicode_invalid_utf8() {
    let input: &[u8] = &[0xFF, 0xFE, 0xFD]; // Invalid UTF-8 bytes
    let result = escape_unicode(input);
    assert!(result.contains(r"\xff") || result.contains(r"\xfe") || result.contains(r"\xfd")); // Check that bytes are escaped
}

#[test]
fn test_escape_unicode_edge_case_empty_input() {
    let input: &[u8] = b""; // Empty input
    let result = escape_unicode(input);
    assert_eq!(result, ""); // Should return an empty string
}

