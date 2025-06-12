// Answer 0

#[test]
fn test_escape_unicode_with_valid_utf8() {
    let input = b"Hello World";
    let expected_output = "Hello World";
    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_escape_unicode_with_whitespace() {
    let input = b"Hello\tWorld\n";
    let expected_output = "Hello\tWorld\n";  // Assuming tab and newline are preserved
    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_escape_unicode_with_non_ascii() {
    let input = b"Hello \xE2\x9C\x94"; // Unicode checkmark character "✓"
    let expected_output = "Hello \u{2714}"; // Should be escaped as Unicode
    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_escape_unicode_with_bytes_that_are_not_utf8() {
    let input = b"\xFF\xFE\xFD"; // Invalid UTF-8 bytes
    let expected_output = "\u{ff}\u{fe}\u{fd}"; // Expected to escape bytes to Unicode
    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_escape_unicode_with_large_unicode() {
    let input = b"\xF0\x9F\x92\xA9"; // Unicode for pile of poop "💩"
    let expected_output = r"\U{1f4a9}";
    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

