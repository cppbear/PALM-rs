// Answer 0

#[test]
fn test_parse_unicode_escape_valid_sequence() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.data.len() {
                let hex = self.data[self.index];
                self.index += 1;
                Ok(hex.into()) // Directly return the value for testing
            } else {
                Err(ErrorCode::EndOfStream) // Simulate end of stream
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(ErrorCode::EndOfStream)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead { data: vec![0xD800, b'\\', b'u', 0xDC00], index: 0 };

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_ok());
    assert!(!scratch.is_empty());
    assert_eq!(scratch[0], 0xED); // Check for the expected byte output in scratch
}

#[test]
fn test_parse_unicode_escape_with_lone_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.data.len() {
                let hex = self.data[self.index];
                self.index += 1;
                Ok(hex.into()) // Directly return the value for testing
            } else {
                Err(ErrorCode::EndOfStream)
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(ErrorCode::EndOfStream)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead { data: vec![0xD800, b'\\', b'u', 0xD800], index: 0 };

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), ErrorCode::LoneLeadingSurrogateInHexEscape));
}

#[test]
fn test_parse_unicode_escape_invalid_sequence() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.data.len() {
                let hex = self.data[self.index];
                self.index += 1;
                Ok(hex.into()) // Directly return the value for testing
            } else {
                Err(ErrorCode::EndOfStream)
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(ErrorCode::EndOfStream)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead { data: vec![0xDBFF, b'\\', b'u', 0xD800], index: 0 };

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_ok());
    assert!(!scratch.is_empty());
    assert_eq!(scratch[0], 0xED); // Check for the expected byte output in scratch
}

