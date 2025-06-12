// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_char_start() {
    struct MockParser {
        chars: Vec<char>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, index: 0 }
        }

        fn char(&self) -> char {
            *self.chars.get(self.index).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.index += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            true
        }

        fn parse_decimal(&self) -> Result<u32> {
            Err(ast::Error {
                kind: ast::ErrorKind::DecimalEmpty,
                pattern: String::new(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 },
                                Position { offset: 0, line: 1, column: 1 }),
            })
        }

        fn is_eof(&self) -> bool {
            self.index >= self.chars.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn pos(&self) -> Position {
            Position { offset: self.index, line: 1, column: self.index as usize + 1 }
        }
    }

    let parser = MockParser::new(vec!['a']); // Starting character is 'a', not '{'
    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![ast::Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 },
                                                       Position { offset: 0, line: 1, column: 1 }))],
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}


#[test]
fn test_parse_counted_repetition_empty_concat() {
    struct MockParser {
        chars: Vec<char>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, index: 0 }
        }

        fn char(&self) -> char {
            *self.chars.get(self.index).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.index += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            true
        }

        fn parse_decimal(&self) -> Result<u32> {
            Err(ast::Error {
                kind: ast::ErrorKind::DecimalEmpty,
                pattern: String::new(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 },
                                Position { offset: 0, line: 1, column: 1 }),
            })
        }

        fn is_eof(&self) -> bool {
            self.index >= self.chars.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn pos(&self) -> Position {
            Position { offset: self.index, line: 1, column: self.index as usize + 1 }
        }
    }

    let parser = MockParser::new(vec!['{', ' ']); // Positioned at the '{'
    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![], // Empty asts to trigger panic condition
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_invalid_decimal() {
    struct MockParser {
        chars: Vec<char>,
        index: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, index: 0 }
        }

        fn char(&self) -> char {
            *self.chars.get(self.index).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.index += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            true
        }

        fn parse_decimal(&self) -> Result<u32> {
            Err(ast::Error {
                kind: ast::ErrorKind::DecimalEmpty,
                pattern: String::new(),
                span: Span::new(Position { offset: 0, line: 1, column: 1 },
                                Position { offset: 0, line: 1, column: 1 }),
            })
        }

        fn is_eof(&self) -> bool {
            self.index >= self.chars.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn pos(&self) -> Position {
            Position { offset: self.index, line: 1, column: self.index as usize + 1 }
        }
    }

    let parser = MockParser::new(vec!['{', ' ', '1', ' ', ',', ' ', '}', ' ']); // valid start but invalid decimal
    let concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 }),
        asts: vec![ast::Ast::Empty(Span::new(Position { offset: 0, line: 1, column: 1 },
                                                       Position { offset: 0, line: 1, column: 1 }))],
    };

    let result = parser.parse_counted_repetition(concat);
    assert!(result.is_err());
}

