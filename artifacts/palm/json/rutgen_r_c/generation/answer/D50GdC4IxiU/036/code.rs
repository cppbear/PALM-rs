// Answer 0

#[test]
fn test_parse_unicode_escape_valid_non_bmp() {
    struct MockReader {
        index: usize,
        data: Vec<u8>,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { index: 0, data }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(value as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
    }

    impl Read<'_> for MockReader {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.peek()?.ok_or(Error::new(ErrorCode::EofWhileParsingString))
        }
    }

    let mut reader = MockReader::new(vec![0xD8, 0x00, b'\\', b'u', 0xD8, 0x00, 0xDC, 0x00]);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().err, ErrorCode::LoneLeadingSurrogateInHexEscape);
}

#[test]
fn test_parse_unicode_escape_valid_single() {
    struct MockReader {
        index: usize,
        data: Vec<u8>,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { index: 0, data }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(value as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }
        
        fn discard(&mut self) {}
    }

    impl Read<'_> for MockReader {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.peek()?.ok_or(Error::new(ErrorCode::EofWhileParsingString))
        }
    }

    let mut reader = MockReader::new(vec![0xD8, 0x00, b'\\', b'u', 0xD2, 0x98]);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, vec![0xED, 0x9E, 0x98]); // Expecting UTF-8 representation of U+D800 (leading surrogate) followed by another value.
}

