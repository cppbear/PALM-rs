// Answer 0

fn test_scan_integer128_valid() {
    struct TestReader {
        input: Vec<u8>,
        index: usize,
    }
    
    impl<'de> Read<'de> for TestReader {
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
            todo!()
        }

        fn peek_position(&self) -> Position {
            todo!()
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            todo!()
        }
    }

    let mut reader = TestReader {
        input: vec![b'1', b'2', b'3'],
        index: 0,
    };
    
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let mut buf = String::new();
    
    let result = deserializer.scan_integer128(&mut buf);
    
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "123");
}

fn test_scan_integer128_invalid_leading_zero() {
    struct TestReader {
        input: Vec<u8>,
        index: usize,
    }
    
    impl<'de> Read<'de> for TestReader {
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
            todo!()
        }

        fn peek_position(&self) -> Position {
            todo!()
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            todo!()
        }
    }

    let mut reader = TestReader {
        input: vec![b'0', b'0', b'1'],
        index: 0,
    };
    
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let mut buf = String::new();
    
    let result = deserializer.scan_integer128(&mut buf);
    
    assert!(result.is_err());
}

fn test_scan_integer128_invalid_non_numeric() {
    struct TestReader {
        input: Vec<u8>,
        index: usize,
    }
    
    impl<'de> Read<'de> for TestReader {
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
            todo!()
        }

        fn peek_position(&self) -> Position {
            todo!()
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            todo!()
        }
    }
    
    let mut reader = TestReader {
        input: vec![b'x'],
        index: 0,
    };
    
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let mut buf = String::new();
    
    let result = deserializer.scan_integer128(&mut buf);
    
    assert!(result.is_err());
}

