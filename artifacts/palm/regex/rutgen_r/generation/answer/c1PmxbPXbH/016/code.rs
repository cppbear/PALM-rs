// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_question_mark_and_empty_ast() {
    struct TestParser {
        position: usize,
        input: Vec<char>,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.position]
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> Result<ast::Concat> {
            Err("Repetition missing".into())
        }
    }

    let mut parser = TestParser {
        position: 0,
        input: vec!['?', 'a'],
    };

    let concat = ast::Concat {
        asts: vec![Ast::Empty(())],
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Question);
    assert!(result.is_err());
}

#[test]
fn test_parse_uncounted_repetition_with_plus_and_flags_ast() {
    struct TestParser {
        position: usize,
        input: Vec<char>,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.position]
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, _: usize, _: ast::ErrorKind) -> Result<ast::Concat> {
            Err("Repetition missing".into())
        }
    }

    let mut parser = TestParser {
        position: 0,
        input: vec!['*', 'a'],
    };

    let concat = ast::Concat {
        asts: vec![Ast::Flags(())],
    };

    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Star);
    assert!(result.is_err());
}

