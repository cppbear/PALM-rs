// Answer 0

#[test]
fn test_next_char_or_null_valid_case() {
    struct TestInput {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestInput {
        fn next_char(&mut self) -> Result<u8> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "No more characters").into())
            }
        }
    
        fn next_char_or_null(&mut self) -> Result<u8> {
            Ok(self.next_char().unwrap_or(b'\x00'))
        }
    }

    let mut input = TestInput {
        chars: vec![b'a', b'b', b'c'],
        index: 0,
    };

    let result = input.next_char_or_null();
    assert_eq!(result.unwrap(), b'a');

    let result = input.next_char_or_null();
    assert_eq!(result.unwrap(), b'b');

    let result = input.next_char_or_null();
    assert_eq!(result.unwrap(), b'c');

    let result = input.next_char_or_null();
    assert_eq!(result.unwrap(), b'\x00');
}

#[test]
#[should_panic]
fn test_next_char_or_null_panic_case() {
    struct TestInput {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestInput {
        fn next_char(&mut self) -> Result<u8> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                panic!("Unexpected character retrieval.");
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8> {
            Ok(self.next_char().unwrap_or(b'\x00'))
        }
    }

    let mut input = TestInput {
        chars: vec![b'a', b'b', b'c'],
        index: 3, // Index set to trigger panic
    };

    input.next_char_or_null(); // This should panic
}

