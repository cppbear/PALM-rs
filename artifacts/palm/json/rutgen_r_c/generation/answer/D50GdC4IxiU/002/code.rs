// Answer 0

#[test]
fn test_parse_unicode_escape_lone_leading_surrogate() {
    struct MockRead {
        index: usize,
        hex_data: Vec<u8>,
    }

    impl MockRead {
        fn new(hex_data: Vec<u8>) -> Self {
            MockRead { index: 0, hex_data }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.hex_data.len() {
                let val = self.hex_data[self.index];
                self.index += 1;
                Ok(val as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.hex_data.len() {
                Ok(Some(self.hex_data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            // Mock discard just moves the index forward
            self.index += 1;
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xDC, 0x00]); // initial valid hex [0xDC00] leading surrogate

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert_eq!(result.is_err(), true);
    assert_eq!(result.unwrap_err().code(), ErrorCode::LoneLeadingSurrogateInHexEscape);
}

#[test]
fn test_parse_unicode_escape_valid_codepoint() {
    struct MockRead {
        index: usize,
        hex_data: Vec<u8>,
    }

    impl MockRead {
        fn new(hex_data: Vec<u8>) -> Self {
            MockRead { index: 0, hex_data }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.hex_data.len() {
                let val = self.hex_data[self.index];
                self.index += 1;
                Ok(val as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.hex_data.len() {
                Ok(Some(self.hex_data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xD8, 0x00, 0xDC, 0x00]); // valid surrogate pair [0xD800] leading and [0xDC00] trailing

    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert_eq!(result.is_ok(), true);
    assert_eq!(scratch.len(), 4); // Check that scratch has expected length after parsing two surrogates
}

