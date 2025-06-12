// Answer 0

#[test]
fn test_ignore_decimal_valid() {
    struct TestReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
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

        fn discard(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
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

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = TestReader { data: b"123.45e+10".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 10 };

    let result = deserializer.ignore_decimal();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_decimal_invalid() {
    struct TestReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(Some(self.data[self.pos - 1]))
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

        fn discard(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos as u64 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
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

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = TestReader { data: b"e10".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 10 };

    let result = deserializer.ignore_decimal();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.err, ErrorCode::InvalidNumber);
    }
}

