// Answer 0

#[test]
fn test_parse_any_signed_number_positive() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.index as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.index as u64, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            self.index
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

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = b"42".to_vec();
    let mut deserializer = Deserializer {
        read: MockRead { input, index: 0 },
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result = deserializer.parse_any_signed_number();
    assert!(result.is_ok());
}

#[test]
fn test_parse_any_signed_number_negative() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.index as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.index as u64, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            self.index
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

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = b"-7".to_vec();
    let mut deserializer = Deserializer {
        read: MockRead { input, index: 0 },
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result = deserializer.parse_any_signed_number();
    assert!(result.is_ok());
}

#[test]
fn test_parse_any_signed_number_invalid() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.index as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.index as u64, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            self.index
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

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = b"abc".to_vec();
    let mut deserializer = Deserializer {
        read: MockRead { input, index: 0 },
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result = deserializer.parse_any_signed_number();
    assert!(result.is_err());
}

#[test]
fn test_parse_any_signed_number_eof() {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.index as u64, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.index as u64, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            self.index
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

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = b"".to_vec();
    let mut deserializer = Deserializer {
        read: MockRead { input, index: 0 },
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result = deserializer.parse_any_signed_number();
    assert!(result.is_err());
}

