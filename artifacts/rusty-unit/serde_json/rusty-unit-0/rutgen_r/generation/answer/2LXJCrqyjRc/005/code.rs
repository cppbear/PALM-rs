// Answer 0

#[test]
fn test_do_deserialize_i128_success() {
    struct TestVisitor {
        value: Option<i128>,
    }

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: String,
        index: usize,
    }

    impl TestDeserializer {
        fn new(input: String) -> Self {
            Self { input, index: 0 }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str(&self.input[self.index..self.index + 1]); // Mock implementation
            self.index += 1;
            Ok(())
        }

        fn parse_whitespace(&mut self) -> Result<Option<char>> {
            if self.index < self.input.len() {
                let c = self.input.chars().nth(self.index).unwrap();
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = TestDeserializer::new("123".to_string());
    let result = deserializer.do_deserialize_i128(TestVisitor { value: None });
    assert_eq!(result, Ok(123));
}

#[test]
fn test_do_deserialize_i128_negative() {
    struct TestVisitor {
        value: Option<i128>,
    }

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: String,
        index: usize,
    }

    impl TestDeserializer {
        fn new(input: String) -> Self {
            Self { input, index: 0 }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str(&self.input[self.index..self.index + 2]); // Mock implementation
            self.index += 2;
            Ok(())
        }

        fn parse_whitespace(&mut self) -> Result<Option<char>> {
            if self.index < self.input.len() {
                let c = self.input.chars().nth(self.index).unwrap();
                self.index += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = TestDeserializer::new("-456".to_string());
    let result = deserializer.do_deserialize_i128(TestVisitor { value: None });
    assert_eq!(result, Ok(-456));
}

#[test]
fn test_do_deserialize_i128_invalid_number() {
    struct TestVisitor {
        value: Option<i128>,
    }

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: String,
        index: usize,
    }

    impl TestDeserializer {
        fn new(input: String) -> Self {
            Self { input, index: 0 }
        }

        fn scan_integer128(&mut self, _: &mut String) -> Result<()> {
            Err(ErrorCode::NumberOutOfRange) // Mock failure
        }

        fn parse_whitespace(&mut self) -> Result<Option<char>> {
            if self.index < self.input.len() {
                self.index += 1;
                Ok(Some(' ')) // Mock whitespace
            } else {
                Ok(None)
            }
        }

        fn error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = TestDeserializer::new("invalid".to_string());
    let result = deserializer.do_deserialize_i128(TestVisitor { value: None });
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_do_deserialize_i128_eof() {
    struct TestVisitor {
        value: Option<i128>,
    }

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: String,
        index: usize,
    }

    impl TestDeserializer {
        fn new(input: String) -> Self {
            Self { input, index: 0 }
        }

        fn scan_integer128(&mut self, _: &mut String) -> Result<()> {
            Ok(()) // Default behavior
        }

        fn parse_whitespace(&mut self) -> Result<Option<char>> {
            Err(ErrorCode::EofWhileParsingValue) // Force EOF error
        }

        fn error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = TestDeserializer::new("".to_string());
    let _result = deserializer.do_deserialize_i128(TestVisitor { value: None });
}

