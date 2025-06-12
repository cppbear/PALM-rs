// Answer 0

fn test_ignore_value_valid_true() -> Result<()> {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
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

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::new(""))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::new(&[]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let reader = TestReader {
        data: b"null, true, false, -123, 456".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    assert!(deserializer.ignore_value().is_ok());
    Ok(())
}

fn test_ignore_value_unexpected_token() -> Result<()> {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
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

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::new(""))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::new(&[]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let reader = TestReader {
        data: b"!invalid_token".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_err());
    match result {
        Err(err) => assert_eq!(err, deserializer.peek_error(ErrorCode::ExpectedSomeValue)),
        _ => unreachable!(),
    }
    Ok(())
}

fn test_ignore_value_empty() -> Result<()> {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
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

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::new(""))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::new(&[]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let reader = TestReader {
        data: b"".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_err());
    match result {
        Err(err) => assert_eq!(err, deserializer.peek_error(ErrorCode::EofWhileParsingValue)),
        _ => unreachable!(),
    }
    Ok(())
}

fn test_ignore_value_unexpected_number() -> Result<()> {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestReader {
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

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::new(""))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::new(&[]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let reader = TestReader {
        data: b"123.45".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_err());
    match result {
        Err(err) => assert_eq!(err, deserializer.peek_error(ErrorCode::ExpectedSomeValue)),
        _ => unreachable!(),
    }
    Ok(())
}

