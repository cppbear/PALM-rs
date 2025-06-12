// Answer 0

#[test]
fn test_parse_unicode_escape_valid_non_bmp() {
    use std::io::Cursor;
    use serde_json::ErrorCode;
    use serde_json::{Result, Read};

    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: &[u8]) -> Self {
            Self {
                data: data.to_vec(),
                position: 0,
            }
        }
        
        fn peek_or_eof(&self) -> Result<u8> {
            if self.position >= self.data.len() {
                Ok(0)
            } else {
                Ok(self.data[self.position])
            }
        }
        
        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let hex_str = &self.data[self.position..self.position + 4];
            self.position += 4; // Simulate reading 4 bytes for a hex escape
            let value = u16::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16).unwrap();
            Ok(value)
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"\\uD800\\uDC00"); // Input representing a valid surrogate pair

    let result: Result<()> = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![0xF0, 0x9D, 0x9C, 0x80]); // Expected UTF-8 encoding of U+10000
}

#[test]
#[should_panic]
fn test_parse_unicode_escape_lone_leading_surrogate() {
    use std::io::Cursor;
    use serde_json::{ErrorCode, Result};

    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: &[u8]) -> Self {
            Self {
                data: data.to_vec(),
                position: 0,
            }
        }
        
        fn peek_or_eof(&self) -> Result<u8> {
            if self.position >= self.data.len() {
                Ok(0)
            } else {
                Ok(self.data[self.position])
            }
        }
        
        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let hex_str = &self.data[self.position..self.position + 4];
            self.position += 4; // Simulate reading 4 bytes for a hex escape
            let value = u16::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16).unwrap();
            Ok(value)
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"\\uD800X"); // Invalid leading surrogate with a trailing character

    let result: Result<()> = parse_unicode_escape(&mut read, true, &mut scratch);
    // This should panic due to the lone leading surrogate condition
}

#[test]
fn test_parse_unicode_escape_invalid_trailing_surrogate() {
    use std::io::Cursor;
    use serde_json::{ErrorCode, Result};

    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: &[u8]) -> Self {
            Self {
                data: data.to_vec(),
                position: 0,
            }
        }
        
        fn peek_or_eof(&self) -> Result<u8> {
            if self.position >= self.data.len() {
                Ok(0)
            } else {
                Ok(self.data[self.position])
            }
        }
        
        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let hex_str = &self.data[self.position..self.position + 4];
            self.position += 4; // Simulate reading 4 bytes for a hex escape
            let value = u16::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16).unwrap();
            Ok(value)
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"\\uD800\\uD800"); // Invalid second surrogate, should trigger validation

    let result: Result<()> = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_err()); // Expecting an error due to validation
}

