// Answer 0

#[test]
fn test_parse_escape_end_text() {
    struct TestParser {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap()
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            // Returning a dummy error for this test.
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos.clone(), self.pos.clone()),
            }
        }
        
        fn parser(&self) -> &Self {
            self
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }
    }

    let mut parser = TestParser {
        pattern: String::from("\\z"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let result = parser.parse_escape();

    assert!(result.is_ok());
    if let Ok(Primitive::Assertion(assertion)) = result {
        assert_eq!(assertion.kind, AssertionKind::EndText);
    }
}

#[test]
fn test_parse_escape_unicode_class() {
    struct TestParser {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap()
        }
        
        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            // Returning a dummy error for this test.
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos.clone(), self.pos.clone()),
            }
        }

        fn parser(&self) -> &Self {
            self
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }
    }

    let mut parser = TestParser {
        pattern: String::from("\\u1234"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let result = parser.parse_escape();

    assert!(result.is_ok());
    // Further assertions can be added to check for Unicode class output
}

#[test]
fn test_parse_escape_perl_class_digit() {
    struct TestParser {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap()
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            // Returning a dummy error for this test.
            ast::Error {
                kind: ast::ErrorKind::EscapeUnrecognized,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos.clone(), self.pos.clone()),
            }
        }

        fn parser(&self) -> &Self {
            self
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }
    }

    let mut parser = TestParser {
        pattern: String::from("\\d"),
        pos: Position { offset: 0, line: 1, column: 1 },
        octal: false,
    };

    let result = parser.parse_escape();

    assert!(result.is_ok());
    // Further assertions can be added to check for Perl class (digit) output
}

