// Answer 0

#[test]
fn test_parse_escape_invalid_escape() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
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

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // Not tested here, but necessary to satisfy the trait
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut mock_reader = MockRead { data: vec![b'\\', b'x'], position: 0 }; // invalid escape case
    let result = parse_escape(&mut mock_reader, true, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code(), ErrorCode::InvalidEscape);
}

#[test]
fn test_parse_escape_eof_before_read() {
    struct EmptyRead;

    impl<'de> Read<'de> for EmptyRead {
        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None) // simulating EOF
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0)
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            Err(Error::from(ErrorCode::EofWhileParsingString))
        }
    }

    let mut scratch = Vec::new();
    let mut empty_reader = EmptyRead;
    let result = parse_escape(&mut empty_reader, true, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code(), ErrorCode::EofWhileParsingString);
}

