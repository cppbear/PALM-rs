// Answer 0

fn test_parse_object_colon_success() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let b = self.input[self.index];
                self.index += 1;
                Ok(Some(b))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            // Simulate eating a character (no-op for test)
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            // Simulate returning a parsing error
            Error::new(code)
        }
    }

    // Expecting success, with ':' as the valid character after whitespace
    let mut parser = TestParser { input: b" :".to_vec(), index: 0 };
    let result = parser.parse_object_colon();
    assert!(result.is_ok());
}

fn test_parse_object_colon_expected_colon_error() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulate valid whitespace followed by a non-colon character
            if self.index < self.input.len() {
                let b = self.input[self.index];
                self.index += 1;
                Ok(Some(b))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {}
        
        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }
    }

    // Using a non-colon character to trigger the expected error
    let mut parser = TestParser { input: b" a".to_vec(), index: 0 };
    let result = parser.parse_object_colon();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::ExpectedColon);
}

fn test_parse_object_colon_eof_error() {
    struct TestParser {
        input: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulate end of input (like EOF)
            if self.index < self.input.len() {
                let b = self.input[self.index];
                self.index += 1;
                Ok(Some(b))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }
    }

    // Only whitespace followed by EOF
    let mut parser = TestParser { input: b" ".to_vec(), index: 0 };
    let result = parser.parse_object_colon();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::EofWhileParsingObject);
}

