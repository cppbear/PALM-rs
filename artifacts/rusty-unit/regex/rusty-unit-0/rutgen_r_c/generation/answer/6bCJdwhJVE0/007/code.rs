// Answer 0

#[test]
fn test_escape_unicode_empty_input() {
    let input: &[u8] = b"";
    let expected = "";
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_whitespace_within_ascii_bounds() {
    let input: &[u8] = b"Hello World"; // ASCII spaces
    let expected = "Hello World"; // No escaping occurs
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_whitespace_within_unicode_bounds() {
    let input: &[u8] = "Hello \u{00A0}World".as_bytes(); // Non-breaking space (U+00A0)
    let expected = format!("Hello \\u{{00a0}}World");
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_whitespace_at_upper_unicode_bound() {
    let input: &[u8] = "Hello \u{FFFF}World".as_bytes(); // Upper bound character (U+FFFF)
    let expected = format!("Hello \\u{{ffff}}World");
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_non_whitespace_chars() {
    let input: &[u8] = "HelloWorld".as_bytes(); // No whitespace characters
    let expected = "HelloWorld"; // No escaping occurs
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

