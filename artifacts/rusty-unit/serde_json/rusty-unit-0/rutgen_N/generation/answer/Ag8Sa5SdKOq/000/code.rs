// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestDeserializer {
        fn new(input: &[u8]) -> Self {
            Self {
                input: input.to_vec(),
                pos: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ()> {
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn parse_ident(&mut self, expected: &[u8]) -> Result<(), ()> {
            for &byte in expected {
                if self.pos < self.input.len() && self.input[self.pos] == byte {
                    self.pos += 1;
                } else {
                    return Err(());
                }
            }
            Ok(())
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> () { }

        fn peek_error(&self, _code: ErrorCode) -> () { }

        fn fix_position(&self, err: ()) -> () {
            // For simplicity, we just return the error unmodified.
        }
    }

    impl TestDeserializer {
        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, ()>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                Ok(None) => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
                Err(_) => return Err(()),
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

    let deserializer = TestDeserializer::new(b"true");
    let result = deserializer.deserialize_bool(TestVisitor);

    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_bool_false() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestDeserializer {
        fn new(input: &[u8]) -> Self {
            Self {
                input: input.to_vec(),
                pos: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ()> {
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn parse_ident(&mut self, expected: &[u8]) -> Result<(), ()> {
            for &byte in expected {
                if self.pos < self.input.len() && self.input[self.pos] == byte {
                    self.pos += 1;
                } else {
                    return Err(());
                }
            }
            Ok(())
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> () { }

        fn peek_error(&self, _code: ErrorCode) -> () { }

        fn fix_position(&self, err: ()) -> () {
            // For simplicity, we just return the error unmodified.
        }
    }

    impl TestDeserializer {
        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, ()>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                Ok(None) => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
                Err(_) => return Err(()),
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

    let deserializer = TestDeserializer::new(b"false");
    let result = deserializer.deserialize_bool(TestVisitor);

    assert_eq!(result, Ok(false));
}

#[test]
#[should_panic]
fn test_deserialize_bool_invalid() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestDeserializer {
        fn new(input: &[u8]) -> Self {
            Self {
                input: input.to_vec(),
                pos: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>, ()> {
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.pos += 1;
        }

        fn parse_ident(&mut self, expected: &[u8]) -> Result<(), ()> {
            for &byte in expected {
                if self.pos < self.input.len() && self.input[self.pos] == byte {
                    self.pos += 1;
                } else {
                    return Err(());
                }
            }
            Ok(())
        }

        fn peek_invalid_type<V>(&self, _visitor: &V) -> () { }

        fn peek_error(&self, _code: ErrorCode) -> () { }

        fn fix_position(&self, err: ()) -> () {
            // For simplicity, we just return the error unmodified.
        }
    }

    impl TestDeserializer {
        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, ()>
        where
            V: de::Visitor<'de>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                Ok(None) => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
                Err(_) => return Err(()),
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

    let deserializer = TestDeserializer::new(b"invalid");
    deserializer.deserialize_bool(TestVisitor).unwrap();
}

