// Answer 0

#[test]
fn test_parse_unicode_escape_valid_substitution() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position >= self.data.len() {
                return Err(ErrorCode::UnexpectedEndOfInput.into());
            }
            let value = self.data[self.position];
            self.position += 1;
            Ok(value as u16)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position >= self.data.len() {
                return Err(ErrorCode::UnexpectedEndOfInput.into());
            }
            Ok(self.data[self.position])
        }
    }

    let mut scratch: Vec<u8> = Vec::new();
    let mut mock_reader = MockReader {
        data: vec![0xD8, 0x00, 0xDC, 0x00], // surrogate pair for U+10000
        position: 0,
    };
    
    let result = parse_unicode_escape(&mut mock_reader, false, &mut scratch);
    
    assert!(result.is_ok());
}

#[test]
fn test_parse_unicode_escape_invalid_surrogate() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position >= self.data.len() {
                return Err(ErrorCode::UnexpectedEndOfInput.into());
            }
            let value = self.data[self.position];
            self.position += 1;
            Ok(value as u16)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position >= self.data.len() {
                return Err(ErrorCode::UnexpectedEndOfInput.into());
            }
            Ok(self.data[self.position])
        }
    }

    let mut scratch: Vec<u8> = Vec::new();
    let mut mock_reader = MockReader {
        data: vec![0xD8, 0x00, 0xD8, 0x00], // lone leading surrogate
        position: 0,
    };
    
    let result = parse_unicode_escape(&mut mock_reader, true, &mut scratch);
    
    assert!(result.is_err());
}

#[test]
fn test_parse_unicode_escape_unexpected_end() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position >= self.data.len() {
                return Err(ErrorCode::UnexpectedEndOfInput.into());
            }
            let value = self.data[self.position];
            self.position += 1;
            Ok(value as u16)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position >= self.data.len() {
                return Err(ErrorCode::UnexpectedEndOfInput.into());
            }
            Ok(self.data[self.position])
        }
    }

    let mut scratch: Vec<u8> = Vec::new();
    let mut mock_reader = MockReader {
        data: vec![0xD8, 0x00, 0x5C], // first part of a surrogate and then '\\'
        position: 0,
    };
    
    let result = parse_unicode_escape(&mut mock_reader, true, &mut scratch);
    
    assert!(result.is_err());
}

