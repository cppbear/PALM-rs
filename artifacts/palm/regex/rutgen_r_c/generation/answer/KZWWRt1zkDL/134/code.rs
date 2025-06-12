// Answer 0

#[test]
fn test_parse_escape_with_valid_unicode() {
    struct MockParser {
        pattern: String,
        pos: Position,
        octal: bool,
        ignore_whitespace: bool,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                octal: true,
                ignore_whitespace: false,
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }
        
        fn pos(&self) -> Position {
            self.pos
        }
        
        // Additional helpers...
    }

    let parser = MockParser::new("\\u0061"); // Unicode escape for 'a'
    let result = parser.parse_escape();
    assert!(result.is_ok());
    match result {
        Ok(Primitive::Literal(literal)) => {
            assert_eq!(literal.c, 'a');
            assert_eq!(literal.kind, ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort));
        }
        _ => panic!("Expected a literal"),
    }
}

#[test]
fn test_parse_escape_with_valid_perl_class() {
    struct MockParser {
        pattern: String,
        pos: Position,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }
        
        // Additional helpers...
    }

    let parser = MockParser::new("\\w"); // Perl class for word
    let result = parser.parse_escape();
    assert!(result.is_ok());
    match result {
        Ok(Primitive::Perl(cls)) => {
            assert_eq!(cls.kind, ast::ClassPerlKind::Word);
            assert!(!cls.negated);
        }
        _ => panic!("Expected a Perl class"),
    }
}

#[test]
fn test_parse_escape_with_invalid_escape() {
    struct MockParser {
        pattern: String,
        pos: Position,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            MockParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }
        
        // Additional helpers...
    }

    let parser = MockParser::new("\\z"); // Invalid escape
    let result = parser.parse_escape();
    assert!(result.is_err());
    match result {
        Err(err) => assert_eq!(err.kind, ast::ErrorKind::EscapeUnrecognized),
        _ => panic!("Expected an error"),
    }
}

