// Answer 0

#[test]
fn test_parse_unicode_escape_with_lone_leading_surrogate() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<i16> {
            if self.position < self.input.len() {
                let hex_value = self.input[self.position] as i16;
                self.position += 1;
                Ok(hex_value)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xD800, b'\\', b'u']);

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::LoneLeadingSurrogateInHexEscape);
}

#[test]
fn test_parse_unicode_escape_with_valid_input() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<i16> {
            if self.position < self.input.len() {
                let hex_value = self.input[self.position] as i16;
                self.position += 1;
                Ok(hex_value)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xD800, b'\\', b'u', 0xDC00]); // Valid surrogate pair

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_ok());
    assert!(!scratch.is_empty());
}

