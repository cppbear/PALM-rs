// Answer 0

fn test_end_map_success() {
    struct TestParser {
        data: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.index < self.data.len() {
                // Simulating whitespace parsing returning a closing brace
                if self.data[self.index] == b'}' {
                    self.index += 1;
                    Ok(Some(b'}'))
                } else {
                    Err(ErrorCode::UnexpectedCharacter)
                }
            } else {
                Err(ErrorCode::EofWhileParsingObject)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_error(&self, err: ErrorCode) -> Error {
            Error::new(err)
        }
    }

    let mut parser = TestParser { data: vec![b'}'], index: 0 };
    let result = parser.end_map();
    assert!(result.is_ok());
}

fn test_end_map_trailing_comma() {
    struct TestParser {
        data: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.index < self.data.len() {
                // Simulating whitespace parsing returning a trailing comma
                if self.data[self.index] == b',' {
                    self.index += 1;
                    Ok(Some(b','))
                } else {
                    Err(ErrorCode::UnexpectedCharacter)
                }
            } else {
                Err(ErrorCode::EofWhileParsingObject)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_error(&self, err: ErrorCode) -> Error {
            Error::new(err)
        }
    }

    let mut parser = TestParser { data: vec![b','], index: 0 };
    let result = parser.end_map();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::TrailingComma);
}

fn test_end_map_trailing_characters() {
    struct TestParser {
        data: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.index < self.data.len() {
                // Simulating whitespace parsing returning a trailing character
                if self.data[self.index] != b'}' {
                    self.index += 1;
                    Ok(Some(b'a')) // A character that is not valid
                } else {
                    Err(ErrorCode::UnexpectedCharacter)
                }
            } else {
                Err(ErrorCode::EofWhileParsingObject)
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_error(&self, err: ErrorCode) -> Error {
            Error::new(err)
        }
    }

    let mut parser = TestParser { data: vec![b'a'], index: 0 };
    let result = parser.end_map();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::TrailingCharacters);
}

fn test_end_map_eof() {
    struct TestParser {
        data: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            if self.index < self.data.len() {
                Ok(None) // Simulating EOF
            } else {
                Err(ErrorCode::EofWhileParsingObject)
            }
        }

        fn eat_char(&mut self) {
            // In this case, we won't be eating any characters since we are simulating EOF
        }

        fn peek_error(&self, err: ErrorCode) -> Error {
            Error::new(err)
        }
    }

    let mut parser = TestParser { data: vec![], index: 0 };
    let result = parser.end_map();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::EofWhileParsingObject);
}

