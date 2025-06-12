// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: &'static [u8],
        position: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&mut self, expected: &[u8]) -> Result<()> {
            let ident: Vec<u8> = self.input[self.position..self.position + expected.len()].to_vec();
            self.position += expected.len();
            if ident == expected {
                Ok(())
            } else {
                Err(ErrorCode::UnexpectedToken)
            }
        }

        fn peek_invalid_type(&self, _visitor: &Visitor) -> ErrorCode {
            ErrorCode::InvalidType
        }

        fn peek_error(&self, _error_code: ErrorCode) -> ErrorCode {
            ErrorCode::ParseError
        }

        fn fix_position(&self, err: ErrorCode) -> ErrorCode {
            err
        }

        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
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

    let mut deserializer = TestDeserializer {
        input: b"true",
        position: 0,
    };

    let result = deserializer.deserialize_bool(Visitor);
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
        input: &'static [u8],
        position: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&mut self, expected: &[u8]) -> Result<()> {
            let ident: Vec<u8> = self.input[self.position..self.position + expected.len()].to_vec();
            self.position += expected.len();
            if ident == expected {
                Ok(())
            } else {
                Err(ErrorCode::UnexpectedToken)
            }
        }

        fn peek_invalid_type(&self, _visitor: &Visitor) -> ErrorCode {
            ErrorCode::InvalidType
        }

        fn peek_error(&self, _error_code: ErrorCode) -> ErrorCode {
            ErrorCode::ParseError
        }

        fn fix_position(&self, err: ErrorCode) -> ErrorCode {
            err
        }

        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
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

    let mut deserializer = TestDeserializer {
        input: b"false",
        position: 0,
    };

    let result = deserializer.deserialize_bool(Visitor);
    assert_eq!(result, Ok(false));
}

#[test]
#[should_panic]
fn test_deserialize_bool_invalid() {
    struct Visitor;
    
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = bool;

        fn visit_bool(self, _: bool) -> Result<Self::Value> {
            panic!("Should not be called");
        }
    }

    struct TestDeserializer {
        input: &'static [u8],
        position: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<()> {
            Err(ErrorCode::UnexpectedToken)
        }

        fn peek_invalid_type(&self, _visitor: &Visitor) -> ErrorCode {
            ErrorCode::InvalidType
        }

        fn peek_error(&self, _error_code: ErrorCode) -> ErrorCode {
            ErrorCode::ParseError
        }

        fn fix_position(&self, err: ErrorCode) -> ErrorCode {
            err
        }

        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => return Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
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

    let deserializer = TestDeserializer {
        input: b"unknown",
        position: 0,
    };

    deserializer.deserialize_bool(Visitor);
}

