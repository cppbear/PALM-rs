// Answer 0

#[test]
fn test_parse_unicode_escape_invalid_lone_surrogate() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::ErrorCode;
    use serde_json::Result;

    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: &[u8]) -> Self {
            TestReader {
                input: input.to_vec(),
                position: 0,
            }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u32> {
            // Simulate returning a value that matches the constraints,
            // Valid hexadecimal character that creates a leading surrogate
            if self.position == 0 {
                self.position += 1;
                return Ok(0xD800); // n == 0xD800
            }
            Err(ErrorCode::UnexpectedEndOfHexEscape)
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position >= self.input.len() {
                return Err(ErrorCode::UnexpectedEndOfHexEscape);
            }
            Ok(self.input[self.position])
        }
        
        fn discard(&mut self) {
            // Nothing special to do here for tests
        }
    }

    let mut scratch = Vec::new();
    let input = b"\\uD800\\u"; // input sequence for the test
    let mut reader = TestReader::new(input);

    // We expect an error due to invalid trailing surrogate as `validate` is false
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorCode::UnexpectedEndOfHexEscape);
}

