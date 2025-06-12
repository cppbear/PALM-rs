// Answer 0

#[test]
fn test_parse_escape_valid_r() {
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

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0) // Mocked implementation for the test
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
    let mut reader = MockRead::new(vec![b'\\', b'r']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_valid_b() {
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

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0) // Mocked implementation for the test
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
    let mut reader = MockRead::new(vec![b'\\', b'b']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_valid_n() {
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

        fn discard(&mut self) {}

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0) // Mocked implementation for the test
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
    let mut reader = MockRead::new(vec![b'\\', b'n']);
    
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\n']);
}

