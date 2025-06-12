// Answer 0

#[test]
fn test_parse_unicode_escape_with_valid_bmp_character() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position < self.input.len() {
                let hex_str = &self.input[self.position..self.position + 4];
                self.position += 4;
                let hex_value = u16::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16).map_err(|_| ErrorCode::InvalidHex)?;
                Ok(hex_value)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }

        fn discard(&mut self) {
            // Do nothing for mock
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape)
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader {
        input: b"\\uD800\\uDC00".to_vec(),
        position: 0,
    };

    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    assert_eq!(result, Ok(()));
}

