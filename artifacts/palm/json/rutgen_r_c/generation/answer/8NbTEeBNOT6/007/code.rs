// Answer 0

fn test_end_seq_ok() {
    struct DummyReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyReader {
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

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
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

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = DummyReader {
        input: vec![b' ', b']'], // Simulate the whitespace followed by closing bracket
        position: 0,
    };
    
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.end_seq();
    assert!(result.is_ok());
}

fn test_end_seq_trailing_comma() {
    struct DummyReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyReader {
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

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
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

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = DummyReader {
        input: vec![b' ', b','], // Simulate the whitespace followed by a comma
        position: 0,
    };

    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.end_seq();
    assert!(result.is_err());
}

fn test_end_seq_trailing_characters() {
    struct DummyReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyReader {
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

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
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

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = DummyReader {
        input: vec![b' ', b'a'], // Simulate the whitespace followed by an unexpected character
        position: 0,
    };

    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.end_seq();
    assert!(result.is_err());
}

fn test_end_seq_eof() {
    struct DummyReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyReader {
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

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u64, 0, 0)
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

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = DummyReader {
        input: vec![], // Simulate EOF
        position: 0,
    };

    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.end_seq();
    assert!(result.is_err());
}

