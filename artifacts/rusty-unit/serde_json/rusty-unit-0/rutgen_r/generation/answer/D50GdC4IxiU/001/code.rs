// Answer 0

#[test]
fn test_parse_unicode_escape_invalid_input() {
    struct MockReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            MockReader { bytes, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            Err(ErrorCode::InvalidHexEscape)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.bytes.len() {
                Ok(self.bytes[self.position])
            } else {
                Err(ErrorCode::UnexpectedEnd)
            }
        }
    }

    impl Read<'_> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            self.decode_hex_escape()
        }

        fn discard(&mut self) {
            self.discard();
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.peek_or_eof()
        }
    }

    let mut reader = MockReader::new(vec![b'\\', b'u']);
    let mut scratch: Vec<u8> = Vec::new();
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
}

#[test]
fn test_parse_unicode_escape_unexpected_end_of_hex_escape() {
    struct MockReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            MockReader { bytes, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.bytes.len() {
                self.position += 1;
                Ok(0xD800) // Return a leading surrogate.
            } else {
                Err(ErrorCode::UnexpectedEnd)
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.bytes.len() {
                Ok(self.bytes[self.position])
            } else {
                Err(ErrorCode::UnexpectedEnd)
            }
        }
    }

    impl Read<'_> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            self.decode_hex_escape()
        }

        fn discard(&mut self) {
            self.discard();
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            self.peek_or_eof()
        }
    }

    let mut reader = MockReader::new(vec![b'\\', b'u', b'D', b'8', b'0']); // Incomplete escape
    let mut scratch: Vec<u8> = Vec::new();
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
}

