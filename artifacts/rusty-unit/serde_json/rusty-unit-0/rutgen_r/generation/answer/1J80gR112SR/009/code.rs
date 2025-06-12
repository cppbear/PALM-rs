// Answer 0

#[test]
fn test_deserialize_number_success_negative() {
    struct Visitor;

    impl<'a> de::Visitor<'a> for Visitor {
        type Value = i32;

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let b = self.input[self.index];
                self.index += 1;
                Ok(Some(b))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_integer(&mut self, _is_positive: bool) -> Result<i32> {
            // Simulating a successful parse of a negative integer
            Ok(-42)
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<i32> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Result<i32> {
            Err(ErrorCode::InvalidType.into())
        }

        fn fix_position(&self, err: Result<i32>) -> Result<i32> {
            err
        }

        pub(crate) fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                Ok(None) => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
                Err(err) => return Err(err),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_integer(false).visit(visitor)
                }
                b'0'..=b'9' => self.parse_integer(true).visit(visitor),
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut parser = Parser {
        input: b"-42".to_vec(),
        index: 0,
    };

    let result = parser.deserialize_number(Visitor);
    assert_eq!(result, Ok(-42));
}

#[test]
fn test_deserialize_number_success_positive() {
    struct Visitor;

    impl<'a> de::Visitor<'a> for Visitor {
        type Value = i32;

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct Parser {
        input: Vec<u8>,
        index: usize,
    }

    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let b = self.input[self.index];
                self.index += 1;
                Ok(Some(b))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_integer(&mut self, _is_positive: bool) -> Result<i32> {
            // Simulating a successful parse of a positive integer
            Ok(42)
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<i32> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Result<i32> {
            Err(ErrorCode::InvalidType.into())
        }

        fn fix_position(&self, err: Result<i32>) -> Result<i32> {
            err
        }

        pub(crate) fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                Ok(None) => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
                Err(err) => return Err(err),
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_integer(false).visit(visitor)
                }
                b'0'..=b'9' => self.parse_integer(true).visit(visitor),
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut parser = Parser {
        input: b"42".to_vec(),
        index: 0,
    };

    let result = parser.deserialize_number(Visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_number_eof_error() {
    struct Visitor;

    impl<'a> de::Visitor<'a> for Visitor {
        type Value = i32;

        fn visit_i32<E>(self, _value: i32) -> Result<Self::Value, E> {
            Ok(0)
        }
    }

    struct Parser {
        index: usize,
    }

    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulating EOF
            Ok(None)
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<i32> {
            Err(ErrorCode::EofWhileParsingValue.into())
        }

        pub(crate) fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                Ok(None) => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
                Err(err) => return Err(err),
            };

            Err(ErrorCode::InvalidType.into())
        }
    }

    let mut parser = Parser { index: 0 };
    let result = parser.deserialize_number(Visitor);
    assert_eq!(result, Err(ErrorCode::EofWhileParsingValue.into()));
}

#[test]
fn test_deserialize_number_invalid_type() {
    struct Visitor;

    impl<'a> de::Visitor<'a> for Visitor {
        type Value = i32;

        fn visit_i32<E>(self, _value: i32) -> Result<Self::Value, E> {
            Ok(0)
        }
    }

    struct Parser {
        index: usize,
    }

    impl Parser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'x')) // Simulating an invalid character
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Result<i32> {
            Err(ErrorCode::InvalidType.into())
        }

        pub(crate) fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                Ok(None) => {
                    return Err(ErrorCode::EofWhileParsingValue.into());
                }
                Err(err) => return Err(err),
            };

            match peek {
                _ => Err(self.peek_invalid_type(&visitor)),
            }
        }
    }

    let mut parser = Parser { index: 0 };
    let result = parser.deserialize_number(Visitor);
    assert_eq!(result, Err(ErrorCode::InvalidType.into()));
}

