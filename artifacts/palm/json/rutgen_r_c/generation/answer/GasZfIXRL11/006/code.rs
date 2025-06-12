// Answer 0

fn test_peek_invalid_type_seq() {
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
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::InvalidEscape, 0, 0))
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut scratch: Vec<u8> = Vec::new();
    let reader = MockRead { data: b"[ ]".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch, remaining_depth: 1 };

    let expected = de::Error::invalid_type(Unexpected::Seq, &());

    let result = deserializer.peek_invalid_type(&());

    assert_eq!(result, expected);
}

fn test_peek_invalid_type_map() {
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
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, self.position)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Err(Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::syntax(ErrorCode::InvalidEscape, 0, 0))
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut scratch: Vec<u8> = Vec::new();
    let reader = MockRead { data: b"{ }".to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch, remaining_depth: 1 };

    let expected = de::Error::invalid_type(Unexpected::Map, &());

    let result = deserializer.peek_invalid_type(&());

    assert_eq!(result, expected);
}

