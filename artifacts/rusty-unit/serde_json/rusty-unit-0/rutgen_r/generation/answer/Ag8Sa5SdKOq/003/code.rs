// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }

        // Implement other methods as necessary for the trait
    }

    struct TestDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.index < self.input.len() && self.input[self.index].is_ascii_whitespace() {
                self.index += 1;
            }
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Err(ErrorCode::EofWhileParsingValue.into())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            let ident_len = ident.len();
            if self.index + ident_len <= self.input.len() && &self.input[self.index..self.index + ident_len] == ident {
                self.index += ident_len;
                Ok(())
            } else {
                Err(ErrorCode::InvalidIdentifier.into())
            }
        }

        fn peek_invalid_type<V: de::Visitor<'de>>(&self, visitor: &V) -> Error {
            Error::InvalidType
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::Eof
        }

        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b't' => {
                    self.eat_char();
                    self.parse_ident(b"rue")?;
                    visitor.visit_bool(true)
                }
                b'f' => {
                    self.eat_char();
                    self.parse_ident(b"alse")?;
                    visitor.visit_bool(false)
                }
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.peek_error(err)),
            }
        }
    }

    let deserializer = TestDeserializer::new(b" true".to_vec());
    let visitor = Visitor;

    let result = deserializer.deserialize_bool(visitor);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_bool_false() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.index < self.input.len() && self.input[self.index].is_ascii_whitespace() {
                self.index += 1;
            }
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Err(ErrorCode::EofWhileParsingValue.into())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            let ident_len = ident.len();
            if self.index + ident_len <= self.input.len() && &self.input[self.index..self.index + ident_len] == ident {
                self.index += ident_len;
                Ok(())
            } else {
                Err(ErrorCode::InvalidIdentifier.into())
            }
        }

        fn peek_invalid_type<V: de::Visitor<'de>>(&self, visitor: &V) -> Error {
            Error::InvalidType
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::Eof
        }

        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b't' => {
                    self.eat_char();
                    self.parse_ident(b"rue")?;
                    visitor.visit_bool(true)
                }
                b'f' => {
                    self.eat_char();
                    self.parse_ident(b"alse")?;
                    visitor.visit_bool(false)
                }
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.peek_error(err)),
            }
        }
    }

    let deserializer = TestDeserializer::new(b" false".to_vec());
    let visitor = Visitor;

    let result = deserializer.deserialize_bool(visitor);
    assert_eq!(result, Ok(false));
}

#[test]
fn test_deserialize_bool_invalid() {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.index < self.input.len() && self.input[self.index].is_ascii_whitespace() {
                self.index += 1;
            }
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Err(ErrorCode::EofWhileParsingValue.into())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Err(ErrorCode::InvalidIdentifier.into())
        }

        fn peek_invalid_type<V: de::Visitor<'de>>(&self, visitor: &V) -> Error {
            Error::InvalidType
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::Eof
        }

        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b't' => {
                    self.eat_char();
                    self.parse_ident(b"rue")?;
                    visitor.visit_bool(true)
                }
                b'f' => {
                    self.eat_char();
                    self.parse_ident(b"alse")?;
                    visitor.visit_bool(false)
                }
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(self.peek_error(err)),
            }
        }
    }

    let deserializer = TestDeserializer::new(b" invalid".to_vec());
    let visitor = Visitor;

    let result = deserializer.deserialize_bool(visitor);
    assert!(result.is_err());
}

