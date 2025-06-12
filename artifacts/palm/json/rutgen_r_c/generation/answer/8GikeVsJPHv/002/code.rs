// Answer 0

fn next_key_seed_test_case_ok() {
    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<Access>(self, _access: Access) -> Result<Self::Value>
        where
            Access: de::MapAccess<'de>,
        {
            Ok("test_key".to_string())
        }
    }

    struct TestRead {
        current_char: Option<u8>,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(self.current_char.take())
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(self.current_char)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut de = TestRead {
        current_char: Some(b'"'),
    };
    let mut map_access = MapAccess {
        de: &mut de,
        first: true,
    };

    let result = map_access.next_key_seed(TestSeed);

    assert_eq!(result.unwrap(), Some("test_key".to_string()));
}

fn next_key_seed_test_case_err_key_must_be_string() {
    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<Access>(self, _access: Access) -> Result<Self::Value>
        where
            Access: de::MapAccess<'de>,
        {
            Ok("test_key".to_string())
        }
    }

    struct TestRead {
        current_char: Option<u8>,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(self.current_char.take())
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(self.current_char)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut de = TestRead {
        current_char: Some(b'}'),
    };
    let mut map_access = MapAccess {
        de: &mut de,
        first: true,
    };

    let result = map_access.next_key_seed(TestSeed);

    assert!(result.is_err());
}

fn next_key_seed_test_case_eof_while_parsing_object() {
    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = String;

        fn deserialize<Access>(self, _access: Access) -> Result<Self::Value>
        where
            Access: de::MapAccess<'de>,
        {
            Ok("test_key".to_string())
        }
    }

    struct TestRead {
        current_char: Option<u8>,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(self.current_char.take())
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(self.current_char)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut de = TestRead {
        current_char: None,
    };
    let mut map_access = MapAccess {
        de: &mut de,
        first: true,
    };

    let result = map_access.next_key_seed(TestSeed);

    assert!(result.is_err());
}

