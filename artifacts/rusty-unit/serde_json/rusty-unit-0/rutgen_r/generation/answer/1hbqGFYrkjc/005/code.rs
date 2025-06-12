// Answer 0

#[test]
fn test_deserialize_str_ok_borrowed() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value> {
            Ok(value)
        }

        fn visit_str(self, value: String) -> Result<Self::Value> {
            Err(Error::custom("Should not be called in this test"))
        }
    }

    struct MockDeserializer {
        scratch: String,
        peek_value: u8,
        result: Result<Reference<'static, String>, Error>
    }

    impl MockDeserializer {
        fn new(peek_value: u8, result: Result<Reference<'static, String>, Error>) -> Self {
            Self {
                scratch: String::new(),
                peek_value,
                result,
            }
        }

        fn parse_whitespace(&self) -> Result<Option<u8>> {
            Ok(Some(self.peek_value))
        }

        fn eat_char(&mut self) {}

        fn read(&self) -> &Self {
            self
        }

        fn read_str(&self, _: &mut String) -> Result<Reference<'static, String>, Error> {
            self.result.clone()
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::custom("Peek error")
        }

        fn peek_invalid_type(&self, _: &MockVisitor) -> Error {
            Error::custom("Invalid type")
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let deserializer = MockDeserializer::new(b'"', Ok(Reference::Borrowed("success")));
    let visitor = MockVisitor;
    let result = deserializer.deserialize_str(visitor);
    assert_eq!(result, Ok("success"));
}

#[test]
fn test_deserialize_str_err() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> {
            Err(Error::custom("Should not be called in this test"))
        }

        fn visit_str(self, value: String) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        scratch: String,
        peek_value: u8,
        result: Result<Reference<'static, String>, Error>
    }

    impl MockDeserializer {
        fn new(peek_value: u8, result: Result<Reference<'static, String>, Error>) -> Self {
            Self {
                scratch: String::new(),
                peek_value,
                result,
            }
        }

        fn parse_whitespace(&self) -> Result<Option<u8>> {
            Ok(Some(self.peek_value))
        }

        fn eat_char(&mut self) {}

        fn read(&self) -> &Self {
            self
        }

        fn read_str(&self, _: &mut String) -> Result<Reference<'static, String>, Error> {
            self.result.clone()
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::custom("Peek error")
        }

        fn peek_invalid_type(&self, _: &MockVisitor) -> Error {
            Error::custom("Invalid type")
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let deserializer = MockDeserializer::new(b'"', Err(Error::custom("Parse error")));
    let visitor = MockVisitor;
    let result = deserializer.deserialize_str(visitor);
    assert_eq!(result, Err(Error::custom("Parse error")));
}

#[test]
#[should_panic]
fn test_deserialize_str_invalid_type() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value> {
            Err(Error::custom("Should not be called in this test"))
        }

        fn visit_str(self, value: String) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        scratch: String,
        peek_value: u8
    }

    impl MockDeserializer {
        fn new(peek_value: u8) -> Self {
            Self {
                scratch: String::new(),
                peek_value
            }
        }

        fn parse_whitespace(&self) -> Result<Option<u8>> {
            Ok(Some(self.peek_value))
        }

        fn eat_char(&mut self) {}

        fn read(&self) -> &Self {
            self
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::custom("Peek error")
        }

        fn peek_invalid_type(&self, _: &MockVisitor) -> Error {
            panic!("Should panic, invalid type");
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let deserializer = MockDeserializer::new(b'a'); // Invalid peek value
    let visitor = MockVisitor;
    let _result = deserializer.deserialize_str(visitor); // This should panic
}

