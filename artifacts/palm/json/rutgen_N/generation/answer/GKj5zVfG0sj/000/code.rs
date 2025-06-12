// Answer 0

#[test]
fn test_deserialize_struct_map() {
    struct TestVisitor {
        value: Option<()>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _: V) -> Result<Self::Value>
        where
            V: de::MapAccess<'de>,
        {
            self.value = Some(());
            Ok(())
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        position: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                if byte.is_ascii_whitespace() {
                    continue;
                }
                return Ok(Some(byte));
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn end_map(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::custom("error")
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn peek_invalid_type<V>(&self, _: &V) -> Error {
            Error::custom("invalid type")
        }
    }

    impl TestDeserializer {
        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            // Call the actual function to test
            // Here we assume `b'{'` is the starting byte for the map
            self.deserialize_struct("test", &[], visitor)
        }
    }

    let mut deserializer = TestDeserializer::new(b"{\"key\": \"value\"}".to_vec());
    let result = deserializer.deserialize_struct("Test", &["key"], TestVisitor { value: None });
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_deserialize_struct_seq() {
    struct TestVisitor {
        value: Option<()>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<V>(self, _: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            self.value = Some(());
            Ok(())
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        position: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                if byte.is_ascii_whitespace() {
                    continue;
                }
                return Ok(Some(byte));
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn end_seq(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::custom("error")
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn peek_invalid_type<V>(&self, _: &V) -> Error {
            Error::custom("invalid type")
        }
    }

    impl TestDeserializer {
        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            // Call the actual function to test
            // Here we assume `b'['` is the starting byte for the sequence
            self.deserialize_struct("test", &[], visitor)
        }
    }

    let mut deserializer = TestDeserializer::new(b"[1, 2, 3]".to_vec());
    let result = deserializer.deserialize_struct("Test", &[], TestVisitor { value: None });
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

