// Answer 0

#[test]
fn test_parse_unicode_escape_with_leading_surrogate_valid() {
    struct MockReader {
        position: usize,
        input: Vec<u8>,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            Self { position: 0, input }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(val as u32)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        // Additional methods for Read trait can be implemented as needed
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader::new(vec![0xD800, b'\\', b'u', 0xDC00]);
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_with_lone_surrogate() {
    struct MockReader {
        position: usize,
        input: Vec<u8>,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            Self { position: 0, input }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(val as u32)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader::new(vec![0xD800, b'\\', b'u', 0xC000]);
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
}

#[test]
#[should_panic]
fn test_parse_unicode_escape_with_invalid_trailing_surrogate() {
    struct MockReader {
        position: usize,
        input: Vec<u8>,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            Self { position: 0, input }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(val as u32)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader::new(vec![0xD800, b'\\', b'u', 0xD000]);
    let _result = parse_unicode_escape(&mut reader, true, &mut scratch);
}

