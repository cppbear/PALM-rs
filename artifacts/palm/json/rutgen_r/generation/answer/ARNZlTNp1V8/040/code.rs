// Answer 0

#[test]
fn test_deserialize_any_boolean_true() {
    struct VisitorMock {
        value: Option<bool>,
    }

    impl<'de> de::Visitor<'de> for VisitorMock {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(false)
        }
    }

    struct DeserializerMock {
        input: Vec<u8>,
        pos: usize,
    }

    impl DeserializerMock {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, &'static str> {
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn parse_ident(&mut self, expected: &[u8]) -> Result<(), &'static str> {
            for &byte in expected {
                if self.pos >= self.input.len() || self.input[self.pos] != byte {
                    return Err("Expected identifier not found");
                }
                self.pos += 1;
            }
            Ok(())
        }
    }

    let mut deserializer = DeserializerMock::new(b"true".to_vec());
    let visitor = VisitorMock { value: None };
    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_any_boolean_false() {
    struct VisitorMock {
        value: Option<bool>,
    }

    impl<'de> de::Visitor<'de> for VisitorMock {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(false)
        }
    }

    struct DeserializerMock {
        input: Vec<u8>,
        pos: usize,
    }

    impl DeserializerMock {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, &'static str> {
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn parse_ident(&mut self, expected: &[u8]) -> Result<(), &'static str> {
            for &byte in expected {
                if self.pos >= self.input.len() || self.input[self.pos] != byte {
                    return Err("Expected identifier not found");
                }
                self.pos += 1;
            }
            Ok(())
        }
    }

    let mut deserializer = DeserializerMock::new(b"false".to_vec());
    let visitor = VisitorMock { value: None };
    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), false);
}

#[test]
#[should_panic]
fn test_deserialize_any_invalid_value() {
    struct VisitorMock {
        value: Option<bool>,
    }

    impl<'de> de::Visitor<'de> for VisitorMock {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(false)
        }
    }

    struct DeserializerMock {
        input: Vec<u8>,
        pos: usize,
    }

    impl DeserializerMock {
        fn new(input: Vec<u8>) -> Self {
            Self { input, pos: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, &'static str> {
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), &'static str> {
            panic!("Invalid identifier");
        }
    }

    let mut deserializer = DeserializerMock::new(b"invalid".to_vec());
    let visitor = VisitorMock { value: None };
    let _ = deserializer.deserialize_any(visitor);
}

