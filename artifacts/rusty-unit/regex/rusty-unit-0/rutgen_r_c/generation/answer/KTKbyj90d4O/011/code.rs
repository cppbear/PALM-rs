// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    struct MockParser {
        pattern: String,
        pos: Position,
        eof: bool,
        char_index: usize,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 1, line: 1, column: 2 },
                eof: false,
                char_index: 1,
            }
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            if self.char_index < self.pattern.len() {
                self.pattern.chars().nth(self.char_index).unwrap()
            } else {
                '\0'
            }
        }

        fn bump(&mut self) -> bool {
            if self.char_index < self.pattern.len() {
                self.char_index += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let parser = MockParser::new("<valid_name>");
    let result = parser.parse_capture_name(0);
    assert!(result.is_ok());
    let cap_name = result.unwrap();
    assert_eq!(cap_name.name, "valid_name");
}

#[test]
#[should_panic]
fn test_parse_capture_name_eof() {
    struct MockParser {
        pattern: String,
        eof: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                eof: true,
            }
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        // Other methods omitted for brevity...
        
        fn char(&self) -> char {
            'a' // This method is not called as is_eof() returns true.
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let parser = MockParser::new("<name>");
    let _ = parser.parse_capture_name(0);
}

#[test]
#[should_panic]
fn test_parse_capture_name_invalid_name() {
    struct MockParser {
        pattern: String,
        pos: Position,
        char_index: usize,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 1, line: 1, column: 2 },
                char_index: 1,
            }
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn char(&self) -> char {
            if self.char_index < self.pattern.len() {
                self.pattern.chars().nth(self.char_index).unwrap()
            } else {
                '>'
            }
        }

        fn bump(&mut self) -> bool {
            self.char_index += 1;
            true
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }

        fn add_capture_name(&self, _: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let parser = MockParser::new("<1invalid>");
    let _ = parser.parse_capture_name(0);
}

