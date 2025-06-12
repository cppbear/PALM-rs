// Answer 0

#[test]
fn test_next_char_or_null_with_zero() {
    struct TestReader {
        chars: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, position: 0 }
        }
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.chars.len() {
                let result = self.chars[self.position];
                self.position += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.chars.len() {
                Ok(Some(self.chars[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader::new(vec![0]);
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.next_char_or_null();
}

#[test]
fn test_next_char_or_null_with_max_value() {
    struct TestReader {
        chars: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, position: 0 }
        }
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.chars.len() {
                let result = self.chars[self.position];
                self.position += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.chars.len() {
                Ok(Some(self.chars[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader::new(vec![255]);
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.next_char_or_null();
}

#[test]
fn test_next_char_or_null_with_multiple_values() {
    struct TestReader {
        chars: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(chars: Vec<u8>) -> Self {
            Self { chars, position: 0 }
        }
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.chars.len() {
                let result = self.chars[self.position];
                self.position += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.chars.len() {
                Ok(Some(self.chars[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader::new(vec![100, 200, 255]);
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    let _ = deserializer.next_char_or_null();
    let _ = deserializer.next_char_or_null();
    let _ = deserializer.next_char_or_null();
}

