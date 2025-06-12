// Answer 0

fn test_parse_unicode_escape_valid_surrogate() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            MockRead { input, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.input.len() {
                let value = self.input[self.index];
                self.index += 1;
                Ok(value as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {
            // Simulate discarding a byte
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xDC, 0x00, 0xDC, 0x01]); // Example input for a valid surrogate pair

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![0xF0, 0x9D, 0x9C, 0x80]); // Expected output corresponding to the surrogate pair
}

fn test_parse_unicode_escape_lone_leading_surrogate() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            MockRead { input, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.input.len() {
                let value = self.input[self.index];
                self.index += 1;
                Ok(value as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xDC, 0x00]); // Lone leading surrogate input

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
}

fn test_parse_unicode_escape_unexpected_end() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            MockRead { input, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.input.len() {
                let value = self.input[self.index];
                self.index += 1;
                Ok(value as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xD8]); // Input with only leading surrogate

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err()); // Expect an error due to unexpected end
}

