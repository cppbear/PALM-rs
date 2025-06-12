// Answer 0

#[test]
fn test_do_deserialize_i128_success() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: String,
        position: usize,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            Self { input: input.to_string(), position: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input.as_bytes()[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            // Simulate eating a char by just doing nothing for the test scope
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            let num_str = &self.input[self.position..];
            let num_bytes = num_str.bytes().take_while(|&b| b.is_ascii_digit() || b == b'-').collect::<Vec<_>>();
            if !num_bytes.is_empty() {
                self.position += num_bytes.len();
                buf.push_str(&String::from_utf8_lossy(&num_bytes));
                Ok(())
            } else {
                Err(Error::new(ErrorCode::NoValidInteger))
            }
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn peek_error(&self, err: ErrorCode) -> Error {
            Error::new(err)
        }
    }

    let mut deserializer = TestDeserializer::new("-1234567890123456789");

    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert_eq!(result.unwrap(), -1234567890123456789);
}

#[test]
fn test_do_deserialize_i128_parse_whitespace_error() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        position: usize,
    }

    impl TestDeserializer {
        fn new() -> Self {
            Self { position: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Err(Error::new(ErrorCode::WhitespaceError))
        }

        fn scan_integer128(&mut self, _buf: &mut String) -> Result<()> {
            Ok(())
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn peek_error(&self, err: ErrorCode) -> Error {
            Error::new(err)
        }
    }

    let mut deserializer = TestDeserializer::new();
    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_do_deserialize_i128_number_out_of_range() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, _value: i128) -> Result<Self::Value> {
            Err(Error::new(ErrorCode::NumberOutOfRange))
        }
    }

    struct TestDeserializer {
        input: String,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            Self { input: input.to_string() }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b' '))
        }

        fn eat_char(&mut self) {}

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str(&self.input);
            Ok(())
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn peek_error(&self, err: ErrorCode) -> Error {
            Error::new(err)
        }
    }

    let mut deserializer = TestDeserializer::new("1234567890123456789012345678901234567890");

    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert!(result.is_err());
}

