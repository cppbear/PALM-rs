// Answer 0

#[test]
fn test_escape_unicode_valid_ascii_whitespace() {
    let input = b"Hello World! \n";
    let output = escape_unicode(input);
    let expected = "Hello\\nWorld!";
    assert_eq!(output, expected);
}

#[test]
fn test_escape_unicode_valid_unicode_whitespace() {
    let input = "Hello, 世界! \t"; // Includes a Unicode character '世'
    let output = escape_unicode(input.as_bytes());
    // '\t' is whitespace, so it should be escaped
    let expected = "Hello, \\u{4e16}! \\t";  
    assert_eq!(output, expected);
}

#[test]
fn test_escape_unicode_valid_ascii_space_boundary() {
    let input = b"Here is a test:\r\n";
    let output = escape_unicode(input);
    let expected = "Here is a test:\\r\\n";
    assert_eq!(output, expected);
}

#[test]
fn test_escape_unicode_non_printable_whitespace() {
    let input = b"Line 1\x0BLine 2\x0C"; // Vertical tab and form feed
    let output = escape_unicode(input);
    let expected = "Line 1\\u{000b}Line 2\\u{000c}";
    assert_eq!(output, expected);
}

