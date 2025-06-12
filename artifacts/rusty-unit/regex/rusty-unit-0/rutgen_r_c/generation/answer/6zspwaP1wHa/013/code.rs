// Answer 0

#[test]
fn test_is_ascii_word_with_lowercase_a() {
    let byte = Byte::byte(b'a'); // Testing with a lowercase letter
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_lowercase_z() {
    let byte = Byte::byte(b'z'); // Testing with a lowercase letter
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_uppercase_a() {
    let byte = Byte::byte(b'A'); // Testing with an uppercase letter
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_uppercase_z() {
    let byte = Byte::byte(b'Z'); // Testing with an uppercase letter
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_digit_0() {
    let byte = Byte::byte(b'0'); // Testing with a digit
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_digit_9() {
    let byte = Byte::byte(b'9'); // Testing with a digit
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_underscore() {
    let byte = Byte::byte(b'_'); // Testing with an underscore
    assert!(byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_non_ascii_character() {
    let byte = Byte::byte(0x80); // Testing with a non-ASCII value
    assert!(!byte.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_eof() {
    let byte = Byte::eof(); // Testing with EOF
    assert!(!byte.is_ascii_word());
}

