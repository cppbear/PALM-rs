// Answer 0

#[test]
fn test_parse_str_bytes_valid() {
    struct DummyParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl DummyParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.pos < self.data.len() {
                let ch = self.data[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Err(ErrorCode::EndOfInput) // Simulate EOF
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Ok(()) // Simulate successful escape parsing
        }
    }

    let mut parser = DummyParser { data: vec![b'a', b'b', b'c', b'"'], pos: 0 };
    let mut scratch = Vec::new();
    
    let result: Result<Vec<u8>> = parser.parse_str_bytes(&mut scratch, false, |_, bytes| Ok(bytes.to_vec()));

    assert_eq!(result, Ok(vec![b'a', b'b', b'c']));
}

#[test]
fn test_parse_str_bytes_control_character_error() {
    struct DummyParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl DummyParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.pos < self.data.len() {
                let ch = self.data[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Err(ErrorCode::EndOfInput) // Simulate EOF
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Ok(()) // Simulate successful escape parsing
        }
    }

    let mut parser = DummyParser { data: vec![b'a', b'\x01', b'b', b'"'], pos: 0 }; // Control character
    let mut scratch = Vec::new();

    let result: Result<Vec<u8>> = parser.parse_str_bytes(&mut scratch, true, |_, bytes| Ok(bytes.to_vec()));

    assert_eq!(result, Err(ErrorCode::ControlCharacterWhileParsingString));
}

#[test]
fn test_parse_str_bytes_eof_error() {
    struct DummyParser {
        data: Vec<u8>,
        pos: usize,
    }

    impl DummyParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.pos < self.data.len() {
                let ch = self.data[self.pos];
                self.pos += 1;
                Ok(ch)
            } else {
                Err(ErrorCode::EndOfInput) // Simulate EOF
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Ok(()) // Simulate successful escape parsing
        }
    }

    let mut parser = DummyParser { data: vec![], pos: 0 };
    let mut scratch = Vec::new();

    let result: Result<Vec<u8>> = parser.parse_str_bytes(&mut scratch, false, |_, bytes| Ok(bytes.to_vec()));

    assert_eq!(result, Err(ErrorCode::EndOfInput));
}

