// Answer 0

#[test]
fn test_error_position() {
    struct DummyRead {
        position: Position,
    }

    impl<'de> Read<'de> for DummyRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let read = DummyRead { position: Position { line: 5, column: 10 } };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };

    let error = deserializer.error(ErrorCode::EofWhileParsingValue);
    assert_eq!(error.err.line, 5);
    assert_eq!(error.err.column, 10);
}

