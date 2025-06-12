// Answer 0

#[test]
fn test_escape_unicode_valid_ascii() {
    let input = b"Hello World";
    let expected = "Hello World";
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_whitespace_ascii() {
    let input = b"Hello\tWorld\n";
    let expected = "Hello\\tWorld\\n";
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_valid_unicode() {
    let input = "こんにちは".as_bytes();
    let expected = "こんにちは";
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_whitespace_unicode() {
    let input = "こんにちは 世界".as_bytes();
    let expected = "こんにちは \\u{4e16}\\u{754c}";
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_invalid_utf8() {
    let input = &[0xFF, 0xFE, 0xFD]; // Invalid UTF-8 sequence
    let expected = "\\xff\\xfe\\xfd";
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_large_unicode() {
    let input = "𝄞".as_bytes(); // U+1D11E (Musical Symbol G Clef)
    let expected = r"\U{1d11e}";
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

