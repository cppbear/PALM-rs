// Answer 0

#[test]
fn test_parse_uncounted_repetition_star() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn bump(&mut self) -> bool {
            self.position += 1;
            self.position < self.input.len()
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.position, self.position)
        }
    }

    let mut parser = MockParser {
        input: vec!['*'],
        position: 0,
    };
    
    let concat = ast::Concat { asts: vec![ast::Ast::Empty(ast::Span::new(0, 0))] };
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
    
    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_plus() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn bump(&mut self) -> bool {
            self.position += 1;
            self.position < self.input.len()
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.position, self.position)
        }
    }

    let mut parser = MockParser {
        input: vec!['+'],
        position: 0,
    };
    
    let concat = ast::Concat { asts: vec![ast::Ast::Empty(ast::Span::new(0, 0))] };
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::OneOrMore);
    
    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_question() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn bump(&mut self) -> bool {
            self.position += 1;
            self.position < self.input.len()
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.position, self.position)
        }
    }

    let mut parser = MockParser {
        input: vec!['?'],
        position: 0,
    };
    
    let concat = ast::Concat { asts: vec![ast::Ast::Empty(ast::Span::new(0, 0))] };
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrOne);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "RepetitionMissing")]
fn test_parse_uncounted_repetition_empty_concat() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn bump(&mut self) -> bool {
            self.position += 1;
            self.position < self.input.len()
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.position, self.position)
        }
    }

    let mut parser = MockParser {
        input: vec!['*'],
        position: 0,
    };

    let concat = ast::Concat { asts: vec![] };
    let _result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
}

