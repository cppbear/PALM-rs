// Answer 0

fn test_ignore_integer_valid() {
    struct MockRead<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'de> Read<'de> for MockRead<'de> {
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

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
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
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"12345", position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_integer();
    assert!(result.is_ok());
}

fn test_ignore_integer_leading_zero() {
    struct MockRead<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'de> Read<'de> for MockRead<'de> {
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

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
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
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"0123", position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

fn test_ignore_integer_invalid_character() {
    struct MockRead<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'de> Read<'de> for MockRead<'de> {
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

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
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
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"abc", position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

