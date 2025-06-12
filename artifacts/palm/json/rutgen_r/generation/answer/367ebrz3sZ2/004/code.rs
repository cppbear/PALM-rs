// Answer 0

fn end_map_test() {
    struct TestParser {
        current_char: Option<u8>,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            // Simulate parsing whitespace sending back a closing brace
            Ok(self.current_char)
        }

        fn eat_char(&mut self) {
            self.current_char = None; // Simulate consuming the character
        }

        fn peek_error(&self, _error: ErrorCode) -> ErrorCode {
            ErrorCode::EofWhileParsingObject // Simulate returning an error code
        }
    }

    let mut parser = TestParser { current_char: Some(b'}') };

    // Test for the successful case
    let result = parser.end_map();
    assert_eq!(result.is_ok(), true);
}

fn end_map_trailing_comma_test() {
    struct TestParser {
        current_char: Option<u8>,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            // Simulate parsing whitespace sending back a comma
            Ok(self.current_char)
        }

        fn eat_char(&mut self) {
            self.current_char = None; // Simulate consuming the character
        }

        fn peek_error(&self, _error: ErrorCode) -> ErrorCode {
            ErrorCode::TrailingComma // Simulate returning an error code
        }
    }

    let mut parser = TestParser { current_char: Some(b',') };

    // Test for trailing comma case
    let result = parser.end_map();
    assert_eq!(result.is_err(), true);
}

fn end_map_trailing_characters_test() {
    struct TestParser {
        current_char: Option<u8>,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            // Simulate parsing whitespace sending back an invalid character
            Ok(self.current_char)
        }

        fn eat_char(&mut self) {
            self.current_char = None; // Simulate consuming the character
        }

        fn peek_error(&self, _error: ErrorCode) -> ErrorCode {
            ErrorCode::TrailingCharacters // Simulate returning an error code
        }
    }

    let mut parser = TestParser { current_char: Some(b'x') };

    // Test for trailing characters case
    let result = parser.end_map();
    assert_eq!(result.is_err(), true);
}

fn end_map_eof_test() {
    struct TestParser {
        current_char: Option<u8>,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            // Simulate parsing whitespace sending back None
            Ok(self.current_char)
        }

        fn eat_char(&mut self) {
            // No characters to eat in this case
        }

        fn peek_error(&self, _error: ErrorCode) -> ErrorCode {
            ErrorCode::EofWhileParsingObject // Simulate returning an error code
        }
    }

    let mut parser = TestParser { current_char: None };

    // Test for EOF while parsing object
    let result = parser.end_map();
    assert_eq!(result.is_err(), true);
}

