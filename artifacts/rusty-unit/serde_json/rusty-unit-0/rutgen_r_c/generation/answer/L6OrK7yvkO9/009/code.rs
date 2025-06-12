// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: &[u8]) -> Self {
            MockRead {
                input: input.to_vec(),
                position: 0,
            }
        }

        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let result = self.input[self.position];
                self.position += 1;
                Some(result)
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn peek_position(&self) -> (usize, usize) {
            (1, 1) // Mock position
        }

        fn read_input(&mut self) -> Result<String, Error> {
            Ok(String::from_utf8_lossy(&self.input).to_string())
        }
    }

    struct MockDeserializer<R> {
        read: R,
        scratch: Vec<u8>,
    }

    impl<R> MockDeserializer<R> 
    where
        R: MockRead {
        fn next_char(&mut self) -> Result<Option<u8>, Error> {
            Ok(self.read.next())
        }

        fn eat_char(&mut self) {
            self.read.discard();
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            self.eat_char();
            let peek = self.next_char()?.ok_or(self.peek_error(ErrorCode::EofWhileParsingValue))?;
            let value = match peek {
                b't' => {
                    self.parse_ident(b"rue\"")?;
                    visitor.visit_bool(true)
                }
                b'f' => {
                    self.parse_ident(b"alse\"")?;
                    visitor.visit_bool(false)
                }
                _ => {
                    self.scratch.clear();
                    let s = self.read_input()?;
                    return Err(de::Error::invalid_type(Unexpected::Str(&s), &visitor));
                }
            };
            value.map_err(|err| self.fix_position(err))
        }
    }

    struct MockVisitor;
    impl de::Visitor<'static> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut reader = MockRead::new(b"true");
    let mut deserializer = MockDeserializer {
        read: reader,
        scratch: vec![],
    };
    let value: bool = deserializer.deserialize_bool(MockVisitor).unwrap();
    assert_eq!(value, true);
}

#[test]
fn test_deserialize_bool_false() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: &[u8]) -> Self {
            MockRead {
                input: input.to_vec(),
                position: 0,
            }
        }

        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let result = self.input[self.position];
                self.position += 1;
                Some(result)
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn peek_position(&self) -> (usize, usize) {
            (1, 1) // Mock position
        }

        fn read_input(&mut self) -> Result<String, Error> {
            Ok(String::from_utf8_lossy(&self.input).to_string())
        }
    }

    struct MockDeserializer<R> {
        read: R,
        scratch: Vec<u8>,
    }

    impl<R> MockDeserializer<R> 
    where
        R: MockRead {
        fn next_char(&mut self) -> Result<Option<u8>, Error> {
            Ok(self.read.next())
        }

        fn eat_char(&mut self) {
            self.read.discard();
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            self.eat_char();
            let peek = self.next_char()?.ok_or(self.peek_error(ErrorCode::EofWhileParsingValue))?;
            let value = match peek {
                b't' => {
                    self.parse_ident(b"rue\"")?;
                    visitor.visit_bool(true)
                }
                b'f' => {
                    self.parse_ident(b"alse\"")?;
                    visitor.visit_bool(false)
                }
                _ => {
                    self.scratch.clear();
                    let s = self.read_input()?;
                    return Err(de::Error::invalid_type(Unexpected::Str(&s), &visitor));
                }
            };
            value.map_err(|err| self.fix_position(err))
        }
    }

    struct MockVisitor;
    impl de::Visitor<'static> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut reader = MockRead::new(b"false");
    let mut deserializer = MockDeserializer {
        read: reader,
        scratch: vec![],
    };
    let value: bool = deserializer.deserialize_bool(MockVisitor).unwrap();
    assert_eq!(value, false);
}

#[test]
#[should_panic]
fn test_deserialize_bool_eof() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: &[u8]) -> Self {
            MockRead {
                input: input.to_vec(),
                position: 0,
            }
        }

        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let result = self.input[self.position];
                self.position += 1;
                Some(result)
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn peek_position(&self) -> (usize, usize) {
            (1, 1) // Mock position
        }

        fn read_input(&mut self) -> Result<String, Error> {
            Ok(String::from_utf8_lossy(&self.input).to_string())
        }
    }

    struct MockDeserializer<R> {
        read: R,
        scratch: Vec<u8>,
    }

    impl<R> MockDeserializer<R> 
    where
        R: MockRead {
        fn next_char(&mut self) -> Result<Option<u8>, Error> {
            Ok(self.read.next())
        }

        fn eat_char(&mut self) {
            self.read.discard();
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            self.eat_char();
            let peek = self.next_char()?.ok_or(self.peek_error(ErrorCode::EofWhileParsingValue))?;
            let value = match peek {
                b't' => {
                    self.parse_ident(b"rue\"")?;
                    visitor.visit_bool(true)
                }
                b'f' => {
                    self.parse_ident(b"alse\"")?;
                    visitor.visit_bool(false)
                }
                _ => {
                    self.scratch.clear();
                    let s = self.read_input()?;
                    return Err(de::Error::invalid_type(Unexpected::Str(&s), &visitor));
                }
            };
            value.map_err(|err| self.fix_position(err))
        }
    }

    struct MockVisitor;
    impl de::Visitor<'static> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut reader = MockRead::new(b"");
    let mut deserializer = MockDeserializer {
        read: reader,
        scratch: vec![],
    };
    let _: bool = deserializer.deserialize_bool(MockVisitor).unwrap();
}  

#[test]
#[should_panic]
fn test_deserialize_bool_invalid() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: &[u8]) -> Self {
            MockRead {
                input: input.to_vec(),
                position: 0,
            }
        }

        fn next(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                let result = self.input[self.position];
                self.position += 1;
                Some(result)
            } else {
                None
            }
        }

        fn discard(&mut self) {}

        fn peek_position(&self) -> (usize, usize) {
            (1, 1) // Mock position
        }

        fn read_input(&mut self) -> Result<String, Error> {
            Ok(String::from_utf8_lossy(&self.input).to_string())
        }
    }

    struct MockDeserializer<R> {
        read: R,
        scratch: Vec<u8>,
    }

    impl<R> MockDeserializer<R> 
    where
        R: MockRead {
        fn next_char(&mut self) -> Result<Option<u8>, Error> {
            Ok(self.read.next())
        }

        fn eat_char(&mut self) {
            self.read.discard();
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }

        fn deserialize_bool<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            self.eat_char();
            let peek = self.next_char()?.ok_or(self.peek_error(ErrorCode::EofWhileParsingValue))?;
            let value = match peek {
                b't' => {
                    self.parse_ident(b"rue\"")?;
                    visitor.visit_bool(true)
                }
                b'f' => {
                    self.parse_ident(b"alse\"")?;
                    visitor.visit_bool(false)
                }
                _ => {
                    self.scratch.clear();
                    let s = self.read_input()?;
                    return Err(de::Error::invalid_type(Unexpected::Str(&s), &visitor));
                }
            };
            value.map_err(|err| self.fix_position(err))
        }
    }

    struct MockVisitor;
    impl de::Visitor<'static> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    let mut reader = MockRead::new(b"string");
    let mut deserializer = MockDeserializer {
        read: reader,
        scratch: vec![],
    };
    let _: bool = deserializer.deserialize_bool(MockVisitor).unwrap();
}

