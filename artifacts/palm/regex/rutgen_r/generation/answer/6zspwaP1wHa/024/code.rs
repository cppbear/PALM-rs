// Answer 0

#[test]
fn test_is_ascii_word_none() {
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

    let test_instance = TestStruct;
    assert_eq!(test_instance.is_ascii_word(), false);
}

