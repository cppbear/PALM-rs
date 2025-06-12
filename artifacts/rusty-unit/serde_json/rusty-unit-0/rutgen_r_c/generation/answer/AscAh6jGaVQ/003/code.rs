// Answer 0

#[test]
fn test_ignore_escape_invalid_escape_sequence() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            TestReader { input, position: 0 }
        }
    }

    impl Deref for TestReader {
        type Target = [u8];
        
        fn deref(&self) -> &Self::Target {
            &self.input
        }
    }

    impl TestReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<()> {
            // Simulate an error *on* decode_hex_escape
            Err(Error::from(ErrorCode::InvalidEscape))
        }
    }

    let mut reader = TestReader::new(vec![b'\\', b'u']);
    let result = ignore_escape(&mut reader);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.err.as_ref().downcast_ref::<ErrorCode>(), Some(&ErrorCode::InvalidEscape));
    }
}

