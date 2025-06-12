// Answer 0

#[test]
fn test_parse_counted_repetition_char_not_open_bracket() {
    struct TestParser {
        char: char,
        asts: Vec<ast::Ast>,
        pos: Position,
        eof: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            true
        }
        
        fn parse_decimal(&self) -> Result<u32> {
            Ok(1)
        }
        
        fn is_eof(&self) -> bool {
            self.eof
        }
        
        fn pos(&self) -> Position {
            self.pos
        }
        
        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
        
        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::RepetitionCountUnclosed, pattern: String::new(), span: _span }
        }
    }

    let mut concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![ast::Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))],
    };

    let mut parser = TestParser {
        char: 'a',
        asts: concat.asts.clone(),
        pos: Position { offset: 0, line: 1, column: 1 },
        eof: false,
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::RepetitionCountUnclosed);
    }
}

#[test]
fn test_parse_counted_repetition_no_ast() {
    struct TestParser {
        char: char,
        asts: Vec<ast::Ast>,
        pos: Position,
        eof: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            '{'
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            true
        }
        
        fn parse_decimal(&self) -> Result<u32> {
            Ok(1)
        }
        
        fn is_eof(&self) -> bool {
            false
        }
        
        fn pos(&self) -> Position {
            self.pos
        }
        
        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
        
        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::RepetitionMissing, pattern: String::new(), span: _span }
        }
    }

    let mut concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![],
    };

    let mut parser = TestParser {
        char: '{',
        asts: concat.asts.clone(),
        pos: Position { offset: 0, line: 1, column: 1 },
        eof: false,
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::RepetitionMissing);
    }
}

#[test]
fn test_parse_counted_repetition_count_unclosed() {
    struct TestParser {
        char: char,
        asts: Vec<ast::Ast>,
        pos: Position,
        eof: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            '{'
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            true
        }
        
        fn parse_decimal(&self) -> Result<u32> {
            Ok(1)
        }
        
        fn is_eof(&self) -> bool {
            false
        }
        
        fn pos(&self) -> Position {
            self.pos
        }
        
        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
        
        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind: ast::ErrorKind::RepetitionCountUnclosed, pattern: String::new(), span: _span }
        }
    }

    let mut concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![ast::Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))],
    };

    let mut parser = TestParser {
        char: '{',
        asts: concat.asts.clone(),
        pos: Position { offset: 0, line: 1, column: 1 },
        eof: false,
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::RepetitionCountUnclosed);
    }
}

