// Answer 0

#[test]
fn test_deserialize_unit_success() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct MockDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new(input: Vec<u8>) -> Self {
            MockDeserializer { input, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                if !byte.is_ascii_whitespace() {
                    return Ok(Some(byte));
                }
            }
            Ok(None)
        }

        fn peek_error(&self, _: ErrorCode) -> serde_json::Result<()> {
            Err(serde_json::Error::custom("EOF error"))
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_ident(&mut self, expected: &[u8]) -> Result<()> {
            let len = expected.len();
            let slice = &self.input[self.index..self.index + len];
            if slice != expected {
                return Err(serde_json::Error::custom("Unexpected identifier"));
            }
            self.index += len;
            Ok(())
        }

        fn fix_position(&self, err: serde_json::Error) -> serde_json::Error {
            err
        }

        fn peek_invalid_type<V: de::Visitor<'de>>(&self, _: &V) -> serde_json::Error {
            serde_json::Error::custom("Invalid type")
        }
    }

    let mut deserializer = MockDeserializer::new(b"null".to_vec());
    let visitor = MockVisitor;

    let result = deserializer.deserialize_unit(visitor);
    assert_eq!(result, Ok(()));
}

#[test]
#[should_panic]
fn test_deserialize_unit_invalid_peek() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct MockDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new(input: Vec<u8>) -> Self {
            MockDeserializer { input, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                if !byte.is_ascii_whitespace() {
                    return Ok(Some(byte));
                }
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_invalid_type<V: de::Visitor<'de>>(&self, _: &V) -> serde_json::Error {
            serde_json::Error::custom("Invalid type")
        }
    }

    let mut deserializer = MockDeserializer::new(b"notnull".to_vec());
    let visitor = MockVisitor;

    let _result = deserializer.deserialize_unit(visitor);
}

