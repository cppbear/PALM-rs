// Answer 0

fn test_ignore_value_empty() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
        
        fn position(&self) -> Position { Position { line: 1, column: self.position as u32 + 1 } }
        fn peek_position(&self) -> Position { self.position() }
        fn byte_offset(&self) -> usize { self.position }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut read = TestRead { data: vec![], position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 128 };
    
    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

fn test_ignore_value_true() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
        
        fn position(&self) -> Position { Position { line: 1, column: self.position as u32 + 1 } }
        fn peek_position(&self) -> Position { self.position() }
        fn byte_offset(&self) -> usize { self.position }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut read = TestRead { data: b"true".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 128 };
    
    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

fn test_ignore_value_number() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
        
        fn position(&self) -> Position { Position { line: 1, column: self.position as u32 + 1 } }
        fn peek_position(&self) -> Position { self.position() }
        fn byte_offset(&self) -> usize { self.position }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut read = TestRead { data: b"12345".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 128 };
    
    let result = deserializer.ignore_value();
    assert!(result.is_ok());
}

fn test_ignore_value_invalid_key() {
    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
        
        fn position(&self) -> Position { Position { line: 1, column: self.position as u32 + 1 } }
        fn peek_position(&self) -> Position { self.position() }
        fn byte_offset(&self) -> usize { self.position }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut read = TestRead { data: b"{:".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 128 };
    
    let result = deserializer.ignore_value();
    assert!(result.is_err());
}

