// Answer 0

#[test]
fn test_is_ascii_word_with_uppercase_A() {
    let byte = Byte::byte(b'A');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_with_lowercase_a() {
    let byte = Byte::byte(b'a');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_with_lowercase_z() {
    let byte = Byte::byte(b'z');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_with_digit_0() {
    let byte = Byte::byte(b'0');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_with_digit_9() {
    let byte = Byte::byte(b'9');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_with_underscore() {
    let byte = Byte::byte(b'_');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_with_non_ascii_below_A() {
    let byte = Byte::byte(0);
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_with_non_ascii_above_z() {
    let byte = Byte::byte(123); // Just above 'z'
    byte.is_ascii_word();
}

