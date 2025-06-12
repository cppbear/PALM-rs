// Answer 0

#[test]
fn test_parse_uncounted_repetition_question_mark_greedy() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Error {
            // Simplified error for testing.
            Error::new()
        }
    }

    let parser = MockParser::new("?");
    let concat = ast::Concat {
        asts: vec![Ast::SomeExistingAst], // Placeholder for valid ast.
    };
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Question);

    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_star_non_greedy() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Error {
            // Simplified error for testing.
            Error::new()
        }
    }

    let parser = MockParser::new("*?");
    let concat = ast::Concat {
        asts: vec![Ast::SomeExistingAst], // Placeholder for valid ast.
    };
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Star);

    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_plus_non_greedy() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Error {
            // Simplified error for testing.
            Error::new()
        }
    }

    let parser = MockParser::new("+?");
    let concat = ast::Concat {
        asts: vec![Ast::SomeExistingAst], // Placeholder for valid ast.
    };
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Plus);

    assert!(result.is_ok());
}

