// Answer 0

fn main() {}

struct AsciiWord {
    byte: Option<u8>,
}

impl AsciiWord {
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
fn test_is_ascii_word_with_underscore() {
    let word = AsciiWord { byte: Some(b'_') };
    assert!(word.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_uppercase() {
    let word = AsciiWord { byte: Some(b'A') };
    assert!(word.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_lowercase() {
    let word = AsciiWord { byte: Some(b'a') };
    assert!(word.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_numeric() {
    let word = AsciiWord { byte: Some(b'0') };
    assert!(word.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_invalid_byte() {
    let word = AsciiWord { byte: Some(b'#') };
    assert!(!word.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_none() {
    let word = AsciiWord { byte: None };
    assert!(!word.is_ascii_word());
}

