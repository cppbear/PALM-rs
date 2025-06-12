// Answer 0

fn test_ignore_exponent_valid_positive() {
    struct MockRead {
        data: Vec<u8>,
        current_index: usize,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                let byte = self.data[self.current_index];
                self.current_index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                Ok(Some(self.data[self.current_index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.current_index < self.data.len() {
                self.current_index += 1;
            }
        }

        fn position(&self) -> Position {
            // Dummy implementation
            Position { line: 0, column: self.current_index as u32 }
        }

        fn peek_position(&self) -> Position {
            // Dummy implementation
            Position { line: 0, column: self.current_index as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.current_index
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: vec![b'e', b'+', b'1', b'2', b'3'], current_index: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.ignore_exponent();
    assert!(result.is_ok());
}

fn test_ignore_exponent_invalid_no_digit() {
    struct MockRead {
        data: Vec<u8>,
        current_index: usize,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                let byte = self.data[self.current_index];
                self.current_index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                Ok(Some(self.data[self.current_index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.current_index < self.data.len() {
                self.current_index += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.current_index as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.current_index as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.current_index
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: vec![b'e', b'+'], current_index: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.ignore_exponent();
    assert!(result.is_err());
}

fn test_ignore_exponent_invalid_no_sign() {
    struct MockRead {
        data: Vec<u8>,
        current_index: usize,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                let byte = self.data[self.current_index];
                self.current_index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.current_index < self.data.len() {
                Ok(Some(self.data[self.current_index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.current_index < self.data.len() {
                self.current_index += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.current_index as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.current_index as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.current_index
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: vec![b'e', b'-', b'0'], current_index: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.ignore_exponent();
    assert!(result.is_err());
}

