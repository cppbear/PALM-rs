// Answer 0

#[test]
fn test_deserialize_map_ok() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestDeserializer {
        fn new(input: &[u8]) -> Self {
            TestDeserializer {
                input: input.to_vec(),
                cursor: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, serde::de::Error> {
            while self.cursor < self.input.len() && self.input[self.cursor].is_ascii_whitespace() {
                self.cursor += 1;
            }
            if self.cursor < self.input.len() {
                Ok(Some(self.input[self.cursor]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn end_map(&mut self) -> Result<(), serde::de::Error> {
            if self.cursor < self.input.len() && self.input[self.cursor] == b'}' {
                self.cursor += 1;
                Ok(())
            } else {
                Err(serde::de::Error::custom("Expected end of map"))
            }
        }

        fn peek_invalid_type(&self, _visitor: &impl serde::de::Visitor) -> serde::de::Error {
            serde::de::Error::custom("Invalid type")
        }

        fn fix_position(&self, err: serde::de::Error) -> serde::de::Error {
            err
        }

        fn peek_error(&self, _code: serde::de::Error) -> serde::de::Error {
            serde::de::Error::custom("Peek error")
        }

        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let peek = self.parse_whitespace()?;
            let peek_byte = match peek {
                Some(b) => b,
                None => return Err(self.peek_error(serde::de::Error::custom("EOF while parsing value"))),
            };

            match peek_byte {
                b'{' => {
                    self.eat_char();
                    let ret = visitor.visit_map(MapAccess::new(self));

                    match (ret, self.end_map()) {
                        (Ok(ret), Ok(())) => Ok(ret),
                        (Err(err), _) | (_, Err(err)) => Err(err),
                    }
                }
                _ => Err(self.peek_invalid_type(&visitor)),
            }
        }
    }

    struct MapAccess {
        deserializer: TestDeserializer,
    }

    impl MapAccess {
        fn new(deserializer: TestDeserializer) -> Self {
            MapAccess { deserializer }
        }
    }

    let deserializer = TestDeserializer::new(b" { } ");
    let result = deserializer.deserialize_map(TestVisitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_map_eof_error() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestDeserializer {
        fn new(input: &[u8]) -> Self {
            TestDeserializer {
                input: input.to_vec(),
                cursor: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, serde::de::Error> {
            while self.cursor < self.input.len() && self.input[self.cursor].is_ascii_whitespace() {
                self.cursor += 1;
            }
            if self.cursor < self.input.len() {
                Ok(Some(self.input[self.cursor]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn end_map(&mut self) -> Result<(), serde::de::Error> {
            Err(serde::de::Error::custom("Expected end of map"))
        }

        fn peek_invalid_type(&self, _visitor: &impl serde::de::Visitor) -> serde::de::Error {
            serde::de::Error::custom("Invalid type")
        }

        fn fix_position(&self, err: serde::de::Error) -> serde::de::Error {
            err
        }

        fn peek_error(&self, _code: serde::de::Error) -> serde::de::Error {
            serde::de::Error::custom("Peek error")
        }

        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let peek = self.parse_whitespace()?;
            let peek_byte = match peek {
                Some(b) => b,
                None => return Err(self.peek_error(serde::de::Error::custom("EOF while parsing value"))),
            };

            match peek_byte {
                b'{' => {
                    self.eat_char();
                    let ret = visitor.visit_map(MapAccess::new(self));

                    match (ret, self.end_map()) {
                        (Ok(ret), Ok(())) => Ok(ret),
                        (Err(err), _) | (_, Err(err)) => Err(err),
                    }
                }
                _ => Err(self.peek_invalid_type(&visitor)),
            }
        }
    }

    struct MapAccess {
        deserializer: TestDeserializer,
    }

    impl MapAccess {
        fn new(deserializer: TestDeserializer) -> Self {
            MapAccess { deserializer }
        }
    }

    let deserializer = TestDeserializer::new(b"  ");
    deserializer.deserialize_map(TestVisitor).unwrap();
}

