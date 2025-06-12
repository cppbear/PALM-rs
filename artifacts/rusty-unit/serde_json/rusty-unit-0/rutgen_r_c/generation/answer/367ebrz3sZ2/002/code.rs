// Answer 0

fn test_end_map_valid() {
    struct TestRead {
        call_count: usize,
    }
    
    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.call_count < 1 {
                self.call_count += 1;
                Ok(Some(b'}'))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            self.next()
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }
        
        fn peek_position(&self) -> Position { Position::default() }
        
        fn byte_offset(&self) -> usize { 0 }
        
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }
        
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: TestRead { call_count: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    
    assert!(deserializer.end_map().is_ok());
}

fn test_end_map_trailing_comma() {
    struct TestRead {
        call_count: usize,
    }
    
    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b','))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b','))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: TestRead { call_count: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.end_map();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().err, ErrorCode::TrailingComma);
}

fn test_end_map_trailing_characters() {
    struct TestRead {
        call_count: usize,
    }
    
    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: TestRead { call_count: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.end_map();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().err, ErrorCode::TrailingCharacters);
}

fn test_end_map_eof() {
    struct TestRead {
        call_count: usize,
    }
    
    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { Position::default() }

        fn peek_position(&self) -> Position { Position::default() }

        fn byte_offset(&self) -> usize { 0 }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { Ok(()) }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let mut deserializer = Deserializer {
        read: TestRead { call_count: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.end_map();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().err, ErrorCode::EofWhileParsingObject);
}

