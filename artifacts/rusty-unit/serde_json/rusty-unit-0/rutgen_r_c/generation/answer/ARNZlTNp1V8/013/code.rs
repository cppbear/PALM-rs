// Answer 0

fn test_deserialize_any_unit() {
    struct TestReader;
    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'n'))
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'n'))
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestReader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<()> = deserializer.deserialize_any(UnitVisitor);
    assert_eq!(result, Ok(()));
}

fn test_deserialize_any_bool_true() {
    struct TestReader;
    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b't'))
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b't'))
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestReader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<bool> = deserializer.deserialize_any(BoolVisitor);
    assert_eq!(result, Ok(true));
}

fn test_deserialize_any_bool_false() {
    struct TestReader;
    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'f'))
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'f'))
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestReader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<bool> = deserializer.deserialize_any(BoolVisitor);
    assert_eq!(result, Ok(false));
}

fn test_deserialize_any_number() {
    struct TestReader;
    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'1'))
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'1'))
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestReader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<u64> = deserializer.deserialize_any(NumberVisitor);
    assert_eq!(result, Ok(1));
}

fn test_deserialize_any_string() {
    struct TestReader;
    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'"'))
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            let result = "test string".to_string();
            Ok(Reference::Borrowed(result.as_str()))
        }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestReader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<&str> = deserializer.deserialize_any(StringVisitor);
    assert_eq!(result, Ok("test string"));
}

fn test_deserialize_any_seq() {
    struct TestReader;
    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'['))
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'['))
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestReader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<Vec<u64>> = deserializer.deserialize_any(SeqVisitor);
    assert!(result.is_err());
}

fn test_deserialize_any_map() {
    struct TestReader;
    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestReader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<HashMap<String, String>> = deserializer.deserialize_any(MapVisitor);
    assert!(result.is_err());
}

