// Answer 0

#[test]
fn test_peek_error_valid_position() {
    struct MockRead {
        position: Position,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            self.position
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {
            *failed = true;
        }
    }

    let mock_read = MockRead {
        position: Position { line: 2, column: 3 },
    };

    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let error = deserializer.peek_error(ErrorCode::EofWhileParsingValue);
    assert_eq!(error.err.line, 2);
    assert_eq!(error.err.column, 3);
}

#[test]
fn test_peek_error_initial_position() {
    struct MockRead {
        position: Position,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            self.position
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {
            *failed = true;
        }
    }

    let mock_read = MockRead {
        position: Position { line: 0, column: 0 },
    };

    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let error = deserializer.peek_error(ErrorCode::ExpectedColon);
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

