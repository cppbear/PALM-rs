// Answer 0

fn test_parse_exponent_positive() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
            Position { line: 0, column: self.position as u32 }
        }
        
        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = b"e+123".to_vec();
    let mut read = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.parse_exponent(true, 42, 0);
    assert_eq!(result, Ok(42e123));
}

fn test_parse_exponent_negative() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
            Position { line: 0, column: self.position as u32 }
        }
        
        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = b"e-123".to_vec();
    let mut read = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.parse_exponent(true, 42, 0);
    assert_eq!(result, Ok(42e-123));
}

fn test_parse_exponent_invalid() {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
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
            Position { line: 0, column: self.position as u32 }
        }
        
        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input = b"e+".to_vec();
    let mut read = TestRead { input, position: 0 };
    let mut deserializer = Deserializer { read, scratch: vec![], remaining_depth: 1 };

    let result = deserializer.parse_exponent(true, 42, 0);
    assert!(result.is_err());
}

