// Answer 0

#[test]
fn test_escape_unicode_non_utf8_bytes() {
    let input = &[0, 159, 146, 150]; // Invalid UTF-8 bytes
    let expected_output = r"\x00\x9f\x92\x96"; // Expect bytes to be escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_escape_unicode_non_whitespace_chars() {
    let input = b"Hello\xFFWorld"; // Contains invalid UTF-8 bytes
    let result = escape_unicode(input);
    assert_ne!(result, ""); // Ensure the result is not empty
    assert!(result.contains("Hello")); // Check if non-whitespace chars remain
    assert!(result.contains("W")); // Check if non-whitespace chars remain
}

#[test]
fn test_escape_unicode_whitespace() {
    let input = b" \n\t "; // Valid UTF-8 but all whitespace
    let expected_output = r"\u{0020}\u{000a}\u{0009}\u{0020}"; // Spaces and newlines escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_escape_unicode_mixed() {
    let input = b" Hello \xFF World "; // Mixed valid UTF-8 and invalid bytes
    let expected_output = r" Hello \u{ff} World "; // Invalid byte should be escaped
    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

