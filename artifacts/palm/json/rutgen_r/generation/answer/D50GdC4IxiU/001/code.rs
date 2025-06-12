// Answer 0

#[test]
fn test_parse_unicode_escape_invalid_hex_escape() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulate an error during hex escape decoding
            Err(ErrorCode::InvalidHexEscape.into())
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            // Simulate end of input
            Err(ErrorCode::UnexpectedEndOfInput.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockRead {
        input: vec![],
        index: 0,
    };

    // Validate is true; expect an error due to the hex escape decoding failure
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
} 

#[test]
fn test_parse_unicode_escape_invalid_surrogate() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulate returning a leading surrogate without a valid trailing surrogate
            if self.index == 0 {
                self.index += 1;
                Ok(0xD800) // Leading surrogate
            } else {
                Err(ErrorCode::InvalidHexEscape.into()) // Error on second call
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            // Simulate reading '\\u'
            Ok(b'u')
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockRead {
        input: vec![],
        index: 0,
    };

    // Validate is true; expect an error due to lone leading surrogate
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
} 

#[test]
fn test_parse_unicode_escape_unexpected_end() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulate a valid leading surrogate
            if self.index == 0 {
                self.index += 1;
                Ok(0xD800)
            } else {
                Ok(0xDC00) // Valid trailing surrogate
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            // Simulate end of input after the first escape
            Err(ErrorCode::UnexpectedEndOfInput.into())
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockRead {
        input: vec![],
        index: 0,
    };

    // Validate is true; expect an error due to unexpected end
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
} 

#[test]
fn test_parse_unicode_escape_success() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulate valid UTF-16 surrogate pairs
            if self.index == 0 {
                self.index += 1;
                Ok(0xD800) // Leading surrogate
            } else {
                Ok(0xDC00) // Trailing surrogate
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            Ok(b'u')
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockRead {
        input: vec![],
        index: 0,
    };

    // Validate is false; expect success processing surrogate pairs
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch.len(), 4); // Check the size of the scratch vector after processing
}

