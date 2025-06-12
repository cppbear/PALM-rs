// Answer 0

#[test]
fn test_ignore_value_with_valid_json_true() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

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

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }
        
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_read = MockRead {
        input: b"true".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    assert!(deserializer.ignore_value().is_ok());
}

#[test]
fn test_ignore_value_with_valid_json_false() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

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

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }
        
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_read = MockRead {
        input: b"false".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    assert!(deserializer.ignore_value().is_ok());
}

#[test]
fn test_ignore_value_with_valid_json_null() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

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

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }
        
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_read = MockRead {
        input: b"null".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    assert!(deserializer.ignore_value().is_ok());
}

#[test]
fn test_ignore_value_with_invalid_json() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

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

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }
        
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_read = MockRead {
        input: b"invalid".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    assert!(deserializer.ignore_value().is_err());
}

