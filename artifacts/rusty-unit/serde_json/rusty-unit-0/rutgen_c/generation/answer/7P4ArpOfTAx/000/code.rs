// Answer 0

#[test]
fn test_parse_decimal_overflow_no_overflow() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead {
            data: b"12345e10andmoredigits".to_vec(),
            position: 0,
        },
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result = deserializer.parse_decimal_overflow(true, 12345, 10);
    assert!(result.is_ok());
}

#[test]
fn test_parse_decimal_overflow_with_exponent() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead {
            data: b"99999e11andmoredigits".to_vec(),
            position: 0,
        },
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let result = deserializer.parse_decimal_overflow(true, 99999, 11);
    assert!(result.is_ok());
}

