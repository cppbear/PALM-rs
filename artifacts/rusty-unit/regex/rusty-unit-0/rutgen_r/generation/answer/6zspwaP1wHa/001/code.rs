// Answer 0

#[test]
fn test_is_ascii_word_with_uppercase_A() {
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

    let test = TestStruct { byte: Some(b'A') };
    assert!(test.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_lowercase_a() {
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

    let test = TestStruct { byte: Some(b'a') };
    assert!(test.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_digit_0() {
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

    let test = TestStruct { byte: Some(b'0') };
    assert!(test.is_ascii_word());
}

#[test]
fn test_is_ascii_word_with_underscore() {
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

    let test = TestStruct { byte: Some(b'_') };
    assert!(test.is_ascii_word());
}

