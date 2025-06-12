// Answer 0

#[test]
fn test_parse_unicode_escape_with_surrogates() {
    use std::io::{Cursor, Read};
    use serde_json::error::{ErrorCode, error};
    use serde_json::push_wtf8_codepoint;
    use serde_json::Result;

    struct MockRead {
        buffer: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(buffer: Vec<u8>) -> Self {
            MockRead { buffer, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position >= self.buffer.len() {
                return Err(error(&mut self.buffer[..], ErrorCode::UnexpectedEndOfHexEscape));
            }

            let hex = self.buffer[self.position];
            self.position += 1;

            if hex >= b'0' && hex <= b'9' {
                Ok((hex - b'0') as u32)
            } else if hex >= b'A' && hex <= b'F' {
                Ok((hex - b'A' + 10) as u32)
            } else {
                Err(error(&mut self.buffer[..], ErrorCode::UnexpectedEndOfHexEscape))
            }
        }

        fn discard(&mut self) {
            if self.position < self.buffer.len() {
                self.position += 1;
            }
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.buffer.len() {
                Ok(self.buffer[self.position])
            } else {
                Err(error(&mut self.buffer[..], ErrorCode::UnexpectedEndOfHexEscape))
            }
        }
    }

    let mut scratch = Vec::new();
    let input_data: Vec<u8> = vec![b'\\', b'u', 0xD8, 0x00, b'\\', b'u', 0xDC, 0x00]; // \uD800\uDC00

    let mut read = MockRead::new(input_data);

    let result = parse_unicode_escape(&mut read, true, &mut scratch);

    assert!(result.is_ok());
    assert!(!scratch.is_empty());
}

#[test]
fn test_parse_unicode_escape_with_lone_leading_surrogate() {
    use std::io::{Cursor, Read};
    use serde_json::error::{ErrorCode, error};
    use serde_json::push_wtf8_codepoint;
    use serde_json::Result;

    struct MockRead {
        buffer: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(buffer: Vec<u8>) -> Self {
            MockRead { buffer, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position >= self.buffer.len() {
                return Err(error(&mut self.buffer[..], ErrorCode::UnexpectedEndOfHexEscape));
            }

            let hex = self.buffer[self.position];
            self.position += 1;

            if hex >= b'0' && hex <= b'9' {
                Ok((hex - b'0') as u32)
            } else if hex >= b'A' && hex <= b'F' {
                Ok((hex - b'A' + 10) as u32)
            } else {
                Err(error(&mut self.buffer[..], ErrorCode::UnexpectedEndOfHexEscape))
            }
        }

        fn discard(&mut self) {
            if self.position < self.buffer.len() {
                self.position += 1;
            }
        }

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.buffer.len() {
                Ok(self.buffer[self.position])
            } else {
                Err(error(&mut self.buffer[..], ErrorCode::UnexpectedEndOfHexEscape))
            }
        }
    }

    let mut scratch = Vec::new();
    let input_data: Vec<u8> = vec![b'\\', b'u', 0xD8, 0x00]; // \uD800 (lone leading surrogate)

    let mut read = MockRead::new(input_data);

    let result = parse_unicode_escape(&mut read, true, &mut scratch);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code(), ErrorCode::LoneLeadingSurrogateInHexEscape);
}

