// Answer 0

#[test]
#[should_panic]
fn test_parse_unicode_escape_invalid_lone_leading_surrogate() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position < self.data.len() {
                let ch = self.data[self.position];
                self.position += 1;
                Ok(ch as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

    }

    let mut reader = MockReader::new(vec![0xDC, 0x00]); // Testing with invalid lone leading surrogate
    let validate = true;
    let mut scratch = Vec::new();
    let _ = parse_unicode_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_valid_surrogate_pair() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position < self.data.len() {
                let ch = self.data[self.position];
                self.position += 1;
                Ok(ch as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

    }

    let mut reader = MockReader::new(vec![0xD8, 0x00, b'\\', b'u', 0xDC, 0x00]); // Valid surrogate pair
    let validate = false;
    let mut scratch = Vec::new();
    let _ = parse_unicode_escape(&mut reader, validate, &mut scratch);
}

#[test]
#[should_panic]
fn test_parse_unicode_escape_invalid_trailing_surrogate() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position < self.data.len() {
                let ch = self.data[self.position];
                self.position += 1;
                Ok(ch as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

    }

    let mut reader = MockReader::new(vec![0xD8, 0x00, b'\\', b'u', 0xDFFF]); // Testing with invalid trailing surrogate
    let validate = true;
    let mut scratch = Vec::new();
    let _ = parse_unicode_escape(&mut reader, validate, &mut scratch);
}

