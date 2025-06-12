// Answer 0

#[test]
fn test_is_ascii_word_uppercase_a() {
    let byte = Byte::byte(b'A');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_uppercase_z() {
    let byte = Byte::byte(b'Z');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_lowercase_a() {
    let byte = Byte::byte(b'a');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_lowercase_z() {
    let byte = Byte::byte(b'z');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_numeric_0() {
    let byte = Byte::byte(b'0');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_numeric_9() {
    let byte = Byte::byte(b'9');
    byte.is_ascii_word();
}

#[test]
fn test_is_ascii_word_underscore() {
    let byte = Byte::byte(b'_');
    byte.is_ascii_word();
}

