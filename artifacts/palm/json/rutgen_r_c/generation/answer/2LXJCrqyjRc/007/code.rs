// Answer 0

#[test]
fn test_do_deserialize_i128_valid_input() {
    struct MockVisitor;
    
    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::new(0, 0) }
        fn peek_position(&self) -> Position { Position::new(0, 0) }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Err(Error::syntax(ErrorCode::Message("Mock".into()), 0, 0)) }
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: vec![],
        remaining_depth: 128,
    };

    assert_eq!(deserializer.do_deserialize_i128(MockVisitor), Ok(0));
}

#[test]
#[should_panic(expected = "NumberOutOfRange")]
fn test_do_deserialize_i128_parse_error() {
    struct MockVisitor;

    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(Some(b'0')) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b'0')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: 0 } }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Err(Error::syntax(ErrorCode::Message("Mock".into()), 0, 0)) }
    }
    
    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: vec![],
        remaining_depth: 128,
    };

    deserializer.do_deserialize_i128(MockVisitor);
}

#[test]
fn test_do_deserialize_i128_eof_error() {
    struct MockVisitor;

    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = i128;

        fn visit_i128(self, _value: i128) -> Result<Self::Value> {
            Ok(0)
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::new(0, 0) }
        fn peek_position(&self) -> Position { Position::new(0, 0) }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Err(Error::syntax(ErrorCode::Message("Mock".into()), 0, 0)) }
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: vec![],
        remaining_depth: 128,
    };

    assert_eq!(deserializer.do_deserialize_i128(MockVisitor), Err(ErrorCode::EofWhileParsingValue));
}

