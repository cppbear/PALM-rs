// Answer 0

fn test_deserialize_struct_valid_map() {
    struct MockDeserializer {
        scratch: Vec<u8>,
        remaining_depth: u8,
    }

    impl<'de> Read<'de> for MockDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }
        
        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::new("", 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
    }

    impl<'de> de::Visitor<'de> for () {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = MockDeserializer {
        scratch: vec![],
        remaining_depth: 1,
    };

    let result: Result<()> = deserializer.deserialize_struct("test", &[], ());

    assert!(result.is_ok());
}

fn test_deserialize_struct_eof_error() {
    struct MockDeserializer {
        scratch: Vec<u8>,
        remaining_depth: u8,
    }

    impl<'de> Read<'de> for MockDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }
        
        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::EofWhileParsingValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
    }

    impl<'de> de::Visitor<'de> for () {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = MockDeserializer {
        scratch: vec![],
        remaining_depth: 1,
    };

    let result: Result<()> = deserializer.deserialize_struct("test", &[], ());

    assert!(result.is_err());
}

fn test_deserialize_struct_exceed_recursion_limit() {
    struct MockDeserializer {
        scratch: Vec<u8>,
        remaining_depth: u8,
    }

    impl<'de> Read<'de> for MockDeserializer {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
        }
        
        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::new("", 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
    }

    impl<'de> de::Visitor<'de> for () {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
        
        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let mut deserializer = MockDeserializer {
        scratch: vec![],
        remaining_depth: 0,
    };

    let result: Result<()> = deserializer.deserialize_struct("test", &[], ());

    assert!(result.is_err());
}

