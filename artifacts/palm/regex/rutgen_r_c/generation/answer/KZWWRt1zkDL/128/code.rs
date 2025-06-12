// Answer 0

#[test]
fn test_parse_escape_with_valid_assertion_A() {
    struct DummyParser {
        input: String,
        index: usize,
        octal: bool,
    }

    impl DummyParser {
        fn bump(&mut self) -> bool {
            if self.index < self.input.len() {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.index..].chars().next().unwrap_or('\0')
        }

        fn parser(&self) -> &DummyParser {
            self
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Error {
            Error {
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: self.input.clone(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
            }
        }
    }

    let mut parser = DummyParser {
        input: "\\A".to_string(),
        index: 0,
        octal: true,
    };

    let result = parser.parse_escape();
    
    assert!(result.is_ok());
    if let Ok(Primitive::Assertion(assertion)) = result {
        assert_eq!(assertion.kind, AssertionKind::StartText);
    } else {
        panic!("Expected assertion kind for '\\A'");
    }
}

#[test]
fn test_parse_escape_with_valid_assertion_z() {
    struct DummyParser {
        input: String,
        index: usize,
        octal: bool,
    }

    impl DummyParser {
        fn bump(&mut self) -> bool {
            if self.index < self.input.len() {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.index..].chars().next().unwrap_or('\0')
        }

        fn parser(&self) -> &DummyParser {
            self
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Error {
            Error {
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: self.input.clone(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
            }
        }
    }

    let mut parser = DummyParser {
        input: "\\z".to_string(),
        index: 0,
        octal: true,
    };

    let result = parser.parse_escape();
    
    assert!(result.is_ok());
    if let Ok(Primitive::Assertion(assertion)) = result {
        assert_eq!(assertion.kind, AssertionKind::EndText);
    } else {
        panic!("Expected assertion kind for '\\z'");
    }
}

#[test]
fn test_parse_escape_with_valid_literal() {
    struct DummyParser {
        input: String,
        index: usize,
        octal: bool,
    }

    impl DummyParser {
        fn bump(&mut self) -> bool {
            if self.index < self.input.len() {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input[self.index..].chars().next().unwrap_or('\0')
        }

        fn parser(&self) -> &DummyParser {
            self
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Error {
            Error {
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: self.input.clone(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
            }
        }
    }

    let mut parser = DummyParser {
        input: "\\d".to_string(),
        index: 0,
        octal: true,
    };

    let result = parser.parse_escape();
    
    assert!(result.is_ok());
    if let Ok(Primitive::Perl(cls)) = result {
        assert_eq!(cls.kind, ast::ClassPerlKind::Digit);
        assert_eq!(cls.negated, false);
    } else {
        panic!("Expected Perl class for '\\d'");
    }
}

