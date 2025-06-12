// Answer 0

#[test]
fn test_deserialize_any_return_err_on_eof() {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();
        
        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_str(self, _: &str) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_seq(self, _: SeqAccess<'de>) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_map(self, _: MapAccess<'de>) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct TestDeserializer {
        input: &'static [u8],
        index: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                self.index += 1;
                Ok(Some(self.input[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, _: ErrorCode) -> Result<()> {
            Err(())
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            // Simulating an error for the test case
            Err(())
        }
        
        fn deserialize_any<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => return self.peek_error(ErrorCode::EofWhileParsingValue),
            };

            match peek {
                b'f' => {
                    self.eat_char();
                    self.parse_ident(b"alse")?; // This will trigger error
                    visitor.visit_bool(false)
                }
                _ => Err(self.peek_error(ErrorCode::ExpectedSomeValue)?),
            }
        }
    }

    let mut deserializer = TestDeserializer { input: b"false", index: 0 };
    let visitor = Visitor;
    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_any_return_err_on_invalid_number() {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_str(self, _: &str) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_seq(self, _: SeqAccess<'de>) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_map(self, _: MapAccess<'de>) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct TestDeserializer {
        input: &'static [u8],
        index: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                self.index += 1;
                Ok(Some(self.input[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, _: ErrorCode) -> Result<()> {
            Err(())
        }

        fn parse_any_number(&mut self, _: bool) -> Result<Visitor> {
            // Simulating an error for the test case
            Err(())
        }

        fn deserialize_any<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => return self.peek_error(ErrorCode::EofWhileParsingValue),
            };

            match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_any_number(false)?; // This will trigger error
                    visitor.visit_bool(false) // Just for fulfillment
                }
                _ => Err(self.peek_error(ErrorCode::ExpectedSomeValue)?),
            }
        }
    }

    let mut deserializer = TestDeserializer { input: b"-123", index: 0 };
    let visitor = Visitor;
    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
}

