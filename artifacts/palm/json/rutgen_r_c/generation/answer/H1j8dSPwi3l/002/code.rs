// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Mock implementation for hex escape
            Ok(0)
        }

        fn discard(&mut self) {
            // Mock discard implementation
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let data = vec![b'\\', b'"'];
    let mut reader = MockReader::new(data);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_backslash() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let data = vec![b'\\', b'\\'];
    let mut reader = MockReader::new(data);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\\']);
}

#[test]
fn test_parse_escape_b() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let data = vec![b'\\', b'b'];
    let mut reader = MockReader::new(data);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_f() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let data = vec![b'\\', b'f'];
    let mut reader = MockReader::new(data);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x0c']);
}

#[test]
fn test_parse_escape_n() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let data = vec![b'\\', b'n'];
    let mut reader = MockReader::new(data);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\n']);
}

#[test]
fn test_parse_escape_r() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let data = vec![b'\\', b'r'];
    let mut reader = MockReader::new(data);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_t() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let data = vec![b'\\', b't'];
    let mut reader = MockReader::new(data);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\t']);
}

#[test]
fn test_parse_escape_slash() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let data = vec![b'\\', b'/'];
    let mut reader = MockReader::new(data);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'/']);
}

#[test]
fn test_parse_escape_invalid() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let data = vec![b'\\', b'x'];  // invalid escape
    let mut reader = MockReader::new(data);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().code(), ErrorCode::InvalidEscape);
}

