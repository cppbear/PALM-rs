// Answer 0

#[test]
fn test_escape_unicode_valid_ascii_whitespace() {
    let input = b"Hello World\n"; // Contains whitespace and valid UTF-8
    let expected = "Hello\\n"; // Expecting newline to be escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_valid_ascii_space() {
    let input = b"Hello World "; // Contains space and valid UTF-8
    let expected = "Hello\\ World"; // Expecting space to be escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_valid_utf16_whitespace() {
    let input = "Hello \u{00A0}World".as_bytes(); // Non-breaking space (U+00A0) is from valid UTF-8
    let expected = "Hello \\u{00a0}World"; // Non-breaking space should be Unicode escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_valid_utf32_whitespace() {
    let input = "Hello \u{2003}World".as_bytes(); // Em Space (U+2003) is from valid UTF-8
    let expected = "Hello \\u{2003}World"; // Em space should be Unicode escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected);
} 

#[test]
fn test_escape_unicode_invalid_utf8() {
    let input = &[0xff, 0xfe, 0xfd]; // Invalid UTF-8 bytes
    let expected = "\\xff\\xfe\\xfd"; // All bytes should be escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

