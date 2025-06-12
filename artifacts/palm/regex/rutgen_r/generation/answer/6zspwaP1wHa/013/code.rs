// Answer 0

#[derive(Debug)]
struct TestStruct {
    byte: Option<u8>,
}

impl TestStruct {
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
fn test_is_ascii_word_with_lowercase_a() {
    let test_struct = TestStruct { byte: Some(b'a') };
    assert!(test_struct.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_lowercase_z() {
    let test_struct = TestStruct { byte: Some(b'z') };
    assert!(test_struct.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_uppercase_a() {
    let test_struct = TestStruct { byte: Some(b'A') };
    assert!(test_struct.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_uppercase_z() {
    let test_struct = TestStruct { byte: Some(b'Z') };
    assert!(test_struct.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_digit_0() {
    let test_struct = TestStruct { byte: Some(b'0') };
    assert!(test_struct.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_digit_9() {
    let test_struct = TestStruct { byte: Some(b'9') };
    assert!(test_struct.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_underscore() {
    let test_struct = TestStruct { byte: Some(b'_') };
    assert!(test_struct.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_none() {
    let test_struct = TestStruct { byte: None };
    assert!(!test_struct.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_non_ascii() {
    let test_struct = TestStruct { byte: Some(b'!') };
    assert!(!test_struct.is_ascii_word());
}

