// Answer 0

#[test]
fn test_parse_set_class_open_unclosed_class() {
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

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulate returning false to indicate unclosed class
            false
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<(ast::ClassBracketed, ast::ClassSetUnion), ()> {
            Err(())
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser::new("[");
    let result = parser.parse_set_class_open();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_open_incomplete_close() {
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

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulate returning false to indicate unclosed class
            false
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<(ast::ClassBracketed, ast::ClassSetUnion), ()> {
            Err(())
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser::new("[^");
    let result = parser.parse_set_class_open();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_open_only_negation() {
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

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulate returning false to indicate unclosed class
            false
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<(ast::ClassBracketed, ast::ClassSetUnion), ()> {
            Err(())
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser::new("[^-]");
    let result = parser.parse_set_class_open();
    assert!(result.is_err());
}

