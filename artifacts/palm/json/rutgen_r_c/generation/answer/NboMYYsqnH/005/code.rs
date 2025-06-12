// Answer 0

#[test]
fn test_do_deserialize_u128_valid_case() {
    struct MockVisitor {
        value: Option<u128>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u128;

        fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.index as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.index as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { input: b" 1234".to_vec(), index: 0 },
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.do_deserialize_u128(MockVisitor { value: None });
    assert_eq!(result, Ok(1234));
}

#[test]
#[should_panic]
fn test_do_deserialize_u128_negative_case() {
    struct MockVisitor {
        value: Option<u128>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _value: u128) -> Result<Self::Value, E> {
            Ok(0)
        }
    }

    struct MockRead {
        index: usize,
        input: Vec<u8>,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.index as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.index as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { input: b" -123".to_vec(), index: 0 },
        scratch: vec![],
        remaining_depth: 0,
    };

    let _ = deserializer.do_deserialize_u128(MockVisitor { value: None });
}

#[test]
fn test_do_deserialize_u128_eof_case() {
    struct MockVisitor {
        value: Option<u128>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u128;

        fn visit_u128<E>(self, _value: u128) -> Result<Self::Value, E> {
            Ok(0)
        }
    }

    struct MockRead {
        index: usize,
        input: Vec<u8>,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.index as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.index as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { input: b" ".to_vec(), index: 0 },
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.do_deserialize_u128(MockVisitor { value: None });
    assert!(result.is_err());
}

