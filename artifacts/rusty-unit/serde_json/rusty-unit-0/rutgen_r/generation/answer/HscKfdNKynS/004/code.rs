// Answer 0

#[test]
fn test_parse_str_bytes_success() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(ErrorCode::EOF)
            }
        }

        fn parse_escape(&mut self, _validate: bool, scratch: &mut Vec<u8>) -> Result<()> {
            scratch.push(b'x'); // Simulate escape handling by pushing 'x'
            Ok(())
        }
    }

    let mut parser = TestParser { input: b"hello \\\"world\"".to_vec(), index: 0 };
    let mut scratch = Vec::new();

    let result = parse_str_bytes(&mut parser, &mut scratch, true, |_, _| {
        Ok("success")
    });

    assert_eq!(result, Ok("success"));
}

#[test]
#[should_panic]
fn test_parse_str_bytes_parse_escape_err() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(ErrorCode::EOF)
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Err(ErrorCode::InvalidEscape) // Simulate error in parsing escape
        }
    }

    let mut parser = TestParser { input: b"hello \\".to_vec(), index: 0 };
    let mut scratch = Vec::new();

    let result = parse_str_bytes(&mut parser, &mut scratch, true, |_, _| {
        Ok("success")
    });

    // This should panic because of the escape parsing error
    assert_eq!(result, Err(ErrorCode::InvalidEscape));
}

#[test]
fn test_parse_str_bytes_control_character() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Ok(0) // Return EOF as 0
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    let mut parser = TestParser { input: b"hello \x00".to_vec(), index: 0 };
    let mut scratch = Vec::new();

    let result = parse_str_bytes(&mut parser, &mut scratch, true, |_, _| {
        Ok("success")
    });

    assert_eq!(result, Err(ErrorCode::ControlCharacterWhileParsingString));
}

