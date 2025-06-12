// Answer 0

fn test_do_deserialize_u128_success() {
    struct MockVisitor {
        value: Option<u128>,
    }

    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = u128;

        fn visit_u128(self, value: u128) -> Result<Self::Value> {
            self.value = Some(value);
            Ok(value)
        }
    }

    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            self.position
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_read = MockRead { input: b"  12345".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: vec![], remaining_depth: 0 };

    let visitor = MockVisitor { value: None };
    let result = deserializer.do_deserialize_u128(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 12345);
}

fn test_do_deserialize_u128_invalid() {
    struct MockVisitor;

    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = u128;

        fn visit_u128(self, _value: u128) -> Result<Self::Value> {
            unimplemented!()
        }
    }

    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            self.position
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_read = MockRead { input: b" -12345".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: vec![], remaining_depth: 0 };

    let visitor = MockVisitor;
    let result = deserializer.do_deserialize_u128(visitor);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().err, ErrorCode::NumberOutOfRange);
}

fn test_do_deserialize_u128_eof() {
    struct MockVisitor;

    impl<'any> de::Visitor<'any> for MockVisitor {
        type Value = u128;

        fn visit_u128(self, _value: u128) -> Result<Self::Value> {
            unimplemented!()
        }
    }

    struct MockRead {
        position: usize,
        end_of_input: bool,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.end_of_input {
                Ok(None)
            } else {
                self.position += 1;
                if self.position > 0 {
                    self.end_of_input = true;
                }
                Ok(Some(b'0'))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.end_of_input {
                Ok(None)
            } else {
                Ok(Some(b'0'))
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            self.position
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_read = MockRead { position: 0, end_of_input: false };
    let mut deserializer = Deserializer { read: mock_read, scratch: vec![], remaining_depth: 0 };

    let visitor = MockVisitor;
    let result = deserializer.do_deserialize_u128(visitor);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().err, ErrorCode::EofWhileParsingValue);
}

