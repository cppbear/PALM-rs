// Answer 0

#[test]
fn test_parse_unicode_escape_leading_surrogate_with_validation() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.input.len() {
                let hex = self.input[self.index];
                self.index += 1;
                Ok(hex) // Mocking valid hex decoding
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape) // Indicating unexpected end
            }
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }

        fn discard(&mut self) {
            // Discard the next byte
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead { input: vec![0xD8, 0x00, b'\\', b'u', 0xDC, 0x00], index: 0 };

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch.len(), 4); // Adjust based on expected UTF-8 length
}

#[test]
fn test_parse_unicode_escape_trailing_surrogate_with_validation() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.input.len() {
                let hex = self.input[self.index];
                self.index += 1;
                Ok(hex) // Mocking valid hex decoding
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape) // Indicating unexpected end
            }
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }

        fn discard(&mut self) {
            // Discard the next byte
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead { input: vec![0xD8, 0x00, b'\\', b'u', 0xDC, 0x00, 0xD9, 0x00], index: 0 };

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch.len(), 4); // Adjust based on expected UTF-8 length for the paired surrogate
}

#[test]
#[should_panic]
fn test_parse_unicode_escape_lone_surrogate_with_validation() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.input.len() {
                let hex = self.input[self.index];
                self.index += 1;
                Ok(hex) // Mocking valid hex decoding
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape) // Indicating unexpected end
            }
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }

        fn discard(&mut self) {
            // Discard the next byte
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead { input: vec![0xD8, 0x00, b'\\', b'u'], index: 0 }; // Lone leading surrogate

    let _result = parse_unicode_escape(&mut read, true, &mut scratch);
    // Should panic due to validation for lone surrogate
}

