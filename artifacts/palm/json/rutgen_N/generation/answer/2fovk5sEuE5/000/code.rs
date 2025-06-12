// Answer 0

#[test]
fn test_deserialize_any_borrowed_str() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value> {
            assert_eq!(value, "test");
            Ok(value)
        }
        
        fn visit_str(self, value: String) -> Result<Self::Value> {
            panic!("visit_str should not be called.");
        }
    }

    struct MockDeserializer {
        de: MockDe,
    }

    struct MockDe {
        scratch: String,
        input: &'static str,
        index: usize,
    }

    impl MockDe {
        fn eat_char(&mut self) {
            self.index += 1;
        }
        
        fn read(&mut self) -> &str {
            &self.input[self.index..]
        }
    }

    impl MockDeserializer {
        fn new(input: &'static str) -> Self {
            Self {
                de: MockDe {
                    scratch: String::new(),
                    input,
                    index: 0,
                },
            }
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            self.de.eat_char();
            self.de.scratch.clear();
            match self.de.read().parse() {
                Ok(value) => visitor.visit_borrowed_str(value),
                Err(_) => Err(/* appropriate error handling here */),
            }
        }
    }

    let deserializer = MockDeserializer::new("test");
    let result = deserializer.deserialize_any(MockVisitor).unwrap();
    assert_eq!(result, "test");
}

#[test]
fn test_deserialize_any_copied_str() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value> {
            panic!("visit_borrowed_str should not be called.");
        }
        
        fn visit_str(self, value: String) -> Result<Self::Value> {
            assert_eq!(value, "test");
            Ok(value)
        }
    }

    struct MockDeserializer {
        de: MockDe,
    }

    struct MockDe {
        scratch: String,
        input: &'static str,
        index: usize,
    }

    impl MockDe {
        fn eat_char(&mut self) {
            self.index += 1;
        }
        
        fn read(&mut self) -> &str {
            &self.input[self.index..]
        }
    }

    impl MockDeserializer {
        fn new(input: &'static str) -> Self {
            Self {
                de: MockDe {
                    scratch: String::new(),
                    input,
                    index: 0,
                },
            }
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            self.de.eat_char();
            self.de.scratch.clear();
            match self.de.read().parse() {
                Ok(value) => visitor.visit_str(value.to_string()),
                Err(_) => Err(/* appropriate error handling here */),
            }
        }
    }

    let deserializer = MockDeserializer::new("test");
    let result = deserializer.deserialize_any(MockVisitor).unwrap();
    assert_eq!(result, "test");
}

