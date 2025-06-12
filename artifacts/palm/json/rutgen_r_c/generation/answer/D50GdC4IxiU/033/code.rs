// Answer 0

#[test]
fn test_parse_unicode_escape_lone_surrogate() {
    struct TestReader {
        pos: usize,
        data: Vec<u8>,
    }

    impl<'de> Read<'de> for TestReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Ok(val.into())
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn discard(&mut self) {}

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Ok(val)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader {
        pos: 0,
        data: vec![0xD8, 0x00, 0xDB, 0xFF], // Values to trigger edges of the bounds
    };
    
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    
    // Test to ensure we can check the result when it goes beyond the surrogate range
    let invalid_result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(invalid_result.is_err());
    match invalid_result {
        Err(err) => assert_eq!(err.code, ErrorCode::LoneLeadingSurrogateInHexEscape),
        _ => unreachable!(),
    }
}

