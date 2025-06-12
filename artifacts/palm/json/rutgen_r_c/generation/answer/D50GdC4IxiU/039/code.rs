// Answer 0

#[test]
fn test_parse_unicode_escape_valid_input() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position + 4 <= self.data.len() {
                let hex_str = std::str::from_utf8(&self.data[self.position..self.position + 4]).unwrap();
                self.position += 4;
                u16::from_str_radix(hex_str, 16).map_err(|_| Error::from(ErrorCode::InvalidEscape)).map(|val| val)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingValue))
            }
        }
        
        fn peek(&self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    impl Read<'_> for MockRead {
        // Implement required Read trait methods here
    }

    let data = b"D800\\uDC00".to_vec(); // Valid surrogate pair
    let mut read = MockRead::new(data);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    
    assert_eq!(result, Ok(()));
    assert!(!scratch.is_empty());
}

#[test]
fn test_parse_unicode_escape_lone_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position + 4 <= self.data.len() {
                let hex_str = std::str::from_utf8(&self.data[self.position..self.position + 4]).unwrap();
                self.position += 4;
                u16::from_str_radix(hex_str, 16).map_err(|_| Error::from(ErrorCode::InvalidEscape)).map(|val| val)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingValue))
            }
        }
        
        fn peek(&self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    impl Read<'_> for MockRead {
        // Implement required Read trait methods here
    }

    let data = b"D800\\uD800".to_vec(); // Lone leading surrogate
    let mut read = MockRead::new(data);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorCode::LoneLeadingSurrogateInHexEscape);
} 

#[test]
fn test_parse_unicode_escape_invalid_escape() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position + 4 <= self.data.len() {
                let hex_str = std::str::from_utf8(&self.data[self.position..self.position + 4]).unwrap();
                self.position += 4;
                u16::from_str_radix(hex_str, 16).map_err(|_| Error::from(ErrorCode::InvalidEscape)).map(|val| val)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingValue))
            }
        }
        
        fn peek(&self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    impl Read<'_> for MockRead {
        // Implement required Read trait methods here
    }

    let data = b"D800\\uXXXX".to_vec(); // Invalid escape sequence
    let mut read = MockRead::new(data);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorCode::InvalidEscape);
} 

#[test]
fn test_parse_unicode_escape_eof() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position + 4 <= self.data.len() {
                let hex_str = std::str::from_utf8(&self.data[self.position..self.position + 4]).unwrap();
                self.position += 4;
                u16::from_str_radix(hex_str, 16).map_err(|_| Error::from(ErrorCode::InvalidEscape)).map(|val| val)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingValue))
            }
        }
        
        fn peek(&self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    impl Read<'_> for MockRead {
        // Implement required Read trait methods here
    }

    let data = b"D800\\u".to_vec(); // End of input
    let mut read = MockRead::new(data);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorCode::UnexpectedEndOfHexEscape);
}

