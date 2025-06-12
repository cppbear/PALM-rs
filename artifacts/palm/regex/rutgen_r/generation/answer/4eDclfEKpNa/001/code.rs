// Answer 0

#[test]
fn test_is_word_byte_true_underscore() {
    assert_eq!(is_word_byte(b'_'), true);
}

#[test]
fn test_is_word_byte_true_digit() {
    assert_eq!(is_word_byte(b'0'), true);
    assert_eq!(is_word_byte(b'5'), true);
    assert_eq!(is_word_byte(b'9'), true);
}

#[test]
fn test_is_word_byte_true_lowercase() {
    assert_eq!(is_word_byte(b'a'), true);
    assert_eq!(is_word_byte(b'm'), true);
    assert_eq!(is_word_byte(b'z'), true);
}

#[test]
fn test_is_word_byte_true_uppercase() {
    assert_eq!(is_word_byte(b'A'), true);
    assert_eq!(is_word_byte(b'M'), true);
    assert_eq!(is_word_byte(b'Z'), true);
}

