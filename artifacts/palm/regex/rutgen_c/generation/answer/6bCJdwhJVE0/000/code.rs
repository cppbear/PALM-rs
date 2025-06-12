// Answer 0

#[test]
fn test_escape_unicode_ascii() {
    let input: &[u8] = b"Hello World";
    let result = escape_unicode(input);
    assert_eq!(result, "Hello World");
}

#[test]
fn test_escape_unicode_utf16() {
    let input: &[u8] = &[0xE2, 0x9C, 0x94]; // CHECK MARK (U+2714)
    let result = escape_unicode(input);
    assert_eq!(result, "\u{2714}");
}

#[test]
fn test_escape_unicode_unicode_whitespace() {
    let input: &[u8] = b"Hello\nWorld";
    let result = escape_unicode(input);
    assert_eq!(result, "Hello\nWorld");
}

#[test]
fn test_escape_unicode_byte_sequence() {
    let input: &[u8] = &[0xFF, 0xFE, 0xFD];
    let result = escape_unicode(input);
    assert_eq!(result, "\u{ff}\u{fe}\u{fd}");
}

#[test]
fn test_escape_unicode_invalid_utf8() {
    let input: &[u8] = &[0xFF, 0xFE, 0xFD, 0xED];
    let result = escape_unicode(input);
    assert_eq!(result, "\u{ff}\u{fe}\u{fd}\u{ed}");
}

#[test]
fn test_escape_unicode_large_unicode() {
    let input: &[u8] = &[0xF0, 0x9F, 0x98, 0xA9]; // GRINNING FACE WITH SMILING EYES (U+1F600)
    let result = escape_unicode(input);
    assert_eq!(result, r"\U{0001f600}");
}

