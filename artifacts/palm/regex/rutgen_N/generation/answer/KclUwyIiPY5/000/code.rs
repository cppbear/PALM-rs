// Answer 0

#[derive(Default)]
struct Input {
    c: Option<char>,
    byte: Option<u8>,
}

impl Input {
    pub fn is_end(&self) -> bool {
        self.c.is_none() && self.byte.is_none()
    }
}

#[test]
fn test_is_end_both_none() {
    let input = Input::default();
    assert!(input.is_end());
}

#[test]
fn test_is_end_char_some() {
    let input = Input {
        c: Some('a'),
        byte: None,
    };
    assert!(!input.is_end());
}

#[test]
fn test_is_end_byte_some() {
    let input = Input {
        c: None,
        byte: Some(255),
    };
    assert!(!input.is_end());
}

#[test]
fn test_is_end_both_some() {
    let input = Input {
        c: Some('a'),
        byte: Some(255),
    };
    assert!(!input.is_end());
}

