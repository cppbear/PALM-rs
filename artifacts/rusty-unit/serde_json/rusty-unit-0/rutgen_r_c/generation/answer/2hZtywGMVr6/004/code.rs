// Answer 0

fn test_deserialize_map_valid() {
    struct MockVisitor {
        visited: bool,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0)
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result = deserializer.deserialize_map(MockVisitor { visited: false });
    assert!(result.is_ok());
}

fn test_deserialize_map_eof_error() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            unimplemented!()
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0)
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result = deserializer.deserialize_map(MockVisitor);
    assert!(result.is_err());
}

fn test_deserialize_map_recursion_limit_exceeded() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            std::panic::panic_any("should not visit map");
        }
    }

    struct MockRead;

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0)
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.deserialize_map(MockVisitor);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.code(), ErrorCode::RecursionLimitExceeded);
    }
}

