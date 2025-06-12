// Answer 0

#[test]
fn test_deserialize_map_valid_input() {
    struct Visitor;
    impl serde::de::Visitor<'_> for Visitor {
        type Value = String;

        fn visit_map<M>(self, _map: M) -> Result<Self::Value>
        where
            M: serde::de::MapAccess<'_>,
        {
            Ok(String::from("Valid Map"))
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                index: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.index < self.input.len() {
                let b = self.input[self.index];
                self.index += 1;
                if !b.is_ascii_whitespace() {
                    return Ok(Some(b));
                }
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn end_map(&self) -> Result<()> {
            Ok(())
        }

        fn peek_invalid_type(&self, _: &Visitor) -> std::fmt::Error {
            std::fmt::Error
        }

        fn peek_error(&self, _: ErrorCode) -> std::fmt::Error {
            std::fmt::Error
        }

        fn fix_position(&self, _err: std::fmt::Error) -> std::fmt::Error {
            std::fmt::Error
        }
    }

    impl TestDeserializer {
        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where
            V: serde::de::Visitor<'_>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b'{' => {
                    self.eat_char();
                    let ret = visitor.visit_map(MapAccess::new(self));
                    match (ret, self.end_map()) {
                        (Ok(ret), Ok(())) => Ok(ret),
                        (Err(err), _) | (_, Err(err)) => Err(err),
                    }
                }
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let deserializer = TestDeserializer::new("{ }");
    let result = deserializer.deserialize_map(Visitor);
    assert_eq!(result, Ok(String::from("Valid Map")));
}

#[test]
fn test_deserialize_map_empty_input() {
    struct Visitor;
    impl serde::de::Visitor<'_> for Visitor {
        type Value = String;

        fn visit_map<M>(self, _map: M) -> Result<Self::Value>
        where
            M: serde::de::MapAccess<'_>,
        {
            Ok(String::from("Valid Map"))
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                index: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.index < self.input.len() {
                let b = self.input[self.index];
                self.index += 1;
                if !b.is_ascii_whitespace() {
                    return Ok(Some(b));
                }
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn end_map(&self) -> Result<()> {
            Err(std::fmt::Error) // Simulate an error in ending map
        }

        fn peek_invalid_type(&self, _: &Visitor) -> std::fmt::Error {
            std::fmt::Error
        }

        fn peek_error(&self, _: ErrorCode) -> std::fmt::Error {
            std::fmt::Error
        }

        fn fix_position(&self, _err: std::fmt::Error) -> std::fmt::Error {
            std::fmt::Error
        }
    }

    impl TestDeserializer {
        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where
            V: serde::de::Visitor<'_>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b'{' => {
                    self.eat_char();
                    let ret = visitor.visit_map(MapAccess::new(self));
                    match (ret, self.end_map()) {
                        (Ok(ret), Ok(())) => Ok(ret),
                        (Err(err), _) | (_, Err(err)) => Err(err),
                    }
                }
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let deserializer = TestDeserializer::new("{ }");
    let result = deserializer.deserialize_map(Visitor);
    assert!(result.is_err());
}

