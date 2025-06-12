// Answer 0

#[test]
fn test_parse_unicode_escape_valid_case() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, pos: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            if self.pos < self.data.len() {
                let byte = self.data[self.pos];
                self.pos += 1;
                Ok(byte as u16) // simplified for the test
            } else {
                Err(Error { err: Box::new(ErrorCode::EofWhileParsingValue) })
            }
        }

        fn peek(&self) -> Result<Option<u8>, Error> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD8, 0x00, b'\\', b'u']);

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_unicode_escape_lone_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            if self.pos < self.data.len() {
                let byte = self.data[self.pos];
                self.pos += 1;
                Ok(byte as u16) // simplified for the test
            } else {
                Err(Error { err: Box::new(ErrorCode::EofWhileParsingValue) })
            }
        }

        fn peek(&self) -> Result<Option<u8>, Error> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD8, 0x00, b'\\', b'u', 0xDC]);

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_err());
}

#[test]
fn test_parse_unicode_escape_non_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            if self.pos < self.data.len() {
                let byte = self.data[self.pos];
                self.pos += 1;
                Ok(byte as u16) // simplified for the test
            } else {
                Err(Error { err: Box::new(ErrorCode::EofWhileParsingValue) })
            }
        }

        fn peek(&self) -> Result<Option<u8>, Error> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0x60, b'\\', b'u']); // 0x60 is valid

    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    assert!(result.is_ok());
}

