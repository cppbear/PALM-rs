// Answer 0

#[test]
fn test_parse_unicode_escape_with_lone_surrogate_no_validation() {
    use std::io::Cursor;

    struct MockReader {
        input: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.input.len() {
                let hex = self.input[self.index];
                self.index += 1;
                Ok(hex as u16)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let input = vec![0xD800, b'\\', b'u']; // leading surrogate
    let mut reader = MockReader::new(input);
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_err());
}

#[test]
fn test_parse_unicode_escape_with_invalid_trailing_surrogate_no_validation() {
    use std::io::Cursor;

    struct MockReader {
        input: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.input.len() {
                let hex = self.input[self.index];
                self.index += 1;
                Ok(hex as u16)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn peek_or_eof(&self) -> Result<u8> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let input = vec![0xD800, b'\\', b'u', 0xD800]; // leading and trailing surrogate
    let mut reader = MockReader::new(input);
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_err());
}

