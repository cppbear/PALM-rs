// Answer 0

#[test]
fn test_decode_hex_escape_invalid_escape() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn next_or_eof(&mut self) -> Result<u8, &'static str> {
            if self.position < self.input.len() {
                let value = self.input[self.position];
                self.position += 1;
                Ok(value)
            } else {
                Err("EOF")
            }
        }
    }

    let mut reader = TestReader {
        input: vec![b'G', b'H', b'I', b'J'], // These values are not valid hexadecimal digits.
        position: 0,
    };

    let result = decode_hex_escape(&mut reader);
    assert!(result.is_err());
}

