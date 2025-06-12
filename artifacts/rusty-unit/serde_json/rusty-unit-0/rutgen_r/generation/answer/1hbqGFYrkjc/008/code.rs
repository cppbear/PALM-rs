// Answer 0

#[test]
fn test_deserialize_str_ok_borrowed() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, v: &'de str) -> Result<Self::Value> {
            Ok(v)
        }

        fn visit_str(self, v: String) -> Result<Self::Value> {
            Ok(Box::leak(v.into_boxed_str()))
        }
    }

    struct TestDeserializer {
        peek_value: Option<u8>,
        scratch: String,
        read: TestRead,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(self.peek_value)
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::default()
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Error {
            Error::default()
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b'"' => {
                    self.eat_char();
                    self.scratch.clear();
                    match self.read.parse_str(&mut self.scratch) {
                        Reference::Borrowed(s) => visitor.visit_borrowed_str(s),
                        Reference::Copied(s) => visitor.visit_str(s),
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

    struct TestRead;

    impl TestRead {
        fn parse_str(&self, _scratch: &mut String) -> Reference<'_> {
            *scratch = "test".to_string();
            Reference::Borrowed("test")
        }
    }

    let deserializer = TestDeserializer {
        peek_value: Some(b'"'),
        scratch: String::new(),
        read: TestRead,
    };

    let result: Result<&str> = deserializer.deserialize_str(TestVisitor);

    assert_eq!(result, Ok("test"));
}

#[test]
fn test_deserialize_str_eof_error() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, _v: &'de str) -> Result<Self::Value> {
            Ok("")
        }

        fn visit_str(self, _v: String) -> Result<Self::Value> {
            Ok("")
        }
    }

    struct TestDeserializer {
        peek_value: Option<u8>,
        scratch: String,
        read: TestRead,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(self.peek_value)
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::default()
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Error {
            Error::default()
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b'"' => {
                    self.eat_char();
                    self.scratch.clear();
                    match self.read.parse_str(&mut self.scratch) {
                        Reference::Borrowed(s) => visitor.visit_borrowed_str(s),
                        Reference::Copied(s) => visitor.visit_str(s),
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

    struct TestRead;

    impl TestRead {
        fn parse_str(&self, _scratch: &mut String) -> Reference<'_> {
            Reference::Borrowed("")
        }
    }

    let deserializer = TestDeserializer {
        peek_value: None,
        scratch: String::new(),
        read: TestRead,
    };

    let result: Result<&str> = deserializer.deserialize_str(TestVisitor);

    assert_eq!(result.is_err(), true);
} 

#[test]
fn test_deserialize_str_invalid_type() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, _v: &'de str) -> Result<Self::Value> {
            Ok("")
        }

        fn visit_str(self, _v: String) -> Result<Self::Value> {
            Ok("")
        }
    }

    struct TestDeserializer {
        peek_value: Option<u8>,
        scratch: String,
        read: TestRead,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(self.peek_value)
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::default()
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Error {
            Error::default()
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b'"' => {
                    self.eat_char();
                    self.scratch.clear();
                    match self.read.parse_str(&mut self.scratch) {
                        Reference::Borrowed(s) => visitor.visit_borrowed_str(s),
                        Reference::Copied(s) => visitor.visit_str(s),
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

    struct TestRead;

    impl TestRead {
        fn parse_str(&self, _scratch: &mut String) -> Reference<'_> {
            Reference::Borrowed("")
        }
    }

    let deserializer = TestDeserializer {
        peek_value: Some(b'a'), // Invalid type
        scratch: String::new(),
        read: TestRead,
    };

    let result: Result<&str> = deserializer.deserialize_str(TestVisitor);

    assert_eq!(result.is_err(), true);
}

