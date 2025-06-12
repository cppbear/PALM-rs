// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    struct MockParser {
        pattern: String,
        position: Position,
        eof: bool,
    }
    
    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                position: Position { offset: 0, line: 1, column: 1 },
                eof: false,
            }
        }
        
        fn is_eof(&self) -> bool {
            self.eof
        }
        
        fn char(&self) -> char {
            if self.position.offset < self.pattern.len() {
                self.pattern.chars().nth(self.position.offset).unwrap()
            } else {
                '\0'
            }
        }

        fn bump(&mut self) -> bool {
            if self.position.offset < self.pattern.len() {
                self.position.offset += 1;
                true
            } else {
                false
            }
        }
        
        fn pos(&self) -> Position {
            self.position
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

    let mut parser = MockParser::new("<validName>");
    let result = parser.parse_capture_name(0);
    assert!(result.is_ok());

    if let Ok(capture_name) = result {
        assert_eq!(capture_name.name, "validName");
    }
}

#[test]
#[should_panic]
fn test_parse_capture_name_eof_before_name() {
    struct MockParser {
        pattern: String,
        position: Position,
        eof: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                position: Position { offset: 0, line: 1, column: 1 },
                eof: true,
            }
        }
        
        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            if self.position.offset < self.pattern.len() {
                self.pattern.chars().nth(self.position.offset).unwrap()
            } else {
                '\0'
            }
        }

        fn bump(&mut self) -> bool {
            if self.position.offset < self.pattern.len() {
                self.position.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.position
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

    let mut parser = MockParser::new("<");
    parser.parse_capture_name(0).unwrap();
}

#[test]
#[should_panic]
fn test_parse_capture_name_invalid_char() {
    struct MockParser {
        pattern: String,
        position: Position,
        eof: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                position: Position { offset: 0, line: 1, column: 1 },
                eof: false,
            }
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            if self.position.offset < self.pattern.len() {
                self.pattern.chars().nth(self.position.offset).unwrap()
            } else {
                '\0'
            }
        }

        fn bump(&mut self) -> bool {
            if self.position.offset < self.pattern.len() {
                self.position.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.position
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

    let mut parser = MockParser::new("<invalid@>");
    parser.parse_capture_name(0).unwrap();
}

#[test]
#[should_panic]
fn test_parse_capture_name_empty_name() {
    struct MockParser {
        pattern: String,
        position: Position,
        eof: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                position: Position { offset: 0, line: 1, column: 1 },
                eof: false,
            }
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn char(&self) -> char {
            if self.position.offset < self.pattern.len() {
                self.pattern.chars().nth(self.position.offset).unwrap()
            } else {
                '\0'
            }
        }

        fn bump(&mut self) -> bool {
            if self.position.offset < self.pattern.len() {
                self.position.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.position
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

    let mut parser = MockParser::new("<>");
    parser.parse_capture_name(0).unwrap();
}

