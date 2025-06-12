// Answer 0

#[test]
fn test_is_word_byte_underscore() {
    let result = is_word_byte(b'_');
    assert_eq!(result, true);
}

#[test]
fn test_is_word_byte_digit_0() {
    let result = is_word_byte(b'0');
    assert_eq!(result, true);
}

#[test]
fn test_is_word_byte_digit_9() {
    let result = is_word_byte(b'9');
    assert_eq!(result, true);
}

#[test]
fn test_is_word_byte_lowercase_a() {
    let result = is_word_byte(b'a');
    assert_eq!(result, true);
}

#[test]
fn test_is_word_byte_lowercase_z() {
    let result = is_word_byte(b'z');
    assert_eq!(result, true);
}

#[test]
fn test_is_word_byte_uppercase_a() {
    let result = is_word_byte(b'A');
    assert_eq!(result, true);
}

#[test]
fn test_is_word_byte_uppercase_z() {
    let result = is_word_byte(b'Z');
    assert_eq!(result, true);
}

