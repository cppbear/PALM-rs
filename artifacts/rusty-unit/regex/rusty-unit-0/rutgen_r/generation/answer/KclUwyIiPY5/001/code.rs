// Answer 0

#[derive(Debug)]
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
fn test_is_end_with_none() {
    let input = Input { c: None, byte: None };
    assert!(input.is_end());
}

#[test]
fn test_is_end_with_some_character() {
    let input = Input { c: Some('a'), byte: None };
    assert!(!input.is_end());
}

#[test]
fn test_is_end_with_some_byte() {
    let input = Input { c: None, byte: Some(1) };
    assert!(!input.is_end());
}

#[test]
fn test_is_end_with_some_character_and_byte() {
    let input = Input { c: Some('a'), byte: Some(1) };
    assert!(!input.is_end());
}

#[test]
fn test_is_end_with_other_none_conditions() {
    let input1 = Input { c: None, byte: Some(0) };
    assert!(!input1.is_end());

    let input2 = Input { c: Some('b'), byte: None };
    assert!(!input2.is_end());
}

