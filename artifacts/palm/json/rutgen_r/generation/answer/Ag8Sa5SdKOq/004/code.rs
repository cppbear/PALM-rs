// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, cursor: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.cursor < self.input.len() && self.input[self.cursor].is_ascii_whitespace() {
                self.cursor += 1;
            }
            if self.cursor < self.input.len() {
                Ok(Some(self.input[self.cursor]))
            } else {
                Err(ErrorCode::EofWhileParsingValue)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            let ident_len = ident.len();
            if self.input[self.cursor..].starts_with(ident) {
                self.cursor += ident_len;
                Ok(())
            } else {
                Err(ErrorCode::InvalidIdentifier)
            }
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Error {
            Error::InvalidType
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::ParseError(code)
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
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
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = TestDeserializer::new(b"true".to_vec());
    let result = deserializer.deserialize_bool(TestVisitor);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_bool_false() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, cursor: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.cursor < self.input.len() && self.input[self.cursor].is_ascii_whitespace() {
                self.cursor += 1;
            }
            if self.cursor < self.input.len() {
                Ok(Some(self.input[self.cursor]))
            } else {
                Err(ErrorCode::EofWhileParsingValue)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            let ident_len = ident.len();
            if self.input[self.cursor..].starts_with(ident) {
                self.cursor += ident_len;
                Ok(())
            } else {
                Err(ErrorCode::InvalidIdentifier)
            }
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Error {
            Error::InvalidType
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::ParseError(code)
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
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
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = TestDeserializer::new(b"false".to_vec());
    let result = deserializer.deserialize_bool(TestVisitor);
    assert_eq!(result.unwrap(), false);
}

#[test]
fn test_deserialize_bool_invalid() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            Ok(false) // Dummy implementation to avoid panic.
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        cursor: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, cursor: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            while self.cursor < self.input.len() && self.input[self.cursor].is_ascii_whitespace() {
                self.cursor += 1;
            }
            if self.cursor < self.input.len() {
                Ok(Some(self.input[self.cursor]))
            } else {
                Err(ErrorCode::EofWhileParsingValue)
            }
        }

        fn eat_char(&mut self) {
            self.cursor += 1;
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            // Simulate an error condition here.
            Err(ErrorCode::InvalidIdentifier)
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> Error {
            Error::InvalidType
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::ParseError(code)
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
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
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = TestDeserializer::new(b"f".to_vec());
    let result = deserializer.deserialize_bool(TestVisitor);
    assert!(result.is_err());
}

