// Answer 0

fn parse_object_colon(&mut self) -> Result<()> {
    match tri!(self.parse_whitespace()) {
        Some(b':') => {
            self.eat_char();
            Ok(())
        }
        Some(_) => Err(self.peek_error(ErrorCode::ExpectedColon)),
        None => Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
    }
}

#[test]
fn test_parse_object_colon_colon_found() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                if byte == b' ' || byte == b'\n' || byte == b'\t' {
                    self.index += 1;
                    Ok(Some(byte))
                } else {
                    Ok(Some(byte))
                }
            } else {
                Ok(None)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Error")
        }
    }

    let mut parser = TestParser { 
        input: vec![b' ', b':'], 
        index: 0 
    };
    let result = parser.parse_object_colon();
    assert!(result.is_ok());
}

#[test]
fn test_parse_object_colon_colon_not_found() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                if byte == b' ' || byte == b'\n' || byte == b'\t' {
                    self.index += 1;
                    Ok(Some(byte))
                } else {
                    Ok(Some(byte))
                }
            } else {
                Ok(None)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Error")
        }
    }

    let mut parser = TestParser { 
        input: vec![b' ', b'a'], 
        index: 0 
    };
    let result = parser.parse_object_colon();
    assert!(result.is_err());
}

#[test]
fn test_parse_object_colon_end_of_input() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let byte = self.input[self.index];
                if byte == b' ' || byte == b'\n' || byte == b'\t' {
                    self.index += 1;
                    Ok(Some(byte))
                } else {
                    Ok(Some(byte))
                }
            } else {
                Ok(None)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Error")
        }
    }

    let mut parser = TestParser { 
        input: vec![], 
        index: 0 
    };
    let result = parser.parse_object_colon();
    assert!(result.is_err());
}

