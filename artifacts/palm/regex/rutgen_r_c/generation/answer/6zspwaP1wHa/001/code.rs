// Answer 0

#[test]
fn test_is_ascii_word_uppercase() {
    let byte = Byte::byte(b'A');
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_lowercase() {
    let byte = Byte::byte(b'z');
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_digit() {
    let byte = Byte::byte(b'5');
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_underscore() {
    let byte = Byte::byte(b'_');
    assert!(byte.is_ascii_word());
}

