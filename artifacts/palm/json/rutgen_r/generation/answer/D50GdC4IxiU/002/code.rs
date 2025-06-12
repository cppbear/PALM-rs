// Answer 0

#[test]
fn test_parse_unicode_escape_lone_leading_surrogate() {
    struct TestReader {
        input: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.input.len() {
                let val = self.input[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {
            // No-op for test
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![0xDC, 0x00]); // Input simulates \uDC00

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code(), ErrorCode::LoneLeadingSurrogateInHexEscape);
}

#[test]
fn test_parse_unicode_escape_surrogate_pair() {
    struct TestReader {
        input: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.input.len() {
                let val = self.input[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {
            // No-op for test
        }
        
        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![0xD8, 0x00, 0xDC, 0x00]); // Input simulates \uD800\uDC00

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, vec![0xF0, 0x90, 0x80, 0x80]); // U+10000 encoded as UTF-8
}

