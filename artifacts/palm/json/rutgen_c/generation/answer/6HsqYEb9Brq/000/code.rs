// Answer 0

#[test]
fn test_ignore_integer_single_zero() {
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
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { todo!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { todo!() }
        fn ignore_str(&mut self) -> Result<()> { todo!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { todo!() }
    }

    let mut reader = MockRead { data: vec![b'0'], pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_multiple_leading_zero() {
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
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { todo!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { todo!() }
        fn ignore_str(&mut self) -> Result<()> { todo!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { todo!() }
    }

    let mut reader = MockRead { data: vec![b'0', b'0'], pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_valid() {
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
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { todo!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { todo!() }
        fn ignore_str(&mut self) -> Result<()> { todo!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { todo!() }
    }

    let mut reader = MockRead { data: vec![b'1', b'2', b'3', b'e'], pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.ignore_integer();
    assert!(result.is_ok());
}

