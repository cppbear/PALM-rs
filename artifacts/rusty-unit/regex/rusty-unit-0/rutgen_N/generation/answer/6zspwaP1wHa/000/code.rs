// Answer 0

#[derive(Debug)]
struct TestChar {
    byte: Option<u8>,
}

impl TestChar {
    fn as_byte(&self) -> Option<u8> {
        self.byte
    }

    fn is_ascii_word(&self) -> bool {
        let b = match self.as_byte() {
            None => return false,
            Some(b) => b,
        };
        match b {
            b'A'...b'Z' | b'a'...b'z' | b'0'...b'9' | b'_' => true,
            _ => false,
        }
    }
}

#[test]
fn test_is_ascii_word_with_uppercase() {
    let test_char = TestChar { byte: Some(b'A') };
    assert!(test_char.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_lowercase() {
    let test_char = TestChar { byte: Some(b'a') };
    assert!(test_char.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_digit() {
    let test_char = TestChar { byte: Some(b'3') };
    assert!(test_char.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_underscore() {
    let test_char = TestChar { byte: Some(b'_') };
    assert!(test_char.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_invalid_byte() {
    let test_char = TestChar { byte: Some(b'#') };
    assert!(!test_char.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_none() {
    let test_char = TestChar { byte: None };
    assert!(!test_char.is_ascii_word());
}

