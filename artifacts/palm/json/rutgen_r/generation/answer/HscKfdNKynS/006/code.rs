// Answer 0

#[test]
fn test_parse_str_bytes_valid_string() {
    struct MockParser {
        pos: usize,
        data: Vec<u8>,
    }

    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Ok(val)
            } else {
                Err(ErrorCode::EOF)
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Ok(()) // Simulate successful escape parsing
        }
    }

    let mut parser = MockParser {
        pos: 0,
        data: vec![b'"', b'H', b'e', b'l', b'l', b'o', b'"'],
    };
    
    let mut scratch = Vec::new();
    let result = parser.parse_str_bytes(&mut scratch, true, |_, bytes| {
        Ok(String::from_utf8_lossy(bytes).into_owned())
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello");
}

#[test]
fn test_parse_str_bytes_escape_character() {
    struct MockParser {
        pos: usize,
        data: Vec<u8>,
    }

    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Ok(val)
            } else {
                Err(ErrorCode::EOF)
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Ok(()) // Simulate successful escape parsing
        }
    }

    let mut parser = MockParser {
        pos: 0,
        data: vec![b'"', b'H', b'e', b'l', b'l', b'o', b'\\', b'"', b'"', b'"', b'"'],
    };

    let mut scratch = Vec::new();
    let result = parser.parse_str_bytes(&mut scratch, true, |_, bytes| {
        Ok(String::from_utf8_lossy(bytes).into_owned())
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello\"\"\"\""); 
}

#[test]
#[should_panic(expected = "ControlCharacterWhileParsingString")]
fn test_parse_str_bytes_invalid_character() {
    struct MockParser {
        pos: usize,
        data: Vec<u8>,
    }

    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Ok(val)
            } else {
                Err(ErrorCode::EOF)
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Ok(()) // Simulate successful escape parsing
        }
    }

    let mut parser = MockParser {
        pos: 0,
        data: vec![b'"', b'H', b'e', b'l', b'l', b'o', b'\n', b'"'],
    };

    let mut scratch = Vec::new();
    let result = parser.parse_str_bytes(&mut scratch, true, |_, _| {
        Err(ErrorCode::ControlCharacterWhileParsingString) // Simulate error
    });

    result.unwrap(); // This will panic
}

