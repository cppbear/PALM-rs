// Answer 0

#[test]
fn test_is_ascii_word_with_underscore() {
    struct TestStruct;

    impl TestStruct {
        fn as_byte(&self) -> Option<u8> {
            Some(b'_')
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

    let test_struct = TestStruct;
    assert_eq!(test_struct.is_ascii_word(), true);
}

#[test]
fn test_is_ascii_word_with_non_word_character() {
    struct TestStruct;

    impl TestStruct {
        fn as_byte(&self) -> Option<u8> {
            Some(b'!')
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

    let test_struct = TestStruct;
    assert_eq!(test_struct.is_ascii_word(), false);
}

#[test]
fn test_is_ascii_word_with_none() {
    struct TestStruct;

    impl TestStruct {
        fn as_byte(&self) -> Option<u8> {
            None
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

    let test_struct = TestStruct;
    assert_eq!(test_struct.is_ascii_word(), false);
}

