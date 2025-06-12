// Answer 0

#[test]
fn test_scan_integer128_zero_and_invalid() {
    struct MockReader {
        chars: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                Ok(Some(self.chars[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: Visitor<'static> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input_chars = vec![b'0', b'0'];
    let mut reader = MockReader { chars: input_chars, pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let mut buf = String::new();
    let result = deserializer.scan_integer128(&mut buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().err.code, ErrorCode::InvalidNumber);
}

#[test]
fn test_scan_integer128_valid() {
    struct MockReader {
        chars: Vec<u8>,
        pos: usize,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.chars.len() {
                Ok(Some(self.chars[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: Visitor<'static> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let input_chars = vec![b'1', b'2', b'3'];
    let mut reader = MockReader { chars: input_chars, pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 0 };

    let mut buf = String::new();
    let result = deserializer.scan_integer128(&mut buf);
    
    assert!(result.is_ok());
    assert_eq!(buf, "123");
}

