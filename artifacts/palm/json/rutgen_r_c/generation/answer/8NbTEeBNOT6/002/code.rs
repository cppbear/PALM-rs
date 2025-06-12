// Answer 0

fn test_end_seq_handle_trailing_comma() {
    struct MockRead {
        pos: usize,
        input: Vec<u8>,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                self.pos += 1;
                Ok(Some(self.input[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(self.pos as u64, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.pos as u64, 0)
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
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mock_reader = MockRead {
        pos: 0,
        input: vec![b',', b']'], // simulating a comma followed by a closing bracket
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    // This should return Err with ErrorCode::TrailingComma since we encounter a `]` after a `,`
    let result = deserializer.end_seq();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().err, ErrorCode::TrailingComma);
}

fn test_end_seq_handle_trailing_characters() {
    struct MockRead {
        pos: usize,
        input: Vec<u8>,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                self.pos += 1;
                Ok(Some(self.input[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(self.pos as u64, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.pos as u64, 0)
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
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mock_reader = MockRead {
        pos: 0,
        input: vec![b'a', b']'], // simulating a non-whitespace character before closing bracket
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    // This should return Err with ErrorCode::TrailingCharacters since we encounter a non-whitespace character before `]`
    let result = deserializer.end_seq();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().err, ErrorCode::TrailingCharacters);
}

fn test_end_seq_handle_eof() {
    struct MockRead {
        pos: usize,
        input: Vec<u8>,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                self.pos += 1;
                Ok(Some(self.input[self.pos - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.input.len() {
                Ok(Some(self.input[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn position(&self) -> Position {
            Position::new(self.pos as u64, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.pos as u64, 0)
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
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mock_reader = MockRead {
        pos: 0,
        input: vec![], // simulating EOF
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: Vec::new(),
        remaining_depth: 1,
    };

    // This should return Err with ErrorCode::EofWhileParsingList since there are no characters to read
    let result = deserializer.end_seq();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().err, ErrorCode::EofWhileParsingList);
}

