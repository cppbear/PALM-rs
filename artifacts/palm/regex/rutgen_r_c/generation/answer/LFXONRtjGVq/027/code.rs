// Answer 0

#[test]
fn test_parse_counted_repetition_valid_case() {
    struct MockParser {
        input: Vec<char>,
        pos: Position,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.input.get(self.pos.offset).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.pos.offset += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            if self.is_eof() {
                return false;
            }
            self.bump();
            true
        }
        
        fn parse_decimal(&self) -> Result<u32> {
            if !self.is_eof() && self.char().is_digit(10) {
                Ok(self.char().to_digit(10).unwrap())
            } else {
                Err(ast::Error { kind: ast::ErrorKind::DecimalEmpty, pattern: "".to_string(), span: Span::new(self.pos, self.pos) })
            }
        }
    }

    let mut concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![ast::Ast::Literal(ast::Literal { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'a' })],
    };
    
    let mut parser = MockParser { input: vec!['{', '1', ',', '2', '}', '\0'], pos: Position { offset: 0, line: 1, column: 1 } };

    let result = parser.parse_counted_repetition(concat.clone()).unwrap();
    assert!(result.asts.len() == 1);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_invalid_case_no_open_brace() {
    struct MockParser {
        input: Vec<char>,
        pos: Position,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.input.get(self.pos.offset).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.pos.offset += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }
        
        fn parse_decimal(&self) -> Result<u32> {
            if !self.is_eof() && self.char().is_digit(10) {
                Ok(self.char().to_digit(10).unwrap())
            } else {
                Err(ast::Error { kind: ast::ErrorKind::DecimalEmpty, pattern: "".to_string(), span: Span::new(self.pos, self.pos) })
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            if self.is_eof() {
                return false;
            }
            self.bump();
            true
        }
    }

    let mut concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![ast::Ast::Literal(ast::Literal { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'a' })],
    };

    let mut parser = MockParser { input: vec!['1', ',', '2', '}', '\0'], pos: Position { offset: 0, line: 1, column: 1 } };

    parser.parse_counted_repetition(concat).unwrap();
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_invalid_case_repetition_missing() {
    struct MockParser {
        input: Vec<char>,
        pos: Position,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.input.get(self.pos.offset).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.pos.offset += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            if self.is_eof() {
                return false;
            }
            self.bump();
            true
        }
        
        fn parse_decimal(&self) -> Result<u32> {
            Err(ast::Error { kind: ast::ErrorKind::DecimalEmpty, pattern: "".to_string(), span: Span::new(self.pos, self.pos) })
        }
    }

    let mut concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: Vec::new(),
    };

    let mut parser = MockParser { input: vec!['{', '1', ',', '2', '}', '\0'], pos: Position { offset: 0, line: 1, column: 1 } };

    parser.parse_counted_repetition(concat).unwrap();
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_invalid_case_unclosed_brace() {
    struct MockParser {
        input: Vec<char>,
        pos: Position,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.input.get(self.pos.offset).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.pos.offset += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
            if self.is_eof() {
                return false;
            }
            self.bump();
            true
        }

        fn parse_decimal(&self) -> Result<u32> {
            if !self.is_eof() && self.char().is_digit(10) {
                Ok(self.char().to_digit(10).unwrap())
            } else {
                Err(ast::Error { kind: ast::ErrorKind::DecimalEmpty, pattern: "".to_string(), span: Span::new(self.pos, self.pos) })
            }
        }
    }

    let mut concat = ast::Concat {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        asts: vec![ast::Ast::Literal(ast::Literal { span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'a' })],
    };
    
    let mut parser = MockParser { input: vec!['{', '1', '2', '\0'], pos: Position { offset: 0, line: 1, column: 1 } };

    parser.parse_counted_repetition(concat).unwrap();
}

