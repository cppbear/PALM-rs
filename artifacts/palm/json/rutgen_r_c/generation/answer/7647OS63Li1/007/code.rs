// Answer 0

#[test]
fn test_parse_integer_valid_single_digit() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(Some(val))
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

        fn position(&self) -> Position { Position::new(0, 0) }

        fn peek_position(&self) -> Position { Position::new(0, 0) }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { input: b"3".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };
    assert_eq!(deserializer.parse_integer(true), Ok(ParserNumber::U64(3)));
}

#[test]
fn test_parse_integer_valid_multi_digit() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(Some(val))
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

        fn position(&self) -> Position { Position::new(0, 0) }

        fn peek_position(&self) -> Position { Position::new(0, 0) }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { input: b"123".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    assert_eq!(deserializer.parse_integer(true), Ok(ParserNumber::U64(123)));
}

#[test]
fn test_parse_integer_leading_zero() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(Some(val))
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

        fn position(&self) -> Position { Position::new(0, 0) }

        fn peek_position(&self) -> Position { Position::new(0, 0) }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { input: b"00".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    assert_eq!(deserializer.parse_integer(true), Err(ErrorCode::InvalidNumber));
}

#[test]
fn test_parse_integer_eof() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(Some(val))
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

        fn position(&self) -> Position { Position::new(0, 0) }

        fn peek_position(&self) -> Position { Position::new(0, 0) }

        fn byte_offset(&self) -> usize { self.position }

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { input: b"".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: vec![], remaining_depth: 0 };

    assert_eq!(deserializer.parse_integer(true), Err(ErrorCode::EofWhileParsingValue));
}

