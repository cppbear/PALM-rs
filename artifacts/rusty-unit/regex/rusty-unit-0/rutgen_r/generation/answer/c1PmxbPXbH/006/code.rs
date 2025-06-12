// Answer 0

#[test]
fn test_parse_uncounted_repetition_empty_concat() {
    struct MockParser {
        pos: usize,
        chars: Vec<char>,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self { pos: 0, chars }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.chars.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new(span, kind)
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.pos, self.pos + 1)
        }
    }

    let parser = MockParser::new(vec!['+']); // This should invoke the repetition operator condition
    let concat = ast::Concat { asts: vec![] }; // Empty concat to trigger panic condition

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Plus);
    
    assert!(result.is_err());
    if let Err(err) = result {
        // Here you can check if the error matches your expectations
        assert_eq!(err.kind(), ast::ErrorKind::RepetitionMissing);
    }
}

#[test]
fn test_parse_uncounted_repetition_invalid_char() {
    struct MockParser {
        pos: usize,
        chars: Vec<char>,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self { pos: 0, chars }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.chars.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new(span, kind)
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.pos, self.pos + 1)
        }
    }

    let parser = MockParser::new(vec!['a']); // Invalid character, not '?', '*', or '+'
    let concat = ast::Concat { asts: vec![ast::Ast::Empty(ast::Span::new(0, 0))] }; // Contains a valid entry

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Plus);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind(), ast::ErrorKind::RepetitionMissing);
    }
}

