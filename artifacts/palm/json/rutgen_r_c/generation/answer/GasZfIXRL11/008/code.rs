// Answer 0

fn test_peek_invalid_type_string() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            scratch.extend_from_slice(&self.data[self.position..self.data.len()]);
            self.position = self.data.len();
            Ok(Reference::Borrowed(std::str::from_utf8(scratch).unwrap()))
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            scratch.extend_from_slice(&self.data[self.position..self.data.len()]);
            self.position = self.data.len();
            Ok(Reference::Borrowed(scratch.as_slice()))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::InvalidEscape, 0, 0))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut scratch = Vec::new();
    let read = TestRead { data: b"\"test_string\"".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch, remaining_depth: 128 };

    let exp = &Expected::Str("expected");
    let result = deserializer.peek_invalid_type(exp);
    
    assert!(matches!(result, Error::InvalidType(ref u) if u == &Unexpected::Str(&"test_string")));
}

fn test_peek_invalid_type_bool_true() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            scratch.extend_from_slice(&self.data[self.position..self.data.len()]);
            self.position = self.data.len();
            Ok(Reference::Borrowed(std::str::from_utf8(scratch).unwrap()))
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            scratch.extend_from_slice(&self.data[self.position..self.data.len()]);
            self.position = self.data.len();
            Ok(Reference::Borrowed(scratch.as_slice()))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::InvalidEscape, 0, 0))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut scratch = Vec::new();
    let read = TestRead { data: b"true".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch, remaining_depth: 128 };

    let exp = &Expected::Bool(true);
    let result = deserializer.peek_invalid_type(exp);
    
    assert!(matches!(result, Error::InvalidType(ref u) if u == &Unexpected::Bool(true)));
}

fn test_peek_invalid_type_bool_false() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            scratch.extend_from_slice(&self.data[self.position..self.data.len()]);
            self.position = self.data.len();
            Ok(Reference::Borrowed(std::str::from_utf8(scratch).unwrap()))
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            scratch.extend_from_slice(&self.data[self.position..self.data.len()]);
            self.position = self.data.len();
            Ok(Reference::Borrowed(scratch.as_slice()))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::InvalidEscape, 0, 0))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut scratch = Vec::new();
    let read = TestRead { data: b"false".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch, remaining_depth: 128 };

    let exp = &Expected::Bool(false);
    let result = deserializer.peek_invalid_type(exp);
    
    assert!(matches!(result, Error::InvalidType(ref u) if u == &Unexpected::Bool(false)));
}

