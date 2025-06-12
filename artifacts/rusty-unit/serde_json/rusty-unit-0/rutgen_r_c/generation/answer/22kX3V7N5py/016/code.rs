// Answer 0

fn test_parse_any_signed_number_positive() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"123".to_vec(), position: 0 },
        scratch: vec![],
        remaining_depth: 10,
    };
    
    let result = deserializer.parse_any_signed_number();
    assert!(result.is_ok());
    match result {
        Ok(ParserNumber::U64(value)) => assert_eq!(value, 123),
        _ => panic!("Expected value to be ParserNumber::U64"),
    }
}

fn test_parse_any_signed_number_negative() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"-456".to_vec(), position: 0 },
        scratch: vec![],
        remaining_depth: 10,
    };
    
    let result = deserializer.parse_any_signed_number();
    assert!(result.is_ok());
    match result {
        Ok(ParserNumber::I64(value)) => assert_eq!(value, -456),
        _ => panic!("Expected value to be ParserNumber::I64"),
    }
}

fn test_parse_any_signed_number_invalid() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"abc".to_vec(), position: 0 },
        scratch: vec![],
        remaining_depth: 10,
    };
    
    let result = deserializer.parse_any_signed_number();
    assert!(result.is_err());
}

