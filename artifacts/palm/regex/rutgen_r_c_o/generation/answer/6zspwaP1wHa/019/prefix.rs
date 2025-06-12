// Answer 0

#[test]
fn test_is_ascii_word_digit_zero() {
    let byte = Byte::byte(0);
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_digit_nine() {
    let byte = Byte::byte(9);
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_underscore() {
    let byte = Byte::byte(95);
    byte.is_ascii_word();
}

