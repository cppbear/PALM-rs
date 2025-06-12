// Answer 0

fn test_parse_integer_zero_leading() {
    struct MockReader {
        chars: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(chars: Vec<u8>) -> Self {
            MockReader { chars, index: 0 }
        }
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                let c = self.chars[self.index];
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position { todo!() }
        fn peek_position(&self) -> Position { todo!() }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { todo!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { todo!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { todo!() }
    }

    let mut reader = MockReader::new(vec![b'0', b'1', b'2']);
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 128 };

    let result = deserializer.parse_integer(true);
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().err, ErrorCode::InvalidNumber);
}

fn test_parse_integer_not_allowed_leading_zero() {
    struct MockReader {
        chars: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(chars: Vec<u8>) -> Self {
            MockReader { chars, index: 0 }
        }
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                let c = self.chars[self.index];
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position { todo!() }
        fn peek_position(&self) -> Position { todo!() }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { todo!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { todo!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { todo!() }
    }

    let mut reader = MockReader::new(vec![b'0', b'0']);
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 128 };

    let result = deserializer.parse_integer(true);
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().err, ErrorCode::InvalidNumber);
}

fn test_parse_integer_eof() {
    struct MockReader {
        index: usize,
    }

    impl MockReader {
        fn new() -> Self {
            MockReader { index: 0 }
        }
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}
        
        fn position(&self) -> Position { todo!() }
        fn peek_position(&self) -> Position { todo!() }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { todo!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { todo!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { todo!() }
    }

    let mut reader = MockReader::new();
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 128 };

    let result = deserializer.parse_integer(true);
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().err, ErrorCode::EofWhileParsingValue);
}

