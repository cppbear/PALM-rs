// Answer 0

fn test_parse_decimal_valid() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(Some(val))
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
            Position::new(0, self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        where V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input = b"12345.6789e2".to_vec();
    let mut reader = MockReader { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 10 };

    let result = deserializer.parse_decimal(true, 12345, 0);
    assert_eq!(result.unwrap(), 12345.6789e2);
}

fn test_parse_decimal_no_digits_after_decimal() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(Some(val))
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
            Position::new(0, self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        where V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input = b"12345.".to_vec();
    let mut reader = MockReader { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 10 };

    let result = deserializer.parse_decimal(true, 12345, 0);
    assert!(result.is_err());
}

fn test_parse_decimal_overflow() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(Some(val))
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
            Position::new(0, self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        where V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input = b"99999999999999999999999999999999999999999999999999999999999999999999999999999999999.1e308".to_vec();
    let mut reader = MockReader { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 10 };

    let result = deserializer.parse_decimal(true, u64::MAX, 0);
    assert!(result.is_err());
}

fn test_parse_decimal_exponent() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(Some(val))
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
            Position::new(0, self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        where V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input = b"12345.6789e-2".to_vec();
    let mut reader = MockReader { input, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 10 };

    let result = deserializer.parse_decimal(true, 12345, 0);
    assert_eq!(result.unwrap(), 12345.6789e-2);
}

