// Answer 0

#[test]
fn test_parse_uncounted_repetition_question() {
    struct MockParser {
        position: usize,
        chars: Vec<char>,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            self.position += 1;
            self.position < self.chars.len()
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.pos(), self.pos() + 1)
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }
    }

    let mut parser = MockParser {
        position: 0,
        chars: vec!['?', 'c'],
    };

    let concat = ast::Concat {
        asts: vec![ast::Ast::Literal('c')],
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore);
    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_star() {
    struct MockParser {
        position: usize,
        chars: Vec<char>,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            self.position += 1;
            self.position < self.chars.len()
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.pos(), self.pos() + 1)
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }
    }

    let mut parser = MockParser {
        position: 0,
        chars: vec!['*', 'd'],
    };

    let concat = ast::Concat {
        asts: vec![ast::Ast::Literal('d')],
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::OneOrMore);
    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_plus() {
    struct MockParser {
        position: usize,
        chars: Vec<char>,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.position).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            self.position += 1;
            self.position < self.chars.len()
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.pos(), self.pos() + 1)
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }
    }

    let mut parser = MockParser {
        position: 0,
        chars: vec!['+', 'e'],
    };

    let concat = ast::Concat {
        asts: vec![ast::Ast::Literal('e')],
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Plus);
    assert!(result.is_ok());
}

