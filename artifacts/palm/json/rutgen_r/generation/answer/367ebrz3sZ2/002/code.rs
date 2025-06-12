// Answer 0

fn test_end_map_success() -> Result<()> {
    struct TestParser {
        whitespace_called: bool,
        char: Option<u8>,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            self.whitespace_called = true;
            Ok(self.char)
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new()
        }
    }

    let mut parser = TestParser {
        whitespace_called: false,
        char: Some(b'}'),
    };
    
    let result = parser.end_map();
    assert!(result.is_ok());
    assert!(parser.whitespace_called);
}

fn test_end_map_trailing_comma() -> Result<()> {
    struct TestParser {
        whitespace_called: bool,
        char: Option<u8>,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            self.whitespace_called = true;
            Ok(Some(b','))
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new()
        }
    }

    let mut parser = TestParser {
        whitespace_called: false,
        char: Some(b','),
    };
    
    let result = parser.end_map();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), parser.peek_error(ErrorCode::TrailingComma));
    assert!(parser.whitespace_called);
}

fn test_end_map_trailing_characters() -> Result<()> {
    struct TestParser {
        whitespace_called: bool,
        char: Option<u8>,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            self.whitespace_called = true;
            Ok(Some(b'x'))
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new()
        }
    }

    let mut parser = TestParser {
        whitespace_called: false,
        char: Some(b'x'),
    };
    
    let result = parser.end_map();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), parser.peek_error(ErrorCode::TrailingCharacters));
    assert!(parser.whitespace_called);
}

fn test_end_map_eof() -> Result<()> {
    struct TestParser {
        whitespace_called: bool,
        char: Option<u8>,
    }

    impl TestParser {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            self.whitespace_called = true;
            Ok(None)
        }

        fn eat_char(&mut self) {}

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new()
        }
    }

    let mut parser = TestParser {
        whitespace_called: false,
        char: None,
    };
    
    let result = parser.end_map();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), parser.peek_error(ErrorCode::EofWhileParsingObject));
    assert!(parser.whitespace_called);
}

