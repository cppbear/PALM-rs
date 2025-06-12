// Answer 0

#[test]
fn test_escape_unicode_valid_ascii() {
    let bytes: &[u8] = b"Hello World";
    let escaped = escape_unicode(bytes);
    assert_eq!(escaped, "Hello World");
}

#[test]
fn test_escape_unicode_valid_utf8_with_whitespace() {
    let bytes: &[u8] = b"Hello \t World\n";
    let escaped = escape_unicode(bytes);
    assert_eq!(escaped, "Hello \t World\n");
}

#[test]
fn test_escape_unicode_valid_utf8_with_non_ascii() {
    // Testing with non-ASCII characters
    let bytes: &[u8] = "Привет".as_bytes();
    let escaped = escape_unicode(bytes);
    assert_eq!(escaped, "Привет"); // Non-whitespace characters remain unchanged
}

#[test]
fn test_escape_unicode_with_whitespace_nonascii() {
    // Testing with non-ASCII whitespace (U+3000 is a full-width space)
    let bytes: &[u8] = &[0xE3, 0x80, 0x80]; // Full-width space
    let escaped = escape_unicode(bytes);
    assert_eq!(escaped, r"\u{3000}"); // It's a whitespace, so it should be escaped
}

#[test]
fn test_escape_unicode_long_unicodes() {
    // Testing with a character that exceeds 0xFFFF
    let bytes: &[u8] = "𠀀".as_bytes(); // U+20000
    let escaped = escape_unicode(bytes);
    assert_eq!(escaped, r"\U{00020000}"); // Should be escaped as long unicode
}

#[test]
fn test_escape_unicode_combined_characters() {
    let bytes: &[u8] = "A \u{3000}".as_bytes(); // "A" followed by a full-width space
    let escaped = escape_unicode(bytes);
    assert_eq!(escaped, "A \u{3000}"); // Non-whitespace characters remain unchanged
}

#[test]
fn test_escape_unicode_multiple_whitespaces() {
    let bytes: &[u8] = b"Line1\n\nLine2\t   ";
    let escaped = escape_unicode(bytes);
    assert_eq!(escaped, "Line1\n\nLine2\t   "); // Only whitespace characters should be preserved
}

