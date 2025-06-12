// Answer 0

#[test]
fn test_deserialize_number_positive_integer() {
    struct Visitor;

    impl<'any> de::Visitor<'any> for Visitor {
        type Value = i32;

        fn visit_i32(self, value: i32) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct Deserializer {
        input: &'static [u8],
        index: usize,
    }

    impl Deserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_integer(&mut self, _: bool) -> Result<i32> {
            // Simulating parsing a positive integer.
            Ok(42)
        }

        fn peek_error(&self, _: ErrorCode) -> Result<i32> {
            Err(Error::new("EOF error"))
        }

        fn peek_invalid_type(&self, _: &Visitor) -> Result<i32> {
            Err(Error::new("Invalid type error"))
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_integer(false)?.visit(visitor)
                },
                b'0'..=b'9' => self.parse_integer(true)?.visit(visitor),
                _ => return Err(self.peek_invalid_type(&visitor)),
            };

            value.map_err(|err| self.fix_position(err))
        }
    }

    let mut deserializer = Deserializer {
        input: b" 42",
        index: 0,
    };

    let result = deserializer.deserialize_number(Visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_number_negative_integer() {
    struct Visitor;

    impl<'any> de::Visitor<'any> for Visitor {
        type Value = i32;

        fn visit_i32(self, value: i32) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct Deserializer {
        input: &'static [u8],
        index: usize,
    }

    impl Deserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_integer(&mut self, _: bool) -> Result<i32> {
            // Simulating parsing a negative integer.
            Ok(-42)
        }

        fn peek_error(&self, _: ErrorCode) -> Result<i32> {
            Err(Error::new("EOF error"))
        }

        fn peek_invalid_type(&self, _: &Visitor) -> Result<i32> {
            Err(Error::new("Invalid type error"))
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_integer(false)?.visit(visitor)
                },
                b'0'..=b'9' => self.parse_integer(true)?.visit(visitor),
                _ => return Err(self.peek_invalid_type(&visitor)),
            };

            value.map_err(|err| self.fix_position(err))
        }
    }

    let mut deserializer = Deserializer {
        input: b"-42",
        index: 0,
    };

    let result = deserializer.deserialize_number(Visitor);
    assert_eq!(result, Ok(-42));
}

#[test]
#[should_panic]
fn test_deserialize_number_eof_error() {
    struct Visitor;

    impl<'any> de::Visitor<'any> for Visitor {
        type Value = i32;

        fn visit_i32(self, _: i32) -> Result<Self::Value> {
            Ok(0)
        }
    }

    struct Deserializer {
        input: &'static [u8],
        index: usize,
    }

    impl Deserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn eat_char(&mut self) {}

        fn parse_integer(&mut self, _: bool) -> Result<i32> {
            Ok(0)
        }

        fn peek_error(&self, _: ErrorCode) -> Result<i32> {
            Err(Error::new("EOF error"))
        }

        fn peek_invalid_type(&self, _: &Visitor) -> Result<i32> {
            Err(Error::new("Invalid type error"))
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_integer(false)?.visit(visitor)
                },
                b'0'..=b'9' => self.parse_integer(true)?.visit(visitor),
                _ => return Err(self.peek_invalid_type(&visitor)),
            };

            value.map_err(|err| self.fix_position(err))
        }
    }

    let mut deserializer = Deserializer {
        input: b"",
        index: 0,
    };

    let _ = deserializer.deserialize_number(Visitor);
}

