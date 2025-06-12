// Answer 0

#[test]
fn test_end_seq_with_closing_bracket() {
    struct TestParser {
        state: u8,
    }

    impl TestParser {
        fn new() -> Self {
            Self { state: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.state == 0 {
                self.state += 1;
                Ok(Some(b']'))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            // Simulating character eating
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error
        }
    }

    let mut parser = TestParser::new();
    let result = parser.end_seq();
    assert!(result.is_ok());
}

#[test]
fn test_end_seq_with_trailing_comma() {
    struct TestParser {
        state: u8,
    }

    impl TestParser {
        fn new() -> Self {
            Self { state: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.state == 0 {
                self.state += 1;
                Ok(Some(b','))
            } else {
                Ok(Some(b']'))
            }
        }

        fn eat_char(&mut self) {
            // Simulating character eating
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            assert_eq!(code, ErrorCode::TrailingComma);
            Error
        }
    }

    let mut parser = TestParser::new();
    let result = parser.end_seq();
    assert!(result.is_err());
}

#[test]
fn test_end_seq_with_trailing_characters() {
    struct TestParser {
        state: u8,
    }

    impl TestParser {
        fn new() -> Self {
            Self { state: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.state == 0 {
                self.state += 1;
                Ok(Some(b'x')) // Invalid character instead of ']' or ','
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            // Simulating character eating
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error
        }
    }

    let mut parser = TestParser::new();
    let result = parser.end_seq();
    assert!(result.is_err());
}

#[test]
fn test_end_seq_with_eof() {
    struct TestParser {
        state: u8,
    }

    impl TestParser {
        fn new() -> Self {
            Self { state: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(None) // Simulating EOF
        }

        fn eat_char(&mut self) {
            // Simulating character eating
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error
        }
    }

    let mut parser = TestParser::new();
    let result = parser.end_seq();
    assert!(result.is_err());
}

