// Answer 0

#[test]
fn test_parse_unicode_escape_with_lone_leading_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
        
        fn discard(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
    }
    
    let mut scratch = Vec::new();
    let mut read = MockRead { data: vec![0xD800, b'\\', b'u', 0xDC00], index: 0 };

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_err());
    assert_eq!(read.index, 3);
}

#[test]
fn test_parse_unicode_escape_with_valid_bmp_char() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
        
        fn discard(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead { data: vec![0xD800, b'\\', b'u', 0x0041], index: 0 }; // U+0041 corresponds to 'A'

    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![0x41]); // 'A' in UTF-8
}

#[test]
fn test_parse_unicode_escape_with_lone_trailing_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead { data: vec![0xD800, b'\\', b'u', 0xDFFF], index: 0 };

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_err());
    assert_eq!(scratch.len(), 0); // should not push anything to scratch due to error
}

