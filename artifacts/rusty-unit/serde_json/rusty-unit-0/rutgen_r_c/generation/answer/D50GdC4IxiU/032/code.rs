// Answer 0

#[test]
fn test_parse_unicode_escape_valid_surrogate_pair() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(value as u16) // Simulating decoded hex escape
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
    }
    
    let mut scratch = Vec::new();
    let mut reader = MockReader::new(vec![0xD8u8, 0x00, b'\\', b'u', 0xDC, 0x00]); // Surrogate pair (0xD800, 0xDC00)
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert!(!scratch.is_empty());
} 

#[test]
fn test_parse_unicode_escape_lone_surrogate() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(value as u16) // Simulating decoded hex escape
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader::new(vec![0xD9u8, 0x00, b'\\', b'u']); // Lone surrogate (0xD900)
    let result = parse_unicode_escape(&mut reader, true, &mut scratch); // Validate is true
    assert!(result.is_err());
    if let Err(ref e) = result {
        assert_eq!(*e, Error::from(ErrorCode::LoneLeadingSurrogateInHexEscape));
    }
}

