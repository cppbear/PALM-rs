// Answer 0

#[test]
fn test_is_word_byte_underscore() {
    let result = is_word_byte(b'_');
    assert_eq!(result, true);
}

#[test]
fn test_is_word_byte_numeric_false() {
    for c in b'0'..=b'9' {
        let result = is_word_byte(c);
        assert_eq!(result, false);
    }
}

#[test]
fn test_is_word_byte_lowercase_false() {
    for c in b'a'..=b'z' {
        let result = is_word_byte(c);
        assert_eq!(result, false);
    }
}

#[test]
fn test_is_word_byte_uppercase_false() {
    for c in b'A'..=b'Z' {
        let result = is_word_byte(c);
        assert_eq!(result, false);
    }
}

