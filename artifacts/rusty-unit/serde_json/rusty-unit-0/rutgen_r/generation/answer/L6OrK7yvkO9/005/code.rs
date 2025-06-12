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
        de: TestDe,
    }

    struct TestDe {
        chars: Vec<u8>,
        pos: usize,
    }

    impl TestDe {
        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                let ch = self.chars[self.pos];
                self.pos += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            if self.chars[self.pos - 1] == ident[0] as u8 {
                Ok(())
            } else {
                Err(de::Error::custom("failed to parse ident"))
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> de::Error {
            de::Error::custom("EOF error")
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    let mut deserializer = TestDeserializer { de: TestDe { chars: b"true".to_vec(), pos: 0 } };
    let result = deserializer.deserialize_bool(TestVisitor);

    assert_eq!(result, Ok(true));
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
        de: TestDe,
    }

    struct TestDe {
        chars: Vec<u8>,
        pos: usize,
    }

    impl TestDe {
        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                let ch = self.chars[self.pos];
                self.pos += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            if self.chars[self.pos - 1] == ident[0] as u8 {
                Ok(())
            } else {
                Err(de::Error::custom("failed to parse ident"))
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> de::Error {
            de::Error::custom("EOF error")
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    let mut deserializer = TestDeserializer { de: TestDe { chars: b"false".to_vec(), pos: 0 } };
    let result = deserializer.deserialize_bool(TestVisitor);

    assert_eq!(result, Ok(false));
}

#[test]
fn test_deserialize_bool_eof() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            Err(de::Error::custom("this shouldn't be called"))
        }
    }

    struct TestDeserializer {
        de: TestDe,
    }

    struct TestDe {
        chars: Vec<u8>,
        pos: usize,
    }

    impl TestDe {
        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                let ch = self.chars[self.pos];
                self.pos += 1;
                Ok(Some(ch))
            } else {
                Err(de::Error::custom("next char error"))
            }
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Err(de::Error::custom("failed to parse ident"))
        }

        fn peek_error(&self, _code: ErrorCode) -> de::Error {
            de::Error::custom("EOF error")
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }
    }

    let mut deserializer = TestDeserializer { de: TestDe { chars: b"t".to_vec(), pos: 0 } };
    let result = deserializer.deserialize_bool(TestVisitor);

    assert!(result.is_err());
}

#[test]
fn test_deserialize_bool_invalid_type() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, _value: bool) -> Result<Self::Value> {
            Err(de::Error::custom("this shouldn't be called"))
        }
    }

    struct TestDeserializer {
        de: TestDe,
    }

    struct TestDe {
        chars: Vec<u8>,
        pos: usize,
        scratch: String,
    }

    impl TestDe {
        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                let ch = self.chars[self.pos];
                self.pos += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Err(de::Error::custom("failed to parse ident"))
        }

        fn peek_error(&self, _code: ErrorCode) -> de::Error {
            de::Error::custom("EOF error")
        }

        fn fix_position(&self, err: de::Error) -> de::Error {
            err
        }

        fn read(&mut self, _buf: &mut String) -> Result<&str> {
            self.scratch.push_str("invalid");
            Ok(&self.scratch)
        }
    }

    let mut deserializer = TestDeserializer { de: TestDe { chars: b"invalid_value".to_vec(), pos: 0, scratch: String::new() } };
    let result = deserializer.deserialize_bool(TestVisitor);

    assert!(result.is_err());
}

