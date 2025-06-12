// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct Visitor;

    impl de::Visitor<'static> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: &'static [u8],
        index: usize,
    }

    impl TestDeserializer {
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

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            if self.index + ident.len() > self.input.len() {
                return Err(ErrorCode::EofWhileParsingValue);
            }
            for &b in ident {
                if self.input[self.index] != b {
                    return Err(ErrorCode::InvalidIdent);
                }
                self.index += 1;
            }
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Error occurred")
        }

        fn peek_invalid_type(&self, _visitor: &Visitor) -> Error {
            Error::new("Invalid type")
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool(&mut self, visitor: Visitor) -> Result<Visitor::Value> {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => {
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
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = TestDeserializer { input: b" true", index: 0 };
    let result = deserializer.deserialize_bool(Visitor);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_bool_false() {
    struct Visitor;

    impl de::Visitor<'static> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: &'static [u8],
        index: usize,
    }

    impl TestDeserializer {
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

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            // Simulate an error on parsing "false"
            if self.index + ident.len() > self.input.len() {
                return Err(ErrorCode::EofWhileParsingValue);
            }
            return Err(ErrorCode::InvalidIdent);
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Error occurred")
        }

        fn peek_invalid_type(&self, _visitor: &Visitor) -> Error {
            Error::new("Invalid type")
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool(&mut self, visitor: Visitor) -> Result<Visitor::Value> {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => {
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
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = TestDeserializer { input: b" f", index: 0 };
    let result = deserializer.deserialize_bool(Visitor);
    assert!(result.is_err());
} 

#[test]
#[should_panic]
fn test_deserialize_bool_invalid_type() {
    struct Visitor;

    impl de::Visitor<'static> for Visitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: &'static [u8],
        index: usize,
    }

    impl TestDeserializer {
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

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            if self.index + ident.len() > self.input.len() {
                return Err(ErrorCode::EofWhileParsingValue);
            }
            for &b in ident {
                if self.input[self.index] != b {
                    return Err(ErrorCode::InvalidIdent);
                }
                self.index += 1;
            }
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Error occurred")
        }

        fn peek_invalid_type(&self, _visitor: &Visitor) -> Error {
            panic!("Test for invalid types");
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool(&mut self, visitor: Visitor) -> Result<Visitor::Value> {
            let peek = match self.parse_whitespace()? {
                Some(b) => b,
                None => {
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
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = TestDeserializer { input: b"x", index: 0 };
    deserializer.deserialize_bool(Visitor).unwrap();
}

