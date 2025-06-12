// Answer 0

#[test]
fn test_parse_unicode_escape_invalid_leading_surrogate() {
    struct DummyReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl DummyReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos < self.input.len() {
                let val = u16::from_str_radix(&String::from_utf8_lossy(&self.input[self.pos..self.pos + 4]).to_string(), 16).unwrap();
                self.pos += 4;
                Ok(val)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue)) // Dummy error for EOF
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    impl Read<'_> for DummyReader {
        // Implement the necessary traits and methods for Read
    }

    let mut scratch = Vec::new();
    let mut reader = DummyReader::new(b"\\uD800\\uDBFF\\u1234".to_vec()); // Leading surrogate sequence
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    assert!(result.is_err());
} 

#[test]
fn test_parse_unicode_escape_invalid_trailing_surrogate() {
    struct DummyReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl DummyReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos < self.input.len() {
                let val = u16::from_str_radix(&String::from_utf8_lossy(&self.input[self.pos..self.pos + 4]).to_string(), 16).unwrap();
                self.pos += 4;
                Ok(val)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue)) // Dummy error for EOF
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    impl Read<'_> for DummyReader {
        // Implement the necessary traits and methods for Read
    }

    let mut scratch = Vec::new();
    let mut reader = DummyReader::new(b"\\uD800\\uD800\\u1234".to_vec()); // Invalid trailing surrogate
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
} 

#[test]
fn test_parse_unicode_escape_valid_utf16() {
    struct DummyReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl DummyReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos < self.input.len() {
                let val = u16::from_str_radix(&String::from_utf8_lossy(&self.input[self.pos..self.pos + 4]).to_string(), 16).unwrap();
                self.pos += 4;
                Ok(val)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue)) // Dummy error for EOF
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    impl Read<'_> for DummyReader {
        // Implement the necessary traits and methods for Read
    }

    let mut scratch = Vec::new();
    let mut reader = DummyReader::new(b"\\uD800\\uDC00\\u1234".to_vec()); // Valid UTF-16 surrogate pair
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![0xF0, 0x9D, 0x9A, 0x80]); // Expected output after decoding
} 

#[test]
fn test_parse_unicode_escape_unexpected_character() {
    struct DummyReader {
        input: Vec<u8>,
        pos: usize,
    }

    impl DummyReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos < self.input.len() {
                let val = u16::from_str_radix(&String::from_utf8_lossy(&self.input[self.pos..self.pos + 4]).to_string(), 16).unwrap();
                self.pos += 4;
                Ok(val)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue)) // Dummy error for EOF
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    impl Read<'_> for DummyReader {
        // Implement the necessary traits and methods for Read
    }

    let mut scratch = Vec::new();
    let mut reader = DummyReader::new(b"\\uD800\\xFF\\u1234".to_vec()); // Unexpected character
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
}

