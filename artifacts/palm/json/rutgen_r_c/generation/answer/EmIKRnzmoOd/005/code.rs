// Answer 0

fn test_scan_integer128_leading_zero() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
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
            Position { line: 1, column: self.position }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut buf = String::new();
    let mut deserializer = Deserializer { read: MockRead { input: vec![b'0'], position: 0 }, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.scan_integer128(&mut buf);
    assert_eq!(result, Ok(()));
}

fn test_scan_integer128_valid_number() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
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
            Position { line: 1, column: self.position }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut buf = String::new();
    let mut deserializer = Deserializer { read: MockRead { input: vec![b'1', b'2', b'3'], position: 0 }, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.scan_integer128(&mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "123");
}

fn test_scan_integer128_invalid_number() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
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
            Position { line: 1, column: self.position }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut buf = String::new();
    let mut deserializer = Deserializer { read: MockRead { input: vec![b'0', b'0'], position: 0 }, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.scan_integer128(&mut buf);
    assert!(result.is_err());
}

