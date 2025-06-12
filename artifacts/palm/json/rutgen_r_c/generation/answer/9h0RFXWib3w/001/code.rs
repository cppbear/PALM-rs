// Answer 0

#[test]
fn test_peek_error_with_expected_colon() {
    struct MockRead {
        position: Position,
    }

    impl MockRead {
        fn new(line: usize, column: usize) -> Self {
            MockRead {
                position: Position { line, column },
            }
        }
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) { unimplemented!() }
    }

    let mut mock_read = MockRead::new(10, 5);
    let deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let error = deserializer.peek_error(ErrorCode::ExpectedColon);
    assert_eq!(error.err.line, 10);
    assert_eq!(error.err.column, 5);
}

#[test]
fn test_peek_error_with_end_of_file() {
    struct MockRead {
        position: Position,
    }

    impl MockRead {
        fn new(line: usize, column: usize) -> Self {
            MockRead {
                position: Position { line, column },
            }
        }
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) { unimplemented!() }
    }

    let mut mock_read = MockRead::new(1, 1);
    let deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let error = deserializer.peek_error(ErrorCode::EofWhileParsingValue);
    assert_eq!(error.err.line, 1);
    assert_eq!(error.err.column, 1);
}

