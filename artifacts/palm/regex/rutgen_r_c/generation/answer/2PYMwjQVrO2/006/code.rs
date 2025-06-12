// Answer 0

#[test]
fn test_escape_unicode_valid_utf8_whitespace() {
    let input: &[u8] = b"Hello World"; // Valid UTF-8 with no non-whitespace characters
    let expected = "Hello World".to_string();
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_valid_utf8_whitespace_with_spaces() {
    let input: &[u8] = b"Hello \t World\n"; // Valid UTF-8 including whitespace characters (tab, newline)
    let expected = "Hello \u{0009} World\u{000A}".to_string(); // Tabs and newlines escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_valid_utf8_all_whitespace() {
    let input: &[u8] = b"   \t\n "; // Valid UTF-8 made of whitespace characters
    let expected = "\u{0020}\u{0009}\u{000A}\u{0020}".to_string(); // All whitespace characters escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_invalid_utf8() {
    let input: &[u8] = &[0xFF, 0xFE, 0xFD]; // Invalid UTF-8 bytes
    let expected = "\\xff\\xfe\\xfd".to_string(); // Bytes are escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_boundary_condition() {
    let input: &[u8] = b"~"; // One character at boundary of valid UTF-8
    let expected = "~".to_string(); // Normal output for a character outside the whitespace range
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

