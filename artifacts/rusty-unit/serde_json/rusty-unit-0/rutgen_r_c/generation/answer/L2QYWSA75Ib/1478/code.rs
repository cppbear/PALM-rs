// Answer 0

#[test]
fn test_ignore_value_empty_input() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position >= self.input.len() {
                Ok(None)
            } else {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.position]))
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock = MockRead { input: vec![], position: 0 };
    let mut deserializer = Deserializer { read: &mut mock, scratch: vec![], remaining_depth: 0 };
    
    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_value_with_null() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position >= self.input.len() {
                Ok(None)
            } else {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.position]))
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock = MockRead { input: b"null".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: &mut mock, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_value_unexpected_end() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position >= self.input.len() {
                Ok(None)
            } else {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.position]))
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock = MockRead { input: b"[1, 2, ".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: &mut mock, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.ignore_value();
    assert!(result.is_err());
}

#[test]
fn test_ignore_value_with_invalid_key() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position >= self.input.len() {
                Ok(None)
            } else {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.position]))
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock = MockRead { input: b"{:}".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: &mut mock, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.ignore_value();
    assert!(result.is_err());
}

