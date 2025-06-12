// Answer 0

#[test]
fn test_parse_escape_with_v_character() {
    struct TestParser {
        pos: Position,
        pattern: String,
        octal: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            '\\' // Starting an escape sequence
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1; // Simulate bumping
            true
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone()) // Simplified for testing
        }

        fn is_eof(&self) -> bool {
            false // Not at EOF for this test
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from("\\n"), // Test input
        octal: false, // Doesn't matter for this test
    };

    let result = parser.parse_escape();
    assert!(result.is_ok());
    if let Ok(Primitive::Literal(lit)) = result {
        assert_eq!(lit.kind, ast::LiteralKind::Special(ast::SpecialLiteralKind::LineFeed));
    } else {
        panic!("Expected a Literal variant, but got different type");
    }
}

#[test]
fn test_parse_escape_with_a_character() {
    struct TestParser {
        pos: Position,
        pattern: String,
        octal: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            '\\'
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1; // Simulate bumping
            true
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from("\\a"), // Test input
        octal: false,
    };

    let result = parser.parse_escape();
    assert!(result.is_ok());
    if let Ok(Primitive::Literal(lit)) = result {
        assert_eq!(lit.kind, ast::LiteralKind::Special(ast::SpecialLiteralKind::Bell));
    } else {
        panic!("Expected a Literal variant, but got different type");
    }
}

#[test]
fn test_parse_escape_with_meta_character() {
    struct TestParser {
        pos: Position,
        pattern: String,
        octal: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            '\\'
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            true
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span,
            }
        }

        fn ignore_whitespace(&self) -> bool {
            false // Simulate that whitespace is not ignored
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from("\\&"), // Test input (using a meta character '&')
        octal: false,
    };

    let result = parser.parse_escape();
    assert!(result.is_ok());
    if let Ok(Primitive::Literal(lit)) = result {
        assert_eq!(lit.kind, ast::LiteralKind::Punctuation);
    } else {
        panic!("Expected a Literal variant, but got different type");
    }
}

