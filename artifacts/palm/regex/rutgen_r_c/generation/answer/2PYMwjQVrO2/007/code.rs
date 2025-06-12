// Answer 0

#[test]
fn test_escape_unicode_all_whitespace() {
    let input = b" \t\n\r"; // All whitespace characters
    let expected = String::from(r"\ "); // Space should be escaped
    assert_eq!(escape_unicode(input), expected);
}

#[test]
fn test_escape_unicode_special_character() {
    let input = b"Hello, \nWorld!"; // Contains newline
    let expected = String::from("Hello, \u{000A}World!"); // Newline should be escaped
    assert_eq!(escape_unicode(input), expected);
}

#[test]
fn test_escape_unicode_boundary_character() {
    let input = [0xFF, 0xFE]; // UTF-16 encoding for characters beyond ASCII
    let expected = String::from(r"\u{00ff}\u{00fe}"); // Characters should be escaped
    assert_eq!(escape_unicode(&input), expected);
}

#[test]
fn test_escape_unicode_high_unicode() {
    let input = &[0xF0, 0x9F, 0x92, 0xA9]; // Emoji character: 💩
    let expected = String::from(r"\U{0001f4a9}"); // High unicode character should be escaped
    assert_eq!(escape_unicode(input), expected);
}

#[test]
fn test_escape_unicode_non_whitespace() {
    let input = b"abcXYZ"; // No whitespace
    let expected = String::from("abcXYZ"); // Output remains unchanged
    assert_eq!(escape_unicode(input), expected);
}

#[test]
fn test_escape_unicode_invalid_utf8() {
    let input = &[0xFF, 0xFE, 0xFD]; // Invalid UTF-8 bytes
    let expected = String::from(r"\xff\xfe\xfd"); // Should escape bytes
    assert_eq!(escape_unicode(input), expected);
}

