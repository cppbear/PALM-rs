// Answer 0

#[test]
fn test_parse_unicode_escape_valid_character() {
    struct MockRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                pos: 0,
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos + 4 <= self.input.len() {
                let hex = std::str::from_utf8(&self.input[self.pos..self.pos + 4]).unwrap();
                self.pos += 4;
                u16::from_str_radix(hex, 16).map_err(|_| Error::new(ErrorCode::InvalidEscape))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Option<u8> {
            if self.pos < self.input.len() {
                Some(self.input[self.pos])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }
    }

    impl Read<'_> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            MockRead::decode_hex_escape(self)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(MockRead::peek(self))
        }

        fn discard(&mut self) {
            MockRead::discard(self);
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(r"\uD800");

    // Set validate to true
    let validate = true;
    
    let result = parse_unicode_escape(&mut read, validate, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::new(ErrorCode::LoneLeadingSurrogateInHexEscape));
}

#[test]
fn test_parse_unicode_escape_lone_surrogate() {
    struct MockRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                pos: 0,
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos + 4 <= self.input.len() {
                let hex = std::str::from_utf8(&self.input[self.pos..self.pos + 4]).unwrap();
                self.pos += 4;
                u16::from_str_radix(hex, 16).map_err(|_| Error::new(ErrorCode::InvalidEscape))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Option<u8> {
            if self.pos < self.input.len() {
                Some(self.input[self.pos])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }
    }

    impl Read<'_> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            MockRead::decode_hex_escape(self)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(MockRead::peek(self))
        }

        fn discard(&mut self) {
            MockRead::discard(self);
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(r"\uD800\\uDFFF");

    // Set validate to true
    let validate = true;
    
    let result = parse_unicode_escape(&mut read, validate, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch.len(), 0); // expecting no output yet
}

