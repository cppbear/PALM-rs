// Answer 0

#[test]
fn test_ignore_integer_with_leading_zero() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut reader = MockReader {
        input: vec![b'0', b'1'], // Input that should lead to an InvalidNumber error
        position: 0,
    };

    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_with_multiple_leading_zero() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut reader = MockReader {
        input: vec![b'0', b'0'], // Input that should lead to an InvalidNumber error
        position: 0,
    };

    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_with_valid_input() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'_> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut reader = MockReader {
        input: vec![b'1', b'9', b'0'], // Valid input leading to successful parsing
        position: 0,
    };

    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    let _ = deserializer.ignore_integer();
}

