// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // For simplicity, assume the hex escape is handled elsewhere
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
    let mut reader = MockRead::new(vec![b'\\', b'"']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_backslash() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // For simplicity, assume the hex escape is handled elsewhere
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
    let mut reader = MockRead::new(vec![b'\\', b'\\']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\\']);
}

#[test]
fn test_parse_escape_forward_slash() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // For simplicity, assume the hex escape is handled elsewhere
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
    let mut reader = MockRead::new(vec![b'\\', b'/']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'/']);
}

#[test]
fn test_parse_escape_backspace() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // For simplicity, assume the hex escape is handled elsewhere
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
    let mut reader = MockRead::new(vec![b'\\', b'b']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_form_feed() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // For simplicity, assume the hex escape is handled elsewhere
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
    let mut reader = MockRead::new(vec![b'\\', b'f']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x0c']);
}

#[test]
fn test_parse_escape_newline() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // For simplicity, assume the hex escape is handled elsewhere
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
    let mut reader = MockRead::new(vec![b'\\', b'n']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\n']);
}

#[test]
fn test_parse_escape_carriage_return() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // For simplicity, assume the hex escape is handled elsewhere
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
    let mut reader = MockRead::new(vec![b'\\', b'r']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_tab() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockRead {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // For simplicity, assume the hex escape is handled elsewhere
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
    let mut reader = MockRead::new(vec![b'\\', b't']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\t']);
}

