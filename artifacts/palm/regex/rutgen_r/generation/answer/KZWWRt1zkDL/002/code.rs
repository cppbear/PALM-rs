// Answer 0

#[test]
fn test_parse_escape_octal_unsupported_backreference() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl TestParser {
        fn new(input: &str, octal: bool) -> Self {
            Self { input: input.chars().collect(), pos: 0, octal }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<Primitive> {
            Err(kind) // Simplified error handling for this test
        }
    }

    let mut parser = TestParser::new("\\7", false);
    assert_eq!(parser.char(), '\\');
    let start = parser.pos();
    
    // Now we bump the parser to move past the escape character
    assert!(parser.bump());
    
    // Next character should be '7' (octal)
    assert_eq!(parser.char(), '7');
    
    // Trying to parse escape with octal flag set to false should yield an error
    let result = parser.parse_escape();
    assert!(result.is_err());

    // Validate the error is of the expected type
    if let Err(kind) = result {
        assert_eq!(kind, ast::ErrorKind::UnsupportedBackreference);
    }
}

#[test]
fn test_parse_escape_unrecognized_escape() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self { input: input.chars().collect(), pos: 0 }
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<Primitive> {
            Err(kind) // Simplified error handling for this test
        }
    }

    let mut parser = TestParser::new("\\x"); // Unrecognized escape
    assert_eq!(parser.char(), '\\');
    let start = parser.pos();
    
    // Move past the escape character
    assert!(parser.bump());
    
    // Next character should be 'x', which is followed by no valid hex digit
    assert_eq!(parser.char(), 'x');
    
    // Now we try to parse the escape sequence
    let result = parser.parse_escape();
    assert!(result.is_err());

    // Validate the error is of the expected type
    if let Err(kind) = result {
        assert_eq!(kind, ast::ErrorKind::EscapeUnrecognized);
    }
}

