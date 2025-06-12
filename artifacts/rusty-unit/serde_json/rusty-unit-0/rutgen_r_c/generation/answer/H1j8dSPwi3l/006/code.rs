// Answer 0

#[test]
fn test_parse_escape_valid_n() {
    struct MockRead {
        data: Vec<u8>,
        current: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Option<Result<u8>> {
            if self.current < self.data.len() {
                let value = self.data[self.current];
                self.current += 1;
                Some(Ok(value))
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<u32> {
            Ok(0) // Simplified for this test
        }
        
        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.current < self.data.len() {
                Ok(self.data[self.current])
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut mock_read = MockRead {
        data: vec![b'\\', b'n'], // Backslash followed by 'n'
        current: 0,
    };

    let result = parse_escape(&mut mock_read, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\n']);
}

#[test]
fn test_parse_escape_valid_b() {
    struct MockRead {
        data: Vec<u8>,
        current: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Option<Result<u8>> {
            if self.current < self.data.len() {
                let value = self.data[self.current];
                self.current += 1;
                Some(Ok(value))
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<u32> {
            Ok(0) // Simplified for this test
        }
        
        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.current < self.data.len() {
                Ok(self.data[self.current])
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut mock_read = MockRead {
        data: vec![b'\\', b'b'], // Backslash followed by 'b'
        current: 0,
    };

    let result = parse_escape(&mut mock_read, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_invalid_escape() {
    struct MockRead {
        data: Vec<u8>,
        current: usize,
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Option<Result<u8>> {
            if self.current < self.data.len() {
                let value = self.data[self.current];
                self.current += 1;
                Some(Ok(value))
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<u32> {
            Ok(0) // Simplified for this test
        }
        
        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.current < self.data.len() {
                Ok(self.data[self.current])
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut mock_read = MockRead {
        data: vec![b'\\', b'x'], // Backslash followed by an invalid escape 'x'
        current: 0,
    };

    let result = parse_escape(&mut mock_read, false, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().code(), ErrorCode::InvalidEscape);
}

