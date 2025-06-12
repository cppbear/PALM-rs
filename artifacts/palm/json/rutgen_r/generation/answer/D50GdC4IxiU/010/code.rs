// Answer 0

#[test]
fn test_parse_unicode_escape_valid_scenario() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            // Simulate returning a specific value as per the test case
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;

                // Simulate a valid escape sequence, such as `0xDC00`
                if ch == b'D' {
                    return Ok(0xDC00);
                } else if ch == b'D' {
                    return Ok(0xD800);
                } else {
                    return Ok(0xDFFF);
                }
            }
            Err(ErrorCode::UnexpectedEndOfHexEscape)
        }

        fn discard(&mut self) {
            // Simulate discarding a character
            self.position += 1;
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader::new(vec![b'D', b'\\', b'u', b'1', b'2']);

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);

    assert!(result.is_err());
    assert_eq!(reader.position, 4); // Position should reflect parsing
}

#[test]
fn test_parse_unicode_escape_with_lone_leading_surrogate() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;

                if ch == b'D' {
                    return Ok(0xD800); // Leading surrogate
                } else if ch == b'\\' {
                    return Ok(0xDC00); // Trailing surrogate
                } else if ch == b'D' {
                    return Ok(0xDFFF);
                }
            }
            Err(ErrorCode::UnexpectedEndOfHexEscape)
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader::new(vec![b'D', b'\\', b'u']);

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);

    assert!(result.is_err());
}

#[test]
fn test_parse_unicode_escape_with_no_trailing_surrogate() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            // Mock the return of a hex escape value
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;

                if ch == b'D' { return Ok(0xD800); } // Leading surrogate
                if ch == b'\\' { return Ok(0x20000); } // No trailing surrogate case
            }
            Err(ErrorCode::UnexpectedEndOfHexEscape)
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader::new(vec![b'D', b'\\', b'u']);

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);

    assert!(result.is_err());
}

