// Answer 0

#[test]
fn test_parse_unicode_escape_lone_leading_surrogate() {
    struct MockRead {
        hex_escape: Vec<u16>,
        index: usize,
    }

    impl MockRead {
        fn new(hex_escape: Vec<u16>) -> Self {
            Self { hex_escape, index: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.hex_escape.len() {
                let val = self.hex_escape[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            Ok(b'\\') // simulate the condition where it's expecting a backslash
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xDC00, 0xDC00]); // leading surrogate case

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_err());
}

#[test]
fn test_parse_unicode_escape_valid_escape() {
    struct MockRead {
        hex_escape: Vec<u16>,
        index: usize,
    }

    impl MockRead {
        fn new(hex_escape: Vec<u16>) -> Self {
            Self { hex_escape, index: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u32> {
            if self.index < self.hex_escape.len() {
                let val = self.hex_escape[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err(ErrorCode::UnexpectedEndOfHexEscape.into())
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            Ok(b'u') // simulate the condition where it gets a valid 'u' after '\'
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD800, 0xDC00]); // valid surrogate pair 

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch.len(), 4); // expect valid UTF-16 surrogate to yield 4 bytes in scratch
}

