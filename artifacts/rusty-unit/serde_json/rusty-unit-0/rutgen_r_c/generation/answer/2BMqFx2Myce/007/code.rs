// Answer 0

fn test_parse_whitespace_all_spaces() {
    struct StubReader {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for StubReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
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

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.index }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) { }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) { }
    }

    let mut deserializer = Deserializer {
        read: StubReader { data: vec![b' ', b'\n', b'\t'], index: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.parse_whitespace();
    assert_eq!(result, Ok(Some(b' ')));
}

fn test_parse_whitespace_all_newlines() {
    struct StubReader {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for StubReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
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

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.index }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) { }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) { }
    }

    let mut deserializer = Deserializer {
        read: StubReader { data: vec![b'\n', b'\r', b'\t'], index: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.parse_whitespace();
    assert_eq!(result, Ok(Some(b'\n')));
}

fn test_parse_whitespace_mixed_with_eof() {
    struct StubReader {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for StubReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
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

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.index }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) { }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) { }
    }

    let mut deserializer = Deserializer {
        read: StubReader { data: vec![b' ', b'a'], index: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.parse_whitespace();
    assert_eq!(result, Ok(Some(b' ')));
}

fn test_parse_whitespace_no_whitespace() {
    struct StubReader {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for StubReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
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

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { self.index }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) { }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) { }
    }

    let mut deserializer = Deserializer {
        read: StubReader { data: vec![b'a', b'b', b'c'], index: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.parse_whitespace();
    assert_eq!(result, Ok(None));
}

