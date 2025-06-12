// Answer 0

#[test]
fn test_escape_unicode_valid_ascii() {
    let input = b"Hello World";
    let result = escape_unicode(input);
    assert_eq!(result, "Hello World");
}

#[test]
fn test_escape_unicode_valid_unicode() {
    let input = b"Hello \xE2\x9C\x94"; // "Hello ✔"
    let result = escape_unicode(input);
    assert_eq!(result, "Hello \u{2714}");
}

#[test]
fn test_escape_unicode_invalid_utf8() {
    let input = b"Hello \xFF World"; // Invalid UTF-8 byte
    let result = escape_unicode(input);
    assert!(result.contains("\\x")); // Expecting some byte escape sequence
}

#[test]
fn test_escape_unicode_whitespace() {
    let input = b"Hello\tWorld\n";
    let result = escape_unicode(input);
    assert_eq!(result, "Hello\\tWorld\\n");
}

#[test]
fn test_escape_unicode_non_ascii() {
    let input = b"\xF0\x9F\x98\x81"; // Grinning Face Emoji
    let result = escape_unicode(input);
    assert_eq!(result, r"\U{1f601}");
}

#[test]
fn test_escape_unicode_mix() {
    let input = b"Hello \xE2\x9C\x94 \xE2\x80\x8B World"; // "Hello ✔ [Zero Width Space] World"
    let result = escape_unicode(input);
    assert_eq!(result, "Hello \u{2714} \\u{200B} World"); // Expect zero width space to be escaped
}

