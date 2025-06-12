// Answer 0

fn is_ascii_word(b: u8) -> bool {
    match b {
        b'A'...b'Z' | b'a'...b'z' | b'0'...b'9' | b'_' => true,
        _ => false,
    }
}

#[test]
fn test_is_ascii_word_with_digit_zero() {
    assert_eq!(is_ascii_word(b'0'), true);
}

#[test]
fn test_is_ascii_word_with_digit_nine() {
    assert_eq!(is_ascii_word(b'9'), true);
}

#[test]
fn test_is_ascii_word_with_lowercase_a() {
    assert_eq!(is_ascii_word(b'a'), true);
}

#[test]
fn test_is_ascii_word_with_lowercase_z() {
    assert_eq!(is_ascii_word(b'z'), true);
}

#[test]
fn test_is_ascii_word_with_uppercase_a() {
    assert_eq!(is_ascii_word(b'A'), true);
}

#[test]
fn test_is_ascii_word_with_uppercase_z() {
    assert_eq!(is_ascii_word(b'Z'), true);
}

#[test]
fn test_is_ascii_word_with_underscore() {
    assert_eq!(is_ascii_word(b'_'), true);
}

#[test]
fn test_is_ascii_word_with_non_ascii_character() {
    assert_eq!(is_ascii_word(b'@'), false);
}

