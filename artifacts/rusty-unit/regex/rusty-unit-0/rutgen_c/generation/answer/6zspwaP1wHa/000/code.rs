// Answer 0

#[test]
fn test_is_ascii_word_with_uppercase() {
    let byte = Byte::byte(b'A');
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_lowercase() {
    let byte = Byte::byte(b'a');
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_digit() {
    let byte = Byte::byte(b'0');
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_underscore() {
    let byte = Byte::byte(b'_');
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_non_word_character() {
    let byte = Byte::byte(b'!');
    assert!(!byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_eof() {
    let byte = Byte::eof();
    assert!(!byte.is_ascii_word());
}

