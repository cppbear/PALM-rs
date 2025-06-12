// Answer 0

#[test]
fn test_ignore_value_with_valid_literals() {
    struct DummyRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for DummyRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let result = Some(self.data[self.pos]);
                self.pos += 1;
                Ok(result)
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

        fn discard(&mut self) {
            self.pos += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let mut read = DummyRead { 
        data: b"null,true,false,-123,0,\"string_value\",[]".to_vec(), 
        pos: 0
    };

    let mut deserializer = Deserializer {
        read,
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer.ignore_value();
    
    assert!(result.is_ok());
}

#[test]
fn test_ignore_value_with_eof_error() {
    struct DummyReadEOF;
    
    impl<'de> Read<'de> for DummyReadEOF {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None) // Just return None to simulate EOF
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None) // Just return None to simulate EOF
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }
    
    let mut read_eof = DummyReadEOF;

    let mut deserializer_eof = Deserializer {
        read: read_eof,
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer_eof.ignore_value();
    
    assert!(matches!(result, Err(ErrorCode::EofWhileParsingValue)));
}

#[test]
fn test_ignore_value_with_invalid_character() {
    struct DummyReadInvalid {
        data: Vec<u8>,
        pos: usize,
    }

    impl<'de> Read<'de> for DummyReadInvalid {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let result = Some(self.data[self.pos]);
                self.pos += 1;
                Ok(result)
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

        fn position(&self) -> Position {
            Position { line: 0, column: self.pos as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.pos as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }
    }

    let mut read_invalid = DummyReadInvalid { 
        data: b"invalid_char".to_vec(), 
        pos: 0 
    };

    let mut deserializer_invalid = Deserializer {
        read: read_invalid,
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer_invalid.ignore_value();
    
    assert!(matches!(result, Err(ErrorCode::ExpectedSomeValue)));
}

