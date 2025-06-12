// Answer 0

#[test]
fn test_escape_unicode_with_exactly_0xFFFF() {
    let input = vec![0xED, 0x9F, 0xBF]; // Represents U+FFFF in UTF-8
    let expected = r"\u{ffff}"; // Escaped representation for U+FFFF
    let result = escape_unicode(&input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_with_whitespace_and_non_ascii() {
    // Input with whitespace and a character in the range of U+0100 to U+FFFF
    let input = b"Hello \xA0 World \xA1";
    let expected = "Hello \\u{00a0} World \\u{00a1}"; // Whitespace and non-ASCII characters
    let result = escape_unicode(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_unicode_high_unicode() {
    // Input with a character that is a high unicode (beyond U+FFFF)
    let input = vec![0xF0, 0x9F, 0x92, 0xA9]; // Represents U+1F4A9 (💩) in UTF-8
    let expected = r"\U{0001f4a9}"; // Escaped representation for U+1F4A9
    let result = escape_unicode(&input);
    assert_eq!(result, expected);
}

