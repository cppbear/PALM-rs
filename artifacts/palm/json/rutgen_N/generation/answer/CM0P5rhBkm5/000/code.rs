// Answer 0

#[test]
fn test_deserialize_option_none() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value> {
            Ok(None)
        }

        fn visit_some<V>(self, value: V) -> Result<Self::Value> {
            Ok(Some(()))
        }
    }
    
    struct MockDeserializer {
        input: &'static [u8],
        current: usize,
    }

    impl MockDeserializer {
        fn new(input: &'static [u8]) -> Self {
            MockDeserializer { input, current: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.current < self.input.len() {
                self.current += 1;
                if self.current < self.input.len() {
                    return Ok(Some(self.input[self.current - 1]));
                }
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.current += 1;
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }
    }

    impl de::Deserializer<'static> for MockDeserializer {
        type Error = std::convert::Infallible;

        fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'static>,
        {
            match self.parse_whitespace().unwrap() {
                Some(b'n') => {
                    self.eat_char();
                    self.parse_ident(b"ull").unwrap();
                    visitor.visit_none()
                }
                _ => visitor.visit_some(self),
            }
        }
    }

    let mut deserializer = MockDeserializer::new(b"null");
    let visitor = TestVisitor;
    let result = deserializer.deserialize_option(visitor).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value> {
            Ok(None)
        }

        fn visit_some<V>(self, _value: V) -> Result<Self::Value> {
            Ok(Some(()))
        }
    }
    
    struct MockDeserializer {
        input: &'static [u8],
        current: usize,
    }

    impl MockDeserializer {
        fn new(input: &'static [u8]) -> Self {
            MockDeserializer { input, current: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.current < self.input.len() {
                self.current += 1;
                if self.current < self.input.len() {
                    return Ok(Some(self.input[self.current - 1]));
                }
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.current += 1;
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }
    }

    impl de::Deserializer<'static> for MockDeserializer {
        type Error = std::convert::Infallible;

        fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'static>,
        {
            match self.parse_whitespace().unwrap() {
                Some(b'n') => {
                    self.eat_char();
                    self.parse_ident(b"ull").unwrap();
                    visitor.visit_none()
                }
                _ => visitor.visit_some(self),
            }
        }
    }

    let mut deserializer = MockDeserializer::new(b"value");
    let visitor = TestVisitor;
    let result = deserializer.deserialize_option(visitor).unwrap();
    assert_eq!(result, Some(()));
}

