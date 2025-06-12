// Answer 0

fn end_map_test() -> Result<()> {
    struct TestParser {
        whitespace: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn new(whitespace: Vec<u8>) -> Self {
            TestParser { whitespace, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index >= self.whitespace.len() {
                return Ok(None);
            }
            let byte = self.whitespace[self.index];
            self.index += 1;
            Ok(Some(byte))
        }

        fn eat_char(&mut self) {
            // Dummy implementation; nothing to do here for this test.
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Test error")
        }
    }

    let mut parser = TestParser::new(vec![b',']);
    let result = parser.end_map();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Test error");

    Ok(())
}

#[test]
fn test_end_map_trailing_comma() {
    let result = end_map_test();
    assert!(result.is_ok());
}

#[test]
fn test_end_map_eof_while_parsing_object() {
    struct TestParserEof {
        whitespace: Vec<u8>,
        index: usize,
    }

    impl TestParserEof {
        fn new(whitespace: Vec<u8>) -> Self {
            TestParserEof { whitespace, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index >= self.whitespace.len() {
                return Ok(None);
            }
            let byte = self.whitespace[self.index];
            self.index += 1;
            Ok(Some(byte))
        }

        fn eat_char(&mut self) {
            // Dummy implementation; nothing to do here for this test.
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Test error")
        }
    }

    let mut parser = TestParserEof::new(vec![]);
    let result = parser.end_map();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Test error");
}

#[test]
fn test_end_map_trailing_characters() {
    struct TestParserTrailing {
        whitespace: Vec<u8>,
        index: usize,
    }

    impl TestParserTrailing {
        fn new(whitespace: Vec<u8>) -> Self {
            TestParserTrailing { whitespace, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.index >= self.whitespace.len() {
                return Ok(None);
            }
            let byte = self.whitespace[self.index];
            self.index += 1;
            Ok(Some(byte))
        }

        fn eat_char(&mut self) {
            // Dummy implementation; nothing to do here for this test.
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("Test error")
        }
    }

    let mut parser = TestParserTrailing::new(vec![b'x']);
    let result = parser.end_map();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Test error");
}

