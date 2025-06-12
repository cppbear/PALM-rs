// Answer 0

fn test_parse_uncounted_repetition_question() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            MockParser { chars, pos: 0 }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.chars.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }
        
        fn error(&self, _span: (), _kind: ()) -> () {
            panic!("Error occurred");
        }
    }

    let mut parser = MockParser::new(vec!['?', 'a']);
    let mut concat = ast::Concat { asts: vec![Ast::SomeValidAst] }; // Assume SomeValidAst is a proper instance of Ast
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Question);
    assert!(result.is_ok());
}

fn test_parse_uncounted_repetition_star() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            MockParser { chars, pos: 0 }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.chars.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }
        
        fn error(&self, _span: (), _kind: ()) -> () {
            panic!("Error occurred");
        }
    }

    let mut parser = MockParser::new(vec!['*', 'b']);
    let mut concat = ast::Concat { asts: vec![Ast::SomeValidAst] }; // Assume SomeValidAst is a proper instance of Ast
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Star);
    assert!(result.is_ok());
}

fn test_parse_uncounted_repetition_plus() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            MockParser { chars, pos: 0 }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.chars.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }
        
        fn error(&self, _span: (), _kind: ()) -> () {
            panic!("Error occurred");
        }
    }

    let mut parser = MockParser::new(vec!['+', 'c']);
    let mut concat = ast::Concat { asts: vec![Ast::SomeValidAst] }; // Assume SomeValidAst is a proper instance of Ast
    let result = parser.parse_uncounted_repetition(concat, ast::RepetitionKind::Plus);
    assert!(result.is_ok());
}

