// Answer 0

fn test_deserialize_bool_true() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;
        
        fn visit_bool(self, value: bool) -> Result<bool> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    struct ReadMock;
    
    impl<'de> Read<'de> for ReadMock {
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
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: ReadMock,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.deserialize_bool(Visitor);
    assert_eq!(result, Ok(true));
}

fn test_deserialize_bool_false() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<bool> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    struct ReadMock;

    impl<'de> Read<'de> for ReadMock {
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
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: ReadMock,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.deserialize_bool(Visitor);
    assert_eq!(result, Ok(false));
}

fn test_deserialize_bool_eof() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<bool> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    struct ReadMock;

    impl<'de> Read<'de> for ReadMock {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: ReadMock,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.deserialize_bool(Visitor);
    assert!(result.is_err());
}

