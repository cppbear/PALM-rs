// Answer 0

#[test]
fn test_parse_unicode_escape_valid_case() {
    struct MockRead {
        buffer: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: &[u8]) -> Self {
            Self {
                buffer: input.to_vec(),
                position: 0,
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, ()> {
            if self.position < self.buffer.len() {
                let value = self.buffer[self.position];
                self.position += 1;
                Ok(value as u32)  // Simulating hex decoding
            } else {
                Err(())
            }
        }

        fn peek_or_eof(&mut self) -> Result<u8, ()> {
            if self.position < self.buffer.len() {
                Ok(self.buffer[self.position])
            } else {
                Err(())
            }
        }

        fn discard(&mut self) {
            if self.position < self.buffer.len() {
                self.position += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(&[0xD800, b'\\', b'u', 0xDC00]);

    let result = parse_unicode_escape(&mut read, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, vec![0x10_0000]); // Expected surrogate pair combined to U+10000
}

#[test]
fn test_parse_unicode_escape_with_invalid_surrogate() {
    struct MockReadWithInvalid {
        buffer: Vec<u8>,
        position: usize,
    }

    impl MockReadWithInvalid {
        fn new(input: &[u8]) -> Self {
            Self {
                buffer: input.to_vec(),
                position: 0,
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, ()> {
            if self.position < self.buffer.len() {
                let value = self.buffer[self.position];
                self.position += 1;
                Ok(value as u32)  // Simulating hex decoding
            } else {
                Err(())
            }
        }

        fn peek_or_eof(&mut self) -> Result<u8, ()> {
            if self.position < self.buffer.len() {
                Ok(self.buffer[self.position])
            } else {
                Err(())
            }
        }

        fn discard(&mut self) {
            if self.position < self.buffer.len() {
                self.position += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockReadWithInvalid::new(&[0xD800, b'\\', b'u', 0xDFFF]); // Lone leading surrogate

    let result = parse_unicode_escape(&mut read, true, &mut scratch);

    assert!(result.is_err()); // Expecting an error due to a lone leading surrogate
}

