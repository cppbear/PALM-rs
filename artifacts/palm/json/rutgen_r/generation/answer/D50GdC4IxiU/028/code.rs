// Answer 0

#[test]
fn test_parse_unicode_escape_valid_bmp() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.input.len() {
                let hex_str = &self.input[self.position..self.position + 4];
                self.position += 4;
                Ok(u32::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16).unwrap())
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
        
        fn discard(&mut self) {}
        
        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0) // Simulate EOF
            }
        }
    }
    
    let mut scratch = Vec::new();
    let mut reader = MockReader { input: b"\\uD800\\uDC00".to_vec(), position: 0 };
    
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![0xF0, 0x90, 0x80, 0x80]); // U+10000 -> UTF-8 encoding
}

#[test]
fn test_parse_unicode_escape_lone_surrogate() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.input.len() {
                let hex_str = &self.input[self.position..self.position + 4];
                self.position += 4;
                Ok(u32::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16).unwrap())
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
        
        fn discard(&mut self) {}
        
        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0) // Simulate EOF
            }
        }
    }
    
    let mut scratch = Vec::new();
    let mut reader = MockReader { input: b"\\uD800".to_vec(), position: 0 };
    
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err()); // This should trigger an error for a lone surrogate
}

#[test]
fn test_parse_unicode_escape_unexpected_end() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.position < self.input.len() {
                let hex_str = &self.input[self.position..self.position + 4];
                self.position += 4;
                Ok(u32::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16).unwrap())
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }
        
        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Ok(0) // Simulate EOF
            }
        }
    }
    
    let mut scratch = Vec::new();
    let mut reader = MockReader { input: b"\\uD800\\".to_vec(), position: 0 };
    
    let result = parse_unicode_escape(&mut reader, true, &mut scratch);
    assert!(result.is_err()); // This should trigger an error for unexpected end of escape sequence
}

