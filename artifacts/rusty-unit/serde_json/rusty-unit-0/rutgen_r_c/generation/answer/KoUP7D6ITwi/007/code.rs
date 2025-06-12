// Answer 0

#[test]
fn test_deserialize_enum_with_valid_object() {
    struct TestVisitor {
        result: Result<String>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_enum<V>(self, _access: V) -> Result<Self::Value>
        where
            V: VariantAccess<'de>, {
            Ok("valid_enum".to_string())
        }
    }

    struct TestRead {
        buffer: Vec<u8>,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.buffer.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.buffer.remove(0)))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.buffer.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.buffer[0]))
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::new(0, 0) }
        fn peek_position(&self) -> Position { Position::new(0, 0) }
        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::Borrowed("test"))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(b"test"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer::<TestRead> {
        read: TestRead { buffer: b"{\"valid_enum\": 42}".to_vec() },
        scratch: vec![],
        remaining_depth: 1,
    };

    let result = deserializer.deserialize_enum("TestEnum", &["valid_enum"], TestVisitor { result: Ok(String::new()) });
    assert_eq!(result.unwrap(), "valid_enum".to_string());
}

#[test]
fn test_deserialize_enum_with_empty_object() {
    struct TestVisitor {
        result: Result<String>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_enum<V>(self, _access: V) -> Result<Self::Value>
        where
            V: VariantAccess<'de>, {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1))
        }
    }

    struct TestRead {
        buffer: Vec<u8>,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.buffer.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.buffer.remove(0)))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.buffer.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.buffer[0]))
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::new(0, 0) }
        fn peek_position(&self) -> Position { Position::new(0, 0) }
        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::Borrowed("test"))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(b"test"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer::<TestRead> {
        read: TestRead { buffer: b"{ }".to_vec() },
        scratch: vec![],
        remaining_depth: 1,
    };

    let result = deserializer.deserialize_enum("TestEnum", &["valid_enum"], TestVisitor { result: Ok(String::new()) });
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().code, ErrorCode::ExpectedSomeValue);
}

#[test]
fn test_deserialize_enum_invalid_input() {
    struct TestVisitor {
        result: Result<String>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_enum<V>(self, _access: V) -> Result<Self::Value>
        where
            V: VariantAccess<'de>, {
            Ok("valid_enum".to_string())
        }
    }

    struct TestRead {
        buffer: Vec<u8>,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.buffer.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.buffer.remove(0)))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.buffer.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.buffer[0]))
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::new(0, 0) }
        fn peek_position(&self) -> Position { Position::new(0, 0) }
        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::Borrowed("test"))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::Borrowed(b"test"))
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer::<TestRead> {
        read: TestRead { buffer: b"random string".to_vec() },
        scratch: vec![],
        remaining_depth: 1,
    };

    let result = deserializer.deserialize_enum("TestEnum", &["valid_enum"], TestVisitor { result: Ok(String::new()) });
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().code, ErrorCode::ExpectedSomeValue);
}

