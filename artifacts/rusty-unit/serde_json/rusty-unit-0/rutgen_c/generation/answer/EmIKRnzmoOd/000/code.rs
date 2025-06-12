// Answer 0

#[test]
fn test_scan_integer128_valid_number() {
    struct TestRead {
        buffer: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                let byte = self.buffer[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                Ok(Some(self.buffer[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
        
        fn position(&self) -> Position {
            // Mock implementation
            Position { line: 1, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            // Mock implementation
            Position { line: 1, column: self.position as u32 }
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

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = TestRead {
        buffer: b"1234567890".to_vec(),
        position: 0,
    };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let mut buf = String::new();
    let result = deserializer.scan_integer128(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "1234567890");
}

#[test]
fn test_scan_integer128_leading_zero() {
    struct TestRead {
        buffer: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                let byte = self.buffer[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                Ok(Some(self.buffer[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
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

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = TestRead {
        buffer: b"0".to_vec(),
        position: 0,
    };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let mut buf = String::new();
    let result = deserializer.scan_integer128(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "0");
}

#[test]
#[should_panic]
fn test_scan_integer128_invalid_number() {
    struct TestRead {
        buffer: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                let byte = self.buffer[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                Ok(Some(self.buffer[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
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

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = TestRead {
        buffer: b"0123".to_vec(),
        position: 0,
    };
    let mut deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 0 };
    let mut buf = String::new();
    deserializer.scan_integer128(&mut buf).unwrap();
}

