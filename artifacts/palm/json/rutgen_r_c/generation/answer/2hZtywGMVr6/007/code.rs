// Answer 0

fn test_deserialize_map_success() {
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = ();
        fn visit_map<M>(self, _map: M) -> Result<Self::Value>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct MockRead;
    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(Some(b'{')) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b'{')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::new(0, 0) }
        fn peek_position(&self) -> Position { Position::new(0, 0) }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<()> = deserializer.deserialize_map(Visitor);
    assert!(result.is_ok());
}

fn test_deserialize_map_eof() {
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = ();
        fn visit_map<M>(self, _map: M) -> Result<Self::Value>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct MockRead;
    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::new(0, 0) }
        fn peek_position(&self) -> Position { Position::new(0, 0) }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<()> = deserializer.deserialize_map(Visitor);
    assert!(result.is_err());
}

fn test_deserialize_map_invalid_type() {
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = ();
        fn visit_map<M>(self, _map: M) -> Result<Self::Value>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct MockRead;
    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { Ok(Some(b'}')) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(Some(b'}')) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::new(0, 0) }
        fn peek_position(&self) -> Position { Position::new(0, 0) }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result: Result<()> = deserializer.deserialize_map(Visitor);
    assert!(result.is_err());
}

