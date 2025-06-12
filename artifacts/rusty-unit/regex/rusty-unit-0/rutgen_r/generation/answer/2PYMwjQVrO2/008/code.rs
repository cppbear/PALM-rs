// Answer 0

#[test]
fn test_escape_unicode_with_non_whitespace_over_0xFFFF() {
    let input: &[u8] = &[0xE2, 0x82, 0xAC]; // Euro sign (U+20AC)
    let expected = r"\u{20ac}"; // Expect Unicode escape for U+20AC
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_non_whitespace_over_0x7F() {
    let input: &[u8] = &[0xC2, 0xA9]; // Copyright sign (U+00A9)
    let expected = r"\u{00a9}"; // Expect Unicode escape for U+00A9
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_non_whitespace_ascii() {
    let input: &[u8] = b"Hello"; // Regular ASCII
    let expected = "Hello"; // No escape needed for ASCII
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_whitespace() {
    let input: &[u8] = b" \t\n"; // Whitespace characters
    let expected = r"\u{0020}\u{0009}\u{000a}"; // Expect Unicode escapes for whitespaces
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_combined_characters() {
    let input: &[u8] = &[0xF0, 0x9F, 0x98, 0x80]; // Grinning face (U+1F600)
    let expected = r"\U{0001f600}"; // Expect Unicode escape for U+1F600
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

