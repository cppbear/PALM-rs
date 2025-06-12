// Answer 0

#[test]
fn test_parse_counted_repetition_missing_opening_brace() {
    struct MockParser {
        input: String,
        position: Position,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.position.offset).unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn bump(&mut self) {
            self.position.offset += 1;
        }

        fn is_eof(&self) -> bool {
            self.position.offset >= self.input.len()
        }

        fn parse_decimal(&self) -> Result<u32> {
            Err(ast::Error {
                kind: ast::ErrorKind::DecimalEmpty,
                pattern: self.input.clone(),
                span: Span::new(self.position, self.position),
            })
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            !self.is_eof()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.input.clone(),
                span,
            }
        }
    }

    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![ast::Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))],
    };

    let mut parser = MockParser {
        input: "abc".to_string(),
        position: Position { offset: 0, line: 1, column: 1 },
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_unclosed_count() {
    struct MockParser {
        input: String,
        position: Position,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.position.offset).unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn bump(&mut self) {
            self.position.offset += 1;
        }

        fn is_eof(&self) -> bool {
            self.position.offset >= self.input.len()
        }

        fn parse_decimal(&self) -> Result<u32> {
            Ok(2)
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            true
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.input.clone(),
                span,
            }
        }
    }

    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![ast::Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))],
    };

    let mut parser = MockParser {
        input: "{}".to_string(),
        position: Position { offset: 1, line: 1, column: 2 },
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_valid_case() {
    struct MockParser {
        input: String,
        position: Position,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.position.offset).unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn bump(&mut self) {
            self.position.offset += 1;
        }

        fn is_eof(&self) -> bool {
            self.position.offset >= self.input.len()
        }

        fn parse_decimal(&self) -> Result<u32> {
            Ok(2)
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            true
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.input.clone(),
                span,
            }
        }
    }

    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![ast::Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))],
    };
    
    let mut parser = MockParser {
        input: "{2,3}".to_string(),
        position: Position { offset: 0, line: 1, column: 1 },
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_ok());
}

