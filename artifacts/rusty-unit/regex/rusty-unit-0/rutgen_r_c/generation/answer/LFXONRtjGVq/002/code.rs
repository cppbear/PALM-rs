// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_char() {
    struct TestParser {
        char_position: usize,
        is_eof: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            if self.char_position == 0 {
                '{'
            } else {
                'x' 
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.char_position += 1;
            true
        }

        fn parse_decimal(&self) -> Result<u32> {
            Ok(2)
        }

        fn is_eof(&self) -> bool {
            self.is_eof
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<ast::Concat> {
            Err(ast::Error { kind, pattern: "".to_string(), span })
        }

        fn pos(&self) -> Position {
            Position { offset: self.char_position, line: 1, column: 1 }
        }

        fn span(&self) -> Span {
            Span { start: self.pos(), end: self.pos() }
        }
    }

    let mut concat = ast::Concat { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), asts: vec![Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))] };

    let mut parser = TestParser { char_position: 1, is_eof: true };
    
    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_invalid_repetition() {
    struct TestParser {
        char_position: usize,
        eof: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            '{'
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.char_position += 1;
            true
        }

        fn parse_decimal(&self) -> Result<u32> {
            Ok(2) // simulate valid decimal
        }

        fn is_eof(&self) -> bool {
            true
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<ast::Concat> {
            Err(ast::Error { kind, pattern: "".to_string(), span })
        }

        fn pos(&self) -> Position {
            Position { offset: self.char_position, line: 1, column: 1 }
        }

        fn span(&self) -> Span {
            Span { start: self.pos(), end: self.pos() }
        }
    }

    let mut concat = ast::Concat { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), asts: vec![Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }))] };

    let mut parser = TestParser { char_position: 0, eof: false }; // eof is false here
    
    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

