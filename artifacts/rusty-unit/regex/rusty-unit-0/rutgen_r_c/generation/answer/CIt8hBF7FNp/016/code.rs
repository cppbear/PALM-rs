// Answer 0

#[test]
fn test_parse_set_class_open_non_negated_with_class_unclosed() {
    struct TestParser {
        input: &'static str,
        pos: Position,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos.offset += 1; // skip whitespace
            }
            self.pos.offset += 1; // move to next char
            self.char() != '\0'
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Error {
            Error { kind, pattern: self.input.to_string(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }
    }

    let mut parser = TestParser {
        input: "[abc",
        pos: Position { offset: 0, line: 1, column: 1 },
    };

    let result = parser.parse_set_class_open();

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassUnclosed);
    }
}

#[test]
fn test_parse_set_class_open_negated_with_class_unclosed() {
    struct TestParser {
        input: &'static str,
        pos: Position,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos.offset += 1; // skip whitespace
            }
            self.pos.offset += 1; // move to next char
            self.char() != '\0'
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Error {
            Error { kind, pattern: self.input.to_string(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }
    }

    let mut parser = TestParser {
        input: "[^abc",
        pos: Position { offset: 0, line: 1, column: 1 },
    };

    let result = parser.parse_set_class_open();

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassUnclosed);
    }
}

#[test]
fn test_parse_set_class_open_empty_class_with_class_unclosed() {
    struct TestParser {
        input: &'static str,
        pos: Position,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char().is_whitespace() {
                self.pos.offset += 1; // skip whitespace
            }
            self.pos.offset += 1; // move to next char
            self.char() != '\0'
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Error {
            Error { kind, pattern: self.input.to_string(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }
    }

    let mut parser = TestParser {
        input: "[]abc",
        pos: Position { offset: 0, line: 1, column: 1 },
    };

    let result = parser.parse_set_class_open();

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassUnclosed);
    }
}

