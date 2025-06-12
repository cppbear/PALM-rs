// Answer 0

#[test]
#[should_panic]
fn test_parse_uncounted_repetition_with_empty_ast() {
    struct MockParser {
        current_char: char,
        position: usize,
        concat: ast::Concat,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            self.position += 1;
            false  // Assume no '?' follows immediately
        }

        fn error(&self, _: Span, kind: ast::ErrorKind) -> ast::Error {
            // Generating a mock error for the sake of the test
            ast::Error { kind }
        }
        
        fn span(&self) -> Span {
            Span::new(self.position, self.position)  // Dummy span
        }
    }

    let mock_parser = MockParser {
        current_char: '?',  // Constraint: this must be true
        position: 0,
        concat: ast::Concat {
            asts: vec![Ast::Empty(())],  // Constraint: empty AST should trigger panic
        },
    };

    let result = mock_parser.parse_uncounted_repetition(mock_parser.concat, ast::RepetitionKind::ZeroOrMore);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_parse_uncounted_repetition_with_flags_ast() {
    struct MockParser {
        current_char: char,
        position: usize,
        concat: ast::Concat,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            self.position += 1;
            false  // Assume no '?' follows immediately
        }

        fn error(&self, _: Span, kind: ast::ErrorKind) -> ast::Error {
            // Generating a mock error for the sake of the test
            ast::Error { kind }
        }

        fn span(&self) -> Span {
            Span::new(self.position, self.position)  // Dummy span
        }
    }

    let mock_parser = MockParser {
        current_char: '?',  // Constraint: this must be true
        position: 0,
        concat: ast::Concat {
            asts: vec![Ast::Flags(())],  // Constraint: flags AST should trigger panic
        },
    };

    let result = mock_parser.parse_uncounted_repetition(mock_parser.concat, ast::RepetitionKind::OneOrMore);
    assert!(result.is_err());
}

