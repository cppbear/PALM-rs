// Answer 0

#[test]
fn test_do_deserialize_i128_valid_negative() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
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
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                index: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                self.index += 1;
                Ok(Some(self.input.as_bytes()[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str("12345678901234567890");
            Ok(())
        }

        fn do_deserialize_i128<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            // Original function code
            let mut buf = String::new();

            match self.parse_whitespace()? {
                Some(b'-') => {
                    self.eat_char();
                    buf.push('-');
                }
                Some(_) => {}
                None => {
                    return Err(Error::new(ErrorCode::EofWhileParsingValue));
                }
            }

            self.scan_integer128(&mut buf)?;

            let value = match buf.parse() {
                Ok(int) => visitor.visit_i128(int),
                Err(_) => {
                    return Err(Error::new(ErrorCode::NumberOutOfRange));
                }
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            }
        }
    }

    let mut deserializer = TestDeserializer::new(" - ");
    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().code(), ErrorCode::NumberOutOfRange);
}

#[test]
fn test_do_deserialize_i128_invalid() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
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
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                index: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                self.index += 1;
                Ok(Some(self.input.as_bytes()[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str("invalid"); // Invalid number
            Ok(())
        }

        fn do_deserialize_i128<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let mut buf = String::new();

            match self.parse_whitespace()? {
                Some(b'-') => {
                    self.eat_char();
                    buf.push('-');
                }
                Some(_) => {}
                None => {
                    return Err(Error::new(ErrorCode::EofWhileParsingValue));
                }
            }

            self.scan_integer128(&mut buf)?;

            let value = match buf.parse() {
                Ok(int) => visitor.visit_i128(int),
                Err(_) => {
                    return Err(Error::new(ErrorCode::NumberOutOfRange));
                }
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            }
        }
    }

    let mut deserializer = TestDeserializer::new(" - ");
    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().code(), ErrorCode::NumberOutOfRange);
}

