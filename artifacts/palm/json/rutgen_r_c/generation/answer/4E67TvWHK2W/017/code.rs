// Answer 0

fn test_parse_decimal_valid() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position { /* implementation not needed for test */ }
        fn peek_position(&self) -> Position { /* implementation not needed for test */ }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = MockRead { data: b"3.14".to_vec(), index: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_decimal(true, 314, 1);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value, 3.14);
}

fn test_parse_decimal_no_digits_after_point() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position { /* implementation not needed for test */ }
        fn peek_position(&self) -> Position { /* implementation not needed for test */ }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = MockRead { data: b"3.".to_vec(), index: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_decimal(true, 3, 0);
    assert!(result.is_err());
}

fn test_parse_decimal_overflow() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position { /* implementation not needed for test */ }
        fn peek_position(&self) -> Position { /* implementation not needed for test */ }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = MockRead { data: b"123456789012345678901234567890.12345678901234567890".to_vec(), index: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_decimal(true, 12345678901234567890, -10);
    assert!(result.is_err());
}

