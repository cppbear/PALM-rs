// Answer 0

fn test_peek_invalid_type_for_opening_brace() {
    struct DummyRead {
        buffer: Vec<u8>,
        position: usize,
    }

    impl Read<'_> for DummyRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                self.position += 1;
                Ok(Some(self.buffer[self.position - 1]))
            } else {
                Ok(None)
            }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                Ok(Some(self.buffer[self.position]))
            } else {
                Ok(None)
            }
        }
        
        fn discard(&mut self) { 
            self.position += 1; 
        }
        
        fn position(&self) -> Position {
            Position::new(self.position, 0) // Mocked position
        }
        
        fn peek_position(&self) -> Position {
            Position::new(self.position, 0) // Mocked position
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            Ok(Reference::Borrowed("mock"))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            Ok(Reference::Borrowed(b"mock"))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let read = DummyRead {
        buffer: vec![b'{'],
        position: 0,
    };

    let mut deserializer = Deserializer {
        read,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let expected_error_code = ErrorCode::ExpectedSomeValue;

    let result = deserializer.peek_invalid_type(&Expected::Map);

    assert_eq!(result.err.code(), expected_error_code);
}

fn test_peek_invalid_type_for_unexpected_character() {
    struct DummyRead {
        buffer: Vec<u8>,
        position: usize,
    }

    impl Read<'_> for DummyRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                self.position += 1;
                Ok(Some(self.buffer[self.position - 1]))
            } else {
                Ok(None)
            }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                Ok(Some(self.buffer[self.position]))
            } else {
                Ok(None)
            }
        }
        
        fn discard(&mut self) { 
            self.position += 1; 
        }
        
        fn position(&self) -> Position {
            Position::new(self.position, 0) // Mocked position
        }
        
        fn peek_position(&self) -> Position {
            Position::new(self.position, 0) // Mocked position
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            Ok(Reference::Borrowed("mock"))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            Ok(Reference::Borrowed(b"mock"))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let read = DummyRead {
        buffer: vec![b'!'],
        position: 0,
    };

    let mut deserializer = Deserializer {
        read,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    let expected_error_code = ErrorCode::ExpectedSomeValue;

    let result = deserializer.peek_invalid_type(&Expected::Map);

    assert_eq!(result.err.code(), expected_error_code);
}

