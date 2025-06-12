// Answer 0

#[test]
fn test_is_word_byte_underscore() {
    let c = b'_';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_digit_0() {
    let c = b'0';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_digit_9() {
    let c = b'9';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_lowercase_a() {
    let c = b'a';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_lowercase_z() {
    let c = b'z';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_uppercase_A() {
    let c = b'A';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_uppercase_Z() {
    let c = b'Z';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_lowercase_mid_range() {
    let c = b'm';
    is_word_byte(c);
}

#[test]
fn test_is_word_byte_uppercase_mid_range() {
    let c = b'M';
    is_word_byte(c);
}

