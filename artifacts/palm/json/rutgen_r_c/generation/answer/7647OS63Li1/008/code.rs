// Answer 0

fn test_parse_integer_valid_single_digit() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let byte = self.data[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_read = MockRead { data: vec![b'7'], pos: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 5 };

    let result = deserializer.parse_integer(true);
    assert_eq!(result.unwrap(), ParserNumber::U64(7));
}

fn test_parse_integer_with_leading_zero() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let byte = self.data[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_read = MockRead { data: vec![b'0', b'1'], pos: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 5 };

    let result = deserializer.parse_integer(true);
    assert_eq!(result.unwrap(), ParserNumber::U64(1));
}

fn test_parse_integer_invalid() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let byte = self.data[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }
        
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_read = MockRead { data: vec![b'0', b'0'], pos: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 5 };

    let result = deserializer.parse_integer(true);
    assert!(result.is_err());
}

