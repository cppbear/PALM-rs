// Answer 0

#[test]
fn test_parse_flags_with_multiple_negations() {
    let pattern = "--"; // Test input to trigger the negation condition
    let span = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 0, line: 1, column: 2 },
    };

    struct TestParser {
        pattern: &'static str,
        pos: Cell<Position>,
        parse_called: bool,
    }

    impl TestParser {
        fn new(pattern: &'static str) -> Self {
            Self {
                pattern,
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                parse_called: false,
            }
        }
        
        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap()
        }
        
        fn bump(&self) -> bool {
            if self.pos.get().offset < self.pattern.len() - 1 {
                self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: self.pos.get().column + 1 });
                true
            } else {
                false
            }
        }
        
        fn span(&self) -> Span {
            span.clone()
        }
        
        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnexpectedEof,
                pattern: self.pattern.to_string(),
                span: span.clone(),
            }
        }
        
        fn parse_flag(&self) -> Result<ast::Flag> {
            Ok(ast::Flag::CaseInsensitive) // Any valid flag
        }
    }

    let parser = TestParser::new(pattern);
    let result = parser.parse_flags();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::FlagUnexpectedEof);
    }
}

#[test]
fn test_parse_flags_with_duplicate_flags() {
    let pattern = "ii"; // Two instances of valid flags that should trigger duplication
    let span = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 0, line: 1, column: 2 },
    };

    struct TestParser {
        pattern: &'static str,
        pos: Cell<Position>,
        parse_called: bool,
    }

    impl TestParser {
        fn new(pattern: &'static str) -> Self {
            Self {
                pattern,
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                parse_called: false,
            }
        }
        
        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap()
        }

        fn bump(&self) -> bool {
            if self.pos.get().offset < self.pattern.len() - 1 {
                self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: self.pos.get().column + 1 });
                true
            } else {
                false
            }
        }
        
        fn span(&self) -> Span {
            span.clone()
        }
        
        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.to_string(),
                span: span.clone(),
            }
        }

        fn parse_flag(&self) -> Result<ast::Flag> {
            match self.char() {
                'i' => Ok(ast::Flag::CaseInsensitive),
                _ => Err(self.error(self.span(), ast::ErrorKind::FlagUnrecognized)),
            }
        }
    }

    let parser = TestParser::new(pattern);
    let result = parser.parse_flags();
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(matches!(err.kind, ast::ErrorKind::FlagDuplicate { .. }));
    }
}

