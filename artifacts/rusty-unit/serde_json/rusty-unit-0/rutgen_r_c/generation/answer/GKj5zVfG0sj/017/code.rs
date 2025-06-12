// Answer 0

fn test_deserialize_struct_ok() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();
        
        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }

        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestRead;

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'{'))
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
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    
    let result: Result<()> = deserializer.deserialize_struct("TestStruct", &[], Visitor);
    assert!(result.is_ok());
}

fn test_deserialize_struct_eof() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();
        
        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }

        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestRead;

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
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
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestRead,
        scratch: Vec::new(),
        remaining_depth: 1,
    };
    
    let result: Result<()> = deserializer.deserialize_struct("TestStruct", &[], Visitor);
    assert!(result.is_err());
}

