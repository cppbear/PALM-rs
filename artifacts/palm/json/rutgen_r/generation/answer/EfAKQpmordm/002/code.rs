// Answer 0

#[test]
fn test_ignore_str_returns_ok_on_double_quote() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                // Simulate EOF
                Err(ErrorCode::EOF)
            }
        }

        fn is_escape(&self, _: bool) -> bool {
            true
        }

        fn ignore_escape(&mut self) -> Result<()> {
            Ok(())
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::ControlCharacterWhileParsingString)
        }
    }

    impl TestReader {
        fn ignore_str(&mut self) -> Result<()> {
            loop {
                let ch = self.next_or_eof()?;
                if !self.is_escape(ch) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        self.ignore_escape()?;
                    }
                    _ => {
                        return self.error(ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut reader = TestReader {
        input: vec![b'\\', b'"'], // Valid escape and then double quote
        position: 0,
    };

    let result = reader.ignore_str();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_str_returns_error_on_control_character() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(ErrorCode::EOF)
            }
        }

        fn is_escape(&self, _: bool) -> bool {
            true
        }

        fn ignore_escape(&mut self) -> Result<()> {
            Ok(())
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::ControlCharacterWhileParsingString)
        }
    }

    impl TestReader {
        fn ignore_str(&mut self) -> Result<()> {
            loop {
                let ch = self.next_or_eof()?;
                if !self.is_escape(ch) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        self.ignore_escape()?;
                    }
                    _ => {
                        return self.error(ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut reader = TestReader {
        input: vec![b'\\', b'x'], // Valid escape but an invalid character after
        position: 0,
    };

    let result = reader.ignore_str();
    assert!(result.is_err());
}

