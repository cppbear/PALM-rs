// Answer 0

#[test]
fn test_parse_any_signed_number_negative() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'_> for MockReader {
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

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'_>,
        {
            unimplemented!()
        }
        
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader { input: b"-42".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 10 };

    let result = deserializer.parse_any_signed_number();
    assert!(result.is_ok());
    match result {
        Ok(ParserNumber::I64(value)) => assert_eq!(value, -42),
        _ => panic!("Expected a signed number"),
    }
}

#[test]
fn test_parse_any_signed_number_positive() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'_> for MockReader {
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

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'_>,
        {
            unimplemented!()
        }
        
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader { input: b"123".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 10 };

    let result = deserializer.parse_any_signed_number();
    assert!(result.is_ok());
    match result {
        Ok(ParserNumber::I64(value)) => assert_eq!(value, 123),
        _ => panic!("Expected a signed number"),
    }
}

#[test]
fn test_parse_any_signed_number_invalid() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'_> for MockReader {
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

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'_>,
        {
            unimplemented!()
        }
        
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = MockReader { input: b"abc".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 10 };

    let result = deserializer.parse_any_signed_number();
    assert!(result.is_err());
}

