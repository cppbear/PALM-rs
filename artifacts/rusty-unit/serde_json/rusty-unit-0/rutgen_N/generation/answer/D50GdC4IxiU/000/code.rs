// Answer 0

#[test]
fn test_parse_unicode_escape_simple() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }
    }

    impl Read<'_> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            let hex_str = std::str::from_utf8(&self.input[self.position..self.position + 4]).unwrap();
            self.position += 4;
            u32::from_str_radix(hex_str, 16).map_err(|_| ErrorCode::InvalidHexEscape)
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(ErrorCode::EndOfInput)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader::new(b"0041".to_vec()); // Represents unicode 'A'
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, b"A");
}

#[test]
fn test_parse_unicode_escape_with_surrogates() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }
    }

    impl Read<'_> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            let hex_str = std::str::from_utf8(&self.input[self.position..self.position + 4]).unwrap();
            self.position += 4;
            u32::from_str_radix(hex_str, 16).map_err(|_| ErrorCode::InvalidHexEscape)
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(ErrorCode::EndOfInput)
            }
        }
    }

    let mut scratch = Vec::new();
    let input_data = b"uD83DuDE00uD83DuDE03";
    let mut reader = MockReader::new(input_data.to_vec());
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorCode::LoneLeadingSurrogateInHexEscape);
}

#[test]
fn test_parse_unicode_escape_unexpected_end() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }
    }

    impl Read<'_> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            let hex_str = std::str::from_utf8(&self.input[self.position..self.position + 4]).unwrap();
            self.position += 4;
            u32::from_str_radix(hex_str, 16).map_err(|_| ErrorCode::InvalidHexEscape)
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(ErrorCode::EndOfInput)
            }
        }
    }

    let mut scratch = Vec::new();
    let input_data = b"uD83D";
    let mut reader = MockReader::new(input_data.to_vec());
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorCode::UnexpectedEndOfHexEscape);
}

