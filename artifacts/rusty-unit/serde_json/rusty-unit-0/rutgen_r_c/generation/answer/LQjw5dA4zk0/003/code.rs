// Answer 0

#[test]
fn test_parse_number_positive_case_with_decimal() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Ok(Some(val))
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
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.pos }
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
    }

    let mut reader = MockReader { data: b"123.45".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 10 };

    let result = deserializer.parse_number(false, 123);
    assert!(result.is_ok());
    if let Ok(ParserNumber::F64(value)) = result {
        assert_eq!(value, -123.0);
    }
}

#[test]
fn test_parse_number_negative_case() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Ok(Some(val))
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
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.pos }
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
    }

    let mut reader = MockReader { data: b"123e10".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 10 };

    let result = deserializer.parse_number(false, 123);
    assert!(result.is_ok());
    if let Ok(ParserNumber::F64(value)) = result {
        assert_eq!(value, -123.0);
    }
}

#[test]
fn test_parse_number_zero_case() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Ok(Some(val))
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
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.pos }
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
    }

    let mut reader = MockReader { data: b"0].345".to_vec(), pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 10 };

    let result = deserializer.parse_number(false, 0);
    assert!(result.is_ok());
    if let Ok(ParserNumber::F64(value)) = result {
        assert_eq!(value, 0.0);
    }
}

