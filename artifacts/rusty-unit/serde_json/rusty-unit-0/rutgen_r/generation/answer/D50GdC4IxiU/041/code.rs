// Answer 0

#[test]
fn test_parse_unicode_escape_valid() {
    use std::io::Cursor;
    use serde_json::ErrorCode;

    struct MockReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> MockReader<'a> {
        fn new(data: &'a [u8]) -> Self {
            Self { data, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, ErrorCode> {
            if self.position >= self.data.len() {
                return Err(ErrorCode::UnexpectedEndOfHexEscape);
            }
            
            let byte = self.data[self.position];
            self.position += 1;
            Ok(byte as u32)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8, ErrorCode> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }
    }

    let mut scratch = Vec::new();
    let input_data = vec![0xD8, 0x00, b'\\', b'u', 0xDC, 0x00]; // Example input for testing
    let mut reader = MockReader::new(&input_data);
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);

    assert!(result.is_err()); // Should trigger an error due to lone leading surrogate
}

#[test]
fn test_parse_unicode_escape_without_validation() {
    use std::io::Cursor;

    struct MockReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> MockReader<'a> {
        fn new(data: &'a [u8]) -> Self {
            Self { data, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, ErrorCode> {
            if self.position >= self.data.len() {
                return Err(ErrorCode::UnexpectedEndOfHexEscape);
            }
            
            let byte = self.data[self.position];
            self.position += 1;
            Ok(byte as u32)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8, ErrorCode> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }
    }

    let mut scratch = Vec::new();
    let input_data = vec![0xD8, 0x00, b'\\', b'u', 0xDC, 0x00]; // Example input for testing
    let mut reader = MockReader::new(&input_data);
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);

    assert!(result.is_ok()); // Should not trigger an error
    assert!(!scratch.is_empty()); // Scratch should contain the processed value
}

