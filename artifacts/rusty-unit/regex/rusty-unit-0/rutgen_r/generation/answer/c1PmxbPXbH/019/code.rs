// Answer 0

#[test]
fn test_parse_uncounted_repetition_question() {
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
            if self.pos + 1 < self.chars.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, _span: (), _kind: ()) -> ast::Error {
            ast::Error {}  // Assuming `ast::Error` has a default constructor.
        }

        fn span(&self) -> () {
            () // Assuming the reusable span is irrelevant for this test.
        }
    }

    let mut concat = ast::Concat { asts: Vec::new() };
    concat.asts.push(ast::Ast::NonEmptyVariant {}); // Assuming NonEmptyVariant is a valid variant.

    let mut parser = MockParser::new(vec!['?', ' ']); // Simulate the parser state.

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Questionmark);
    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_star() {
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
            if self.pos + 1 < self.chars.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, _span: (), _kind: ()) -> ast::Error {
            ast::Error {} // Assuming `ast::Error` has a default constructor.
        }

        fn span(&self) -> () {
            () // Assuming the reusable span is irrelevant for this test.
        }
    }

    let mut concat = ast::Concat { asts: Vec::new() };
    concat.asts.push(ast::Ast::NonEmptyVariant {}); // Assuming NonEmptyVariant is a valid variant.

    let mut parser = MockParser::new(vec!['*', ' ']); // Simulate the parser state.

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Star);
    assert!(result.is_ok());
}

#[test]
fn test_parse_uncounted_repetition_plus() {
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
            if self.pos + 1 < self.chars.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, _span: (), _kind: ()) -> ast::Error {
            ast::Error {} // Assuming `ast::Error` has a default constructor.
        }

        fn span(&self) -> () {
            () // Assuming the reusable span is irrelevant for this test.
        }
    }

    let mut concat = ast::Concat { asts: Vec::new() };
    concat.asts.push(ast::Ast::NonEmptyVariant {}); // Assuming NonEmptyVariant is a valid variant.

    let mut parser = MockParser::new(vec!['+', ' ']); // Simulate the parser state.

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Plus);
    assert!(result.is_ok());
}

