// Answer 0

#[test]
fn test_end_seq_eof_while_parsing_list() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl Read<'_> for MockRead {
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
        
        fn peek(&mut self) -> Result<Option<u8>> { self.next() }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: self.position } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }
    
    let mut deserializer = Deserializer {
        read: MockRead { data: vec![], position: 0 },
        scratch: vec![],
        remaining_depth: 0,
    };
    
    let result = deserializer.end_seq();
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.peek_error(ErrorCode::EofWhileParsingList).err, error.err);
    }
}

#[test]
fn test_end_seq_trailing_comma() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl Read<'_> for MockRead {
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

        fn peek(&mut self) -> Result<Option<u8>> { self.next() }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: self.position } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: vec![b','], position: 0 },
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.end_seq();
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.peek_error(ErrorCode::TrailingComma).err, error.err);
    }
}

#[test]
fn test_end_seq_trailing_characters() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl Read<'_> for MockRead {
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

        fn peek(&mut self) -> Result<Option<u8>> { self.next() }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: self.position } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: vec![b'x'], position: 0 },
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.end_seq();
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.peek_error(ErrorCode::TrailingCharacters).err, error.err);
    }
}

#[test]
fn test_end_seq_closing_bracket() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl Read<'_> for MockRead {
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

        fn peek(&mut self) -> Result<Option<u8>> { self.next() }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position { line: 0, column: 0 } }
        fn peek_position(&self) -> Position { Position { line: 0, column: self.position } }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: MockRead { data: vec![b']'], position: 0 },
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.end_seq();
    assert!(result.is_ok());
}

