// Answer 0

fn test_parse_exponent_valid_positive() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockRead {
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

        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: 0 } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut reader = MockRead { input: b"e+10".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.parse_exponent(true, 12345, 0).unwrap();
    
    assert_eq!(result, 12345e10);
}

fn test_parse_exponent_valid_negative() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockRead {
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

        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: 0 } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut reader = MockRead { input: b"e-10".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    
    let result = deserializer.parse_exponent(true, 12345, 0).unwrap();
    
    assert_eq!(result, 12345e-10);
}

fn test_parse_exponent_invalid_character() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockRead {
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

        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: 0 } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut reader = MockRead { input: b"eA10".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };
    
    let result = deserializer.parse_exponent(true, 12345, 0);
    
    assert!(result.is_err());
} 

fn test_parse_exponent_overflow() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockRead {
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

        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: 0 } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut reader = MockRead { input: b"e+2147483648".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let result = deserializer.parse_exponent(true, 12345, 0);
    
    assert!(result.is_err());
}

