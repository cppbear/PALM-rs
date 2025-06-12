// Answer 0

#[test]
fn test_do_deserialize_i128_success() {
    struct TestVisitor {
        value: Option<i128>,
    }

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            self.value = Some(value);
            Ok(value)
        }
    }

    struct TestDeserializer {
        cursor: usize,
        input: Vec<char>,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            Self {
                cursor: 0,
                input: input.chars().collect(),
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<char>> {
            while self.cursor < self.input.len() && self.input[self.cursor].is_whitespace() {
                self.cursor += 1;
            }
            if self.cursor < self.input.len() {
                Ok(Some(self.input[self.cursor]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            while self.cursor < self.input.len() && self.input[self.cursor].is_digit(10) {
                buf.push(self.input[self.cursor]);
                self.cursor += 1;
            }
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
                    return Err(ErrorCode::EofWhileParsingValue.into());
                }
            }

            self.scan_integer128(&mut buf)?;

            let value = match buf.parse() {
                Ok(int) => visitor.visit_i128(int),
                Err(_) => {
                    return Err(ErrorCode::NumberOutOfRange.into());
                }
            };

            value
        }
    }

    let mut deserializer = TestDeserializer::new("  -12345  ");
    let visitor = TestVisitor { value: None };
    let result = deserializer.do_deserialize_i128(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -12345);
}

#[test]
fn test_do_deserialize_i128_invalid_number() {
    struct TestVisitor;

    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;

        fn visit_i128(self, _value: i128) -> Result<Self::Value> {
            Ok(0)
        }
    }

    struct TestDeserializer {
        cursor: usize,
        input: Vec<char>,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            Self {
                cursor: 0,
                input: input.chars().collect(),
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<char>> {
            while self.cursor < self.input.len() && self.input[self.cursor].is_whitespace() {
                self.cursor += 1;
            }
            if self.cursor < self.input.len() {
                Ok(Some(self.input[self.cursor]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            while self.cursor < self.input.len() && self.input[self.cursor].is_digit(10) {
                buf.push(self.input[self.cursor]);
                self.cursor += 1;
            }
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
                    return Err(ErrorCode::EofWhileParsingValue.into());
                }
            }

            self.scan_integer128(&mut buf)?;

            let value = match buf.parse() {
                Ok(int) => visitor.visit_i128(int),
                Err(_) => {
                    return Err(ErrorCode::NumberOutOfRange.into());
                }
            };

            value
        }
    }

    let mut deserializer = TestDeserializer::new("not_a_number");
    let visitor = TestVisitor;
    let result = deserializer.do_deserialize_i128(visitor);
    assert!(result.is_err());
}

