// Answer 0

#[test]
fn test_escape_unicode_with_ascii() {
    let input = b"Hello World";
    let result = escape_unicode(input);
    let expected = "Hello\\ World";
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_unicode() {
    let input = b"Hello\tWorld\n";
    let result = escape_unicode(input);
    let expected = "Hello\\\tWorld\\\n";
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_non_ascii() {
    let input = b"\xE2\x82\xAC"; // Euro sign € in UTF-8
    let result = escape_unicode(input);
    let expected = r"\u{20ac}";
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_whitespace() {
    let input = b" \t\r\n";
    let result = escape_unicode(input);
    let expected = r"\ \t\r\n";
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_bytes() {
    let input = b"\xFF\xFE\xFD"; // Non-UTF8 byte values
    let result = escape_unicode(input);
    let expected = r"\xff\xfe\fd";
    assert_eq!(result, expected);
}

