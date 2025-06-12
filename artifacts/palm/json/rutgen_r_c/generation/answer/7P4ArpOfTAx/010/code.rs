// Answer 0

fn test_parse_decimal_overflow_with_digit_only() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader::new(vec![b'1', b'2', b'3', b'e', b'4']);
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.parse_decimal_overflow(true, 123, 4);
    assert!(result.is_ok());
}

fn test_parse_decimal_overflow_with_exponent() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader::new(vec![b'1', b'2', b'3', b'4', b'e', b'5']);
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.parse_decimal_overflow(true, 1234, 5);
    assert!(result.is_ok());
}

fn test_parse_decimal_overflow_with_large_significand() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) { unimplemented!() }
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader::new(vec![b'9', b'9', b'9', b'9', b'9', b'e', b'3']);
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.parse_decimal_overflow(true, 99999, 3);
    assert!(result.is_ok());
}

