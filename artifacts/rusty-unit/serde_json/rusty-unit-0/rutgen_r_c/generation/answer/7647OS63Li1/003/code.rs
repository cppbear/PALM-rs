// Answer 0

#[test]
fn test_parse_integer_zero() {
    struct TestRead {
        chars: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                let res = Some(self.chars[self.index]);
                self.index += 1;
                Ok(res)
            } else {
                Ok(None)
            }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.index }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.index }
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestRead { chars: vec![b'0'], index: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.parse_integer(true);
    assert!(result.is_ok());
    match result.unwrap() {
        ParserNumber::U64(val) => assert_eq!(val, 0),
        _ => panic!("Expected a U64 with value 0"),
    }
}

#[test]
fn test_parse_integer_non_zero() {
    struct TestRead {
        chars: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                let res = Some(self.chars[self.index]);
                self.index += 1;
                Ok(res)
            } else {
                Ok(None)
            }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.index }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.index }
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestRead { chars: vec![b'1', b'2', b'3'], index: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.parse_integer(true);
    assert!(result.is_ok());
    match result.unwrap() {
        ParserNumber::U64(val) => assert_eq!(val, 123),
        _ => panic!("Expected a U64 with value 123"),
    }
}

#[test]
fn test_parse_integer_invalid_zero() {
    struct TestRead {
        chars: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                let res = Some(self.chars[self.index]);
                self.index += 1;
                Ok(res)
            } else {
                Ok(None)
            }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.index }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.index }
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestRead { chars: vec![b'0', b'0'], index: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.parse_integer(true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::InvalidNumber);
}

#[test]
fn test_parse_integer_overflow() {
    struct TestRead {
        chars: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                let res = Some(self.chars[self.index]);
                self.index += 1;
                Ok(res)
            } else {
                Ok(None)
            }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.chars.len() {
                Ok(Some(self.chars[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.index }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.index }
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: TestRead { chars: vec![b'9', b'9', b'9', b'9', b'9', b'9', b'9', b'9', b'9'], index: 0 },
        scratch: Vec::new(),
        remaining_depth: 8,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.parse_integer(true);
    assert!(result.is_ok());
    match result.unwrap() {
        ParserNumber::F64(_) => {},
        _ => panic!("Expected a F64 due to overflow"),
    }
}

