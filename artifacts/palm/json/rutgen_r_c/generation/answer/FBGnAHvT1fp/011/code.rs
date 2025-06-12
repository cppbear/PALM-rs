// Answer 0

fn test_ignore_decimal_valid_number() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let ch = self.data[self.position];
                self.position += 1;
                Ok(Some(ch))
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

        fn position(&self) -> Position {
            Position::new(self.position as u64)
        }
        
        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }
    
    let mut mock_reader = MockRead {
        data: vec![b'1', b'2', b'3', b'e', b'4'],
        position: 0,
    };
    
    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    
    let result = deserializer.ignore_decimal();
    assert!(result.is_ok());
}


fn test_ignore_decimal_no_digits() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let ch = self.data[self.position];
                self.position += 1;
                Ok(Some(ch))
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

        fn position(&self) -> Position {
            Position::new(self.position as u64)
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }
    
    let mut mock_reader = MockRead {
        data: vec![b'.', b'e', b'4'],
        position: 0,
    };
    
    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    
    let result = deserializer.ignore_decimal();
    assert!(result.is_err());
}


fn test_ignore_decimal_valid_exponent() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let ch = self.data[self.position];
                self.position += 1;
                Ok(Some(ch))
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

        fn position(&self) -> Position {
            Position::new(self.position as u64)
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
            todo!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_reader = MockRead {
        data: vec![b'4', b'5', b'e', b'6'],
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.ignore_decimal();
    assert!(result.is_ok());
}

