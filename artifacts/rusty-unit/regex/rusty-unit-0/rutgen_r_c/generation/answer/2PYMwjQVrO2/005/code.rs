// Answer 0

#[test]
fn test_escape_unicode_invalid_utf8() {
    let input: &[u8] = &[0xFF, 0xFE, 0xFD]; // Invalid UTF-8 sequence
    let expected_output = "\\xff\\xfe\\xfd"; // Expectation for escape_bytes

    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_escape_unicode_no_whitespace() {
    let input: &[u8] = b"hello"; // Valid UTF-8 with no whitespace
    let expected_output = "hello"; // No spaces to escape, should be same as input

    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

#[test]
fn test_escape_unicode_with_large_unicode() {
    let input: &[u8] = &[0xF0, 0x9F, 0x8C, 0x80]; // Valid UTF-8 for 🌈 (Rainbow)
    let expected_output = "🌈"; // No spaces, output should remain as is

    let result = escape_unicode(input);
    assert_eq!(result, expected_output);
}

