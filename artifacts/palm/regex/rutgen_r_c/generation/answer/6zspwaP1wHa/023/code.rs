// Answer 0

#[test]
fn test_is_ascii_word_uppercase() {
    let byte = Byte::byte(b'A');
    assert_eq!(byte.is_ascii_word(), true);
}

#[test]
fn test_is_ascii_word_lowercase() {
    let byte = Byte::byte(b'a');
    assert_eq!(byte.is_ascii_word(), true);
}

#[test]
fn test_is_ascii_word_digit() {
    let byte = Byte::byte(b'0');
    assert_eq!(byte.is_ascii_word(), true);
}

#[test]
fn test_is_ascii_word_underscore() {
    let byte = Byte::byte(b'_');
    assert_eq!(byte.is_ascii_word(), true);
}

#[test]
fn test_is_ascii_word_not_ascii_word() {
    let byte = Byte::byte(b'!');
    assert_eq!(byte.is_ascii_word(), false);
}

#[test]
fn test_is_ascii_word_eof() {
    let byte = Byte::eof();
    assert_eq!(byte.is_ascii_word(), false);
}

