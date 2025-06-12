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

    struct MockDeserializer {
        de: MockDe,
    }

    struct MockDe {
        input: Vec<u8>,
        index: usize,
    }

    impl MockDe {
        fn next_char(&mut self) -> Result<Option<u8>, Error> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<(), Error> {
            let ident_str = std::str::from_utf8(ident).map_err(|_| Error::new_invalid_value())?;
            if ident_str == "rue\"" {
                Ok(())
            } else {
                Err(Error::new_invalid_value())
            }
        }

        fn peek_error(&self, error_code: ErrorCode) -> Error {
            Error::new(error_code)
        }

        fn eat_char(&mut self) {}

        fn read(&self) -> &Self {
            self
        }
    }

    impl MockDe {
        fn parse_str(&mut self, _scratch: &mut Vec<u8>) -> Result<String, Error> {
            Ok("true".to_string())
        }
    }

    let mock_de = MockDe { input: b"true".to_vec(), index: 0 };
    let deserializer = MockDeserializer { de: mock_de };
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

    struct MockDeserializer {
        de: MockDe,
    }

    struct MockDe {
        input: Vec<u8>,
        index: usize,
    }

    impl MockDe {
        fn next_char(&mut self) -> Result<Option<u8>, Error> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<(), Error> {
            let ident_str = std::str::from_utf8(ident).map_err(|_| Error::new_invalid_value())?;
            if ident_str == "alse\"" {
                Ok(())
            } else {
                Err(Error::new_invalid_value())
            }
        }

        fn peek_error(&self, error_code: ErrorCode) -> Error {
            Error::new(error_code)
        }

        fn eat_char(&mut self) {}

        fn read(&self) -> &Self {
            self
        }
    }

    impl MockDe {
        fn parse_str(&mut self, _scratch: &mut Vec<u8>) -> Result<String, Error> {
            Ok("false".to_string())
        }
    }

    let mock_de = MockDe { input: b"false".to_vec(), index: 0 };
    let deserializer = MockDeserializer { de: mock_de };
    let result = deserializer.deserialize_bool(TestVisitor);
    assert_eq!(result, Ok(false));
}

#[test]
#[should_panic]
fn test_deserialize_bool_invalid() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> {
            unreachable!()
        }
    }

    struct MockDeserializer {
        de: MockDe,
    }

    struct MockDe {
        input: Vec<u8>,
        index: usize,
    }

    impl MockDe {
        fn next_char(&mut self) -> Result<Option<u8>, Error> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), Error> {
            Err(Error::new_invalid_value())
        }

        fn eat_char(&mut self) {}

        fn read(&self) -> &Self {
            self
        }

        fn peek_error(&self, _error_code: ErrorCode) -> Error {
            Error::new_invalid_value()
        }
    }

    let mock_de = MockDe { input: b"not_a_bool".to_vec(), index: 0 };
    let deserializer = MockDeserializer { de: mock_de };
    let _result = deserializer.deserialize_bool(TestVisitor);
}

