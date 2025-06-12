// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    struct TestParser {
        pos: Position,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let parser = TestParser::new("<validName>");
    let result = parser.parse_capture_name(0);
    assert!(result.is_ok());
    let capture_name = result.unwrap();
    assert_eq!(capture_name.name, "validName");
}

#[test]
#[should_panic(expected = "GroupNameUnexpectedEof")]
fn test_parse_capture_name_eof_before_gt() {
    struct TestParser {
        pos: Position,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<()> {
            Err(ast::Error { kind: ast::ErrorKind::GroupNameUnexpectedEof, pattern: self.pattern.clone(), span: self.span() })
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let parser = TestParser::new("<invalidName");
    parser.parse_capture_name(0).unwrap();
}

#[test]
fn test_parse_capture_name_invalid_character() {
    struct TestParser {
        pos: Position,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let parser = TestParser::new("<invalid-name>");
    let result = parser.parse_capture_name(0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ast::ErrorKind::GroupNameInvalid);
}

#[test]
fn test_parse_capture_name_empty_name() {
    struct TestParser {
        pos: Position,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: pattern.to_string(),
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span }
        }

        fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<()> {
            Ok(())
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let parser = TestParser::new("<>");
    let result = parser.parse_capture_name(0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ast::ErrorKind::GroupNameEmpty);
}

