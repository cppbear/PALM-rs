// Answer 0

fn test_parse_exponent_valid() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
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
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
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

    let mock_reader = MockReader::new(b"e+10".to_vec());
    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer.parse_exponent(true, 100, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 100e10);
}

fn test_parse_exponent_with_negative_exponent() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
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
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
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

    let mock_reader = MockReader::new(b"e-10".to_vec());
    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer.parse_exponent(true, 100, 0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 100e-10);
}

fn test_parse_exponent_invalid_character() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
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
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
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

    let mock_reader = MockReader::new(b"e!10".to_vec());
    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer.parse_exponent(true, 100, 0);
    assert!(result.is_err());
}

