// Answer 0

#[test]
fn test_ignore_value_empty_object() {
    struct MockRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos >= self.input.len() {
                Ok(None)
            } else {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.pos]))
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.pos, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.pos, 0)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut read = MockRead {
        input: b"{}".to_vec(),
        pos: 0,
    };
    let mut deserializer = Deserializer {
        read,
        scratch: Vec::new(),
        remaining_depth: 128,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_value_empty_array() {
    struct MockRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos >= self.input.len() {
                Ok(None)
            } else {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.pos]))
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.pos, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.pos, 0)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut read = MockRead {
        input: b"[]".to_vec(),
        pos: 0,
    };
    let mut deserializer = Deserializer {
        read,
        scratch: Vec::new(),
        remaining_depth: 128,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_value_invalid_key() {
    struct MockRead {
        input: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos >= self.input.len() {
                Ok(None)
            } else {
                let byte = self.input[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos >= self.input.len() {
                Ok(None)
            } else {
                Ok(Some(self.input[self.pos]))
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.pos, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.pos, 0)
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut read = MockRead {
        input: b"{invalid}".to_vec(),
        pos: 0,
    };
    let mut deserializer = Deserializer {
        read,
        scratch: Vec::new(),
        remaining_depth: 128,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_err());
}

