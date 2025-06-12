// Answer 0

#[test]
fn test_parse_escape_octals_disabled() {
    struct MockParser {
        octal: bool,
        pattern: String,
        pos: Position,
        current: char,
    }

    impl MockParser {
        fn new(octal: bool, pattern: &str) -> Self {
            Self {
                octal,
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                current: '\\',
            }
        }

        fn char(&self) -> char {
            self.current
        }

        fn bump(&mut self) -> bool {
            self.current = 'a'; // Next character after escape sequence
            true
        }

        fn error(&self, span: Span, error_kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: error_kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos) // Just an example implementation
        }
        
        fn parser(&self) -> &Self {
            self
        }
        
        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn parse_unicode_class(&self) -> Result<ast::ClassUnicode> {
            Err(ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            })
        }
    }

    let parser = MockParser::new(false, "\\p{");
    let result = parser.parse_escape();
    
    assert!(result.is_err());
}

#[test]
fn test_parse_escape_invalid_unicode_class() {
    struct MockParser {
        octal: bool,
        pattern: String,
        pos: Position,
        current: char,
    }

    impl MockParser {
        fn new(octal: bool, pattern: &str) -> Self {
            Self {
                octal,
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                current: '\\',
            }
        }

        fn char(&self) -> char {
            self.current
        }

        fn bump(&mut self) -> bool {
            self.current = 'P'; // Next valid character after escape sequence
            true
        }

        fn error(&self, span: Span, error_kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: error_kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos) // Just an example implementation
        }
        
        fn parser(&self) -> &Self {
            self
        }
        
        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn parse_unicode_class(&self) -> Result<ast::ClassUnicode> {
            Err(ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos, self.pos),
            })
        }
    }

    let parser = MockParser::new(true, "\\P{");
    let result = parser.parse_escape();
    
    assert!(result.is_err());
}

