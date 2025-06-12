// Answer 0

fn test_ignore_value_success() {
    struct MockRead {
        data: Vec<u8>,
        current: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.current < self.data.len() {
                let byte = self.data[self.current];
                self.current += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.current < self.data.len() {
                Ok(Some(self.data[self.current]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.current < self.data.len() {
                self.current += 1;
            }
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.current }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"null".to_vec(), current: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

fn test_ignore_value_invalid_input() {
    struct MockRead {
        data: Vec<u8>,
        current: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.current < self.data.len() {
                let byte = self.data[self.current];
                self.current += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.current < self.data.len() {
                Ok(Some(self.data[self.current]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.current < self.data.len() {
                self.current += 1;
            }
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.current }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"invalid".to_vec(), current: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_err());
}

fn test_ignore_value_empty_input() {
    struct MockRead {
        data: Vec<u8>,
        current: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.current < self.data.len() {
                let byte = self.data[self.current];
                self.current += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.current < self.data.len() {
                Ok(Some(self.data[self.current]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.current < self.data.len() {
                self.current += 1;
            }
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.current }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: Vec::new(), current: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_err());
}

fn test_ignore_value_with_enclosing() {
    struct MockRead {
        data: Vec<u8>,
        current: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.current < self.data.len() {
                let byte = self.data[self.current];
                self.current += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.current < self.data.len() {
                Ok(Some(self.data[self.current]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.current < self.data.len() {
                self.current += 1;
            }
        }

        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.current }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: b"[1, 2, 3]".to_vec(), current: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
    };

    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

