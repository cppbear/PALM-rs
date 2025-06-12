// Answer 0

fn test_parse_decimal_success() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockReader {
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
            Position { line: 0, column: self.position } // Simplified for testing
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position } // Simplified for testing
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let data = b"1234";
    let mut reader = MockReader { data: data.to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 128 };

    let result = deserializer.parse_decimal(true, 123, 0).unwrap();
    assert_eq!(result, 1234.0);
}

fn test_parse_decimal_overflow() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockReader {
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
            Position { line: 0, column: self.position } // Simplified for testing
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position } // Simplified for testing
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let data = b"999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999990";
    let mut reader = MockReader { data: data.to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 128 };

    let result = deserializer.parse_decimal(true, u64::MAX, 0);
    assert!(result.is_err());
}

fn test_parse_decimal_invalid_number() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockReader {
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
            Position { line: 0, column: self.position } // Simplified for testing
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position } // Simplified for testing
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let data = b"123.";
    let mut reader = MockReader { data: data.to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 128 };

    let result = deserializer.parse_decimal(true, 123, 0);
    assert!(result.is_err());
}

