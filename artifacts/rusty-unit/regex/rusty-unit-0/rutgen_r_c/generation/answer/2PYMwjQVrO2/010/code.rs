// Answer 0

#[test]
fn test_escape_unicode_valid_ascii() {
    let input = b"Hello World!";
    let expected = String::from("Hello World!");
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_valid_whitespace() {
    let input = b"Hello\tWorld!";
    let expected = String::from("Hello\tWorld!");
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_valid_unicode() {
    let input = "Hello \u{2603} World!".as_bytes(); // Unicode Snowman
    let expected = String::from("Hello \u{2603} World!");
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_valid_unicode_whitespace() {
    let input = "Hello \u{00A0}World!".as_bytes(); // Non-breaking space
    let expected = String::from("Hello \u{00A0}World!");
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_no_whitespace() {
    let input = b"HelloWorld!";
    let expected = String::from("HelloWorld!");
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_boundary_unicode() {
    let input = b"\xFF"; // Invalid UTF-8
    let expected = escape_bytes(input); // Will trigger fallback to escape_bytes
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

