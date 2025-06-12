// Answer 0

fn test_parse_str_bytes_escape() {
    struct MockParser {
        input: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(val)
            } else {
                Err(ErrorCode::Eof)
            }
        }
        
        fn parse_escape(&mut self, _validate: bool, scratch: &mut Vec<u8>) -> Result<()> {
            scratch.push(b'\\');
            Ok(())
        }
    }

    let input = vec![b'H', b'e', b'l', b'l', b'o', b'\\', b'"', b' ', b'W', b'o', b'r', b'l', b'd', b'"'];
    let mut parser = MockParser { input, position: 0 };
    let mut scratch = Vec::new();
    
    let result = parser.parse_str_bytes(&mut scratch, true, |_, bytes| {
        Ok(String::from_utf8(bytes.to_vec()).map_err(|_| ErrorCode::InvalidUtf8)?)
    });

    assert_eq!(result.unwrap(), "Hello\\\" World");
}

fn test_parse_str_bytes_no_escape() {
    struct MockParser {
        input: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(val)
            } else {
                Err(ErrorCode::Eof)
            }
        }

        fn parse_escape(&mut self, _validate: bool, scratch: &mut Vec<u8>) -> Result<()> {
            scratch.push(b'\\');
            Ok(())
        }
    }

    let input = vec![b'S', b't', b'r', b'i', b'n', b'g', b' ', b'1', b'2', b'3', b'\\', b'"', b' ', b'4', b'5', b'6', b'"'];
    let mut parser = MockParser { input, position: 0 };
    let mut scratch = Vec::new();

    let result = parser.parse_str_bytes(&mut scratch, true, |_, bytes| {
        Ok(String::from_utf8(bytes.to_vec()).map_err(|_| ErrorCode::InvalidUtf8)?)
    });

    assert_eq!(result.unwrap(), "String 123\\\" 456");
}

fn test_parse_str_bytes_invalid_character() {
    struct MockParser {
        input: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let val = self.input[self.position];
                self.position += 1;
                Ok(val)
            } else {
                Err(ErrorCode::Eof)
            }
        }

        fn parse_escape(&mut self, validate: bool, scratch: &mut Vec<u8>) -> Result<()> {
            if validate {
                return Err(ErrorCode::ControlCharacterWhileParsingString);
            }
            scratch.push(b'\\');
            Ok(())
        }
    }

    let input = vec![b'B', b'a', b'd', b'\\', b'c', b'h', b'a', b'r'];
    let mut parser = MockParser { input, position: 0 };
    let mut scratch = Vec::new();
    
    let result = parser.parse_str_bytes(&mut scratch, true, |_, bytes| {
        Ok(String::from_utf8(bytes.to_vec()).map_err(|_| ErrorCode::InvalidUtf8)?)
    });

    match result {
        Err(ErrorCode::ControlCharacterWhileParsingString) => assert!(true),
        _ => assert!(false, "Expected error not returned"),
    }
}

fn test_parse_str_bytes_empty_input() {
    struct MockParser {
        input: Vec<u8>,
        position: usize,
    }

    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            Err(ErrorCode::Eof)
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    let input = vec![];
    let mut parser = MockParser { input, position: 0 };
    let mut scratch = Vec::new();

    let result = parser.parse_str_bytes(&mut scratch, true, |_, _| {
        Ok("".to_string())
    });

    assert!(result.is_err());
}

