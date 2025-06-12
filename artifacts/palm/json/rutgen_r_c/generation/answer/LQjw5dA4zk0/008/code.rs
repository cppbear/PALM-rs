// Answer 0

#[test]
fn test_parse_number_positive_integer() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockReader {
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
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut reader = MockReader { data: vec![b'1'], pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_number(true, 42);
    assert!(result.is_ok());
    if let Ok(ParserNumber::U64(value)) = result {
        assert_eq!(value, 42);
    } else {
        panic!("Expected U64 variant");
    }
}

#[test]
fn test_parse_number_negative_integer() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockReader {
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
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut reader = MockReader { data: vec![b'-', b'1'], pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_number(false, 42);
    assert!(result.is_ok());
    if let Ok(ParserNumber::I64(value)) = result {
        assert_eq!(value, -42);
    } else {
        panic!("Expected I64 variant");
    }
}

#[test]
fn test_parse_number_float() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for MockReader {
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
        fn position(&self) -> Position { Position::default() }
        fn peek_position(&self) -> Position { Position::default() }
        fn byte_offset(&self) -> usize { self.pos }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut reader = MockReader { data: vec![b'.', b'5', b'e', b'2'], pos: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    let result = deserializer.parse_number(true, 5);
    assert!(result.is_ok());
    if let Ok(ParserNumber::F64(value)) = result {
        assert_eq!(value, 5.0 * 10.0_f64.powi(2));
    } else {
        panic!("Expected F64 variant");
    }
}

