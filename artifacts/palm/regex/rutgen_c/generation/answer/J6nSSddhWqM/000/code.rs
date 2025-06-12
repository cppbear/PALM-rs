// Answer 0

#[test]
fn test_len_utf8_valid_char() {
    let valid_char = Char(97); // ASCII 'a'
    assert_eq!(valid_char.len_utf8(), 1);
}

#[test]
fn test_len_utf8_non_character() {
    let non_character = Char(0xD800); // High surrogate
    assert_eq!(non_character.len_utf8(), 0);
}

#[test]
fn test_len_utf8_invalid_unicode() {
    let invalid_unicode = Char(0x110000); // Code point out of range
    assert_eq!(invalid_unicode.len_utf8(), 0);
}

#[test]
fn test_len_utf8_boundary_char() {
    let boundary_char = Char(0x10FFFF); // Maximum valid Unicode code point
    assert_eq!(boundary_char.len_utf8(), 4);
}

