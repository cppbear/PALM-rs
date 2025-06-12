// Answer 0

#[test]
fn test_escape_unicode_with_whitespace_and_high_unicode() {
    let input = b"\n\xA0";
    let expected = r"\u{00a0}"; // Non-breaking space (0xA0) is greater than 0x7F and 0xFFFF
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_only_high_unicode() {
    let input = b"\xF0\x9F\x98\x80"; // U+1F600 (grinning face)
    let expected = r"\U{0001f600}";
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_mixed_input() {
    let input = b"Hello \n\xA3"; // Includes whitespace and high unicode (0xA3 is the pound sign)
    let expected = "Hello ".to_string() + r"\u{00a3}"; // Space remains as is, pound sign escapes
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_empty_input() {
    let input = b"";
    let expected = ""; // Empty input should return empty string
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_no_whitespace() {
    let input = b"abc\xA3def"; // Contains characters that are not whitespace
    let expected = "abc".to_string() + r"\u{00a3}" + "def"; // Pound sign escapes but others remain the same
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

