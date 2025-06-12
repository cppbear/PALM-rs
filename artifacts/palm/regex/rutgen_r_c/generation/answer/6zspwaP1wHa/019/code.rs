// Answer 0

#[test]
fn test_is_ascii_word_with_digit() {
    let byte = Byte::byte(b'5');
    assert_eq!(byte.is_ascii_word(), true);
}

#[test]
fn test_is_ascii_word_with_underscore() {
    let byte = Byte::byte(b'_');
    assert_eq!(byte.is_ascii_word(), true);
}

#[test]
fn test_is_ascii_word_with_uppercase() {
    let byte = Byte::byte(b'A');
    assert_eq!(byte.is_ascii_word(), true);
}

#[test]
fn test_is_ascii_word_with_lowercase() {
    let byte = Byte::byte(b'z');
    assert_eq!(byte.is_ascii_word(), true);
}

#[test]
fn test_is_ascii_word_with_non_ascii_character() {
    let byte = Byte::byte(b'@');
    assert_eq!(byte.is_ascii_word(), false);
}

#[test]
fn test_is_ascii_word_with_eof() {
    let byte = Byte::eof();
    assert_eq!(byte.is_ascii_word(), false);
}

#[test]
fn test_is_ascii_word_with_empty() {
    let byte = Byte(0);
    assert_eq!(byte.is_ascii_word(), false);
}

