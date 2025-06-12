// Answer 0

#[test]
fn test_deserialize_number_with_negative_integer() {
    struct Visitor;
    impl de::Visitor<'_> for Visitor {
        type Value = i32;
        fn visit_i32(self, value: i32) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        input: &'static [u8],
        index: usize,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Some(self.input[self.index]).into()
            } else {
                Err(ErrorCode::EofWhileParsingValue.into())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_integer(&mut self, _: bool) -> Result<i32> {
            let num_str = std::str::from_utf8(&self.input[self.index..self.index+2]).unwrap(); // e.g., "42"
            self.index += 2; // Move past the integer
            Ok(num_str.parse().unwrap())
        }

        fn peek_error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn peek_invalid_type(&self, _: &Visitor) -> Result<()> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn fix_position(&self, err: Result<i32>) -> Result<i32> {
            err
        }

        fn deserialize_number(&mut self, visitor: Visitor) -> Result<Visitor::Value> {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => return Err(ErrorCode::EofWhileParsingValue.into()),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_integer(false).and_then(|val| visitor.visit_i32(val))
                },
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = MockDeserializer { input: b"-42", index: 0 };
    let result = deserializer.deserialize_number(Visitor);
    assert_eq!(result, Ok(-42));
}

#[test]
fn test_deserialize_number_with_positive_integer() {
    struct Visitor;
    impl de::Visitor<'_> for Visitor {
        type Value = i32;
        fn visit_i32(self, value: i32) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        input: &'static [u8],
        index: usize,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Some(self.input[self.index]).into()
            } else {
                Err(ErrorCode::EofWhileParsingValue.into())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_integer(&mut self, _: bool) -> Result<i32> {
            let num_str = std::str::from_utf8(&self.input[self.index..self.index+2]).unwrap(); // e.g., "42"
            self.index += 2; // Move past the integer
            Ok(num_str.parse().unwrap())
        }

        fn peek_error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn peek_invalid_type(&self, _: &Visitor) -> Result<()> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn fix_position(&self, err: Result<i32>) -> Result<i32> {
            err
        }

        fn deserialize_number(&mut self, visitor: Visitor) -> Result<Visitor::Value> {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => return Err(ErrorCode::EofWhileParsingValue.into()),
            };

            let value = match peek {
                b'0'..=b'9' => {
                    self.parse_integer(true).and_then(|val| visitor.visit_i32(val))
                },
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = MockDeserializer { input: b"42", index: 0 };
    let result = deserializer.deserialize_number(Visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_number_eof_error() {
    struct Visitor;
    impl de::Visitor<'_> for Visitor {
        type Value = i32;
        fn visit_i32(self, _: i32) -> Result<Self::Value> {
            Ok(0) // Dummy return
        }
    }

    struct MockDeserializer {
        input: &'static [u8],
        index: usize,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Some(self.input[self.index]).into()
            } else {
                Err(ErrorCode::EofWhileParsingValue.into())
            }
        }

        fn deserialize_number(&mut self, visitor: Visitor) -> Result<Visitor::Value> {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => return Err(ErrorCode::EofWhileParsingValue.into()),
            };

            let value = match peek {
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }

        fn peek_invalid_type(&self, _: &Visitor) -> Result<()> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn fix_position(&self, err: Result<i32>) -> Result<i32> {
            err
        }
    }

    let mut deserializer = MockDeserializer { input: b"", index: 0 };
    let result = deserializer.deserialize_number(Visitor);
    assert!(result.is_err());
}

