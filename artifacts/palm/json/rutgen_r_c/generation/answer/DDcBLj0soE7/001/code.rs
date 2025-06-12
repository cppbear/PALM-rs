// Answer 0

#[test]
fn test_error_with_expected_syntax() {
    struct MockRead {
        position: Position,
    }
    
    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'b'))
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

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let position = Position { line: 1, column: 1 };
    let mock_read = MockRead { position };
    let mut deserializer = Deserializer { read: mock_read, scratch: vec![], remaining_depth: 5 };

    let error_code = ErrorCode::ExpectedSomeValue;
    let error = deserializer.error(error_code);
    
    assert_eq!(error.err.code, ErrorCode::ExpectedSomeValue);
    assert_eq!(error.err.line, 1);
    assert_eq!(error.err.column, 1);
}

#[test]
fn test_error_with_default_position() {
    struct MockRead {
        position: Position,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'b'))
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

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mock_read = MockRead { position: Position { line: 0, column: 0 } };
    let mut deserializer = Deserializer { read: mock_read, scratch: vec![], remaining_depth: 5 };

    let error_code = ErrorCode::Io(io::Error::new(io::ErrorKind::Other, "mock error"));
    let error = deserializer.error(error_code);

    assert_eq!(error.err.code, ErrorCode::Io(io::Error::new(io::ErrorKind::Other, "mock error")));
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

