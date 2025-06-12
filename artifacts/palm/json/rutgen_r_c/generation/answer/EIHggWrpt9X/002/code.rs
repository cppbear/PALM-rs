// Answer 0

#[test]
fn test_end_with_trailing_characters() {
    struct DummyRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for DummyRead {
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

        fn discard(&mut self) {
            self.position = self.data.len();
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 + 1 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 + 1 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
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

    let data = b"some data with trailing characters";
    let reader = DummyRead { data: data.to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 128 };

    let result = deserializer.end();
    assert!(result.is_err());
    
    if let Err(error) = result {
        // Expect the error to be the trailing characters error.
        assert_eq!(error.peek_error(ErrorCode::TrailingCharacters).err, ErrorCode::TrailingCharacters);
    }
}

#[test]
fn test_end_with_only_whitespace() {
    struct DummyRead {
        data: Vec<u8>,
        position: usize,
    }
    
    impl<'de> Read<'de> for DummyRead {
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

        fn discard(&mut self) {
            self.position = self.data.len();
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 + 1 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 + 1 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }
        
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

    let data = b"   "; // Only whitespace
    let reader = DummyRead { data: data.to_vec(), position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 128 };

    let result = deserializer.end();
    assert!(result.is_ok());
}

