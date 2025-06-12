// Answer 0

#[test]
fn test_parse_unicode_escape_valid() {
    struct MockRead {
        data: Vec<u8>,
        cursor: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, cursor: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            if self.cursor + 4 > self.data.len() {
                return Err(Error::from(ErrorCode::EofWhileParsingString));
            }
            let hex_str = str::from_utf8(&self.data[self.cursor..self.cursor + 4]).unwrap();
            self.cursor += 4;
            i16::from_str_radix(hex_str, 16).map_err(|_| Error::from(ErrorCode::InvalidEscape))
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.cursor >= self.data.len() {
                return Ok(None);
            }
            Ok(Some(self.data[self.cursor]))
        }

        fn discard(&mut self) {
            self.cursor += 1;
        }
    }

    impl Deref for MockRead {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }
    
    let mut read = MockRead::new(b"\\u0061\\u007A".to_vec()); // represents "a" and "z"
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, b"a");
}

#[test]
fn test_parse_unicode_escape_lone_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        cursor: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, cursor: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            if self.cursor + 4 > self.data.len() {
                return Err(Error::from(ErrorCode::EofWhileParsingString));
            }
            let hex_str = str::from_utf8(&self.data[self.cursor..self.cursor + 4]).unwrap();
            self.cursor += 4;
            i16::from_str_radix(hex_str, 16).map_err(|_| Error::from(ErrorCode::InvalidEscape))
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.cursor >= self.data.len() {
                return Ok(None);
            }
            Ok(Some(self.data[self.cursor]))
        }

        fn discard(&mut self) {
            self.cursor += 1;
        }
    }

    impl Deref for MockRead {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }
    
    let mut read = MockRead::new(b"\\uD800".to_vec()); // represents a lone leading surrogate
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().err.code, ErrorCode::LoneLeadingSurrogateInHexEscape);
}

