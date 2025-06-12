// Answer 0

fn test_parse_decimal_valid() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let byte = self.data[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")] fn begin_raw_buffering(&mut self) {}
        #[cfg(feature = "raw_value")] fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mock_read = MockRead { data: vec![b'1', b'2', b'3', b'.', b'4', b'5'], pos: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_decimal(true, 123, 0);
    assert!(result.is_ok());
}

fn test_parse_decimal_no_digits_after_decimal() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let byte = self.data[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")] fn begin_raw_buffering(&mut self) {}
        #[cfg(feature = "raw_value")] fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mock_read = MockRead { data: vec![b'1', b'.'], pos: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_decimal(true, 123, 0);
    assert!(result.is_err());
}

fn test_parse_decimal_peek_error() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")] fn begin_raw_buffering(&mut self) {}
        #[cfg(feature = "raw_value")] fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: Visitor<'de> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mock_read = MockRead { data: vec![], pos: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_decimal(true, 0, 0);
    assert!(result.is_err());
}

