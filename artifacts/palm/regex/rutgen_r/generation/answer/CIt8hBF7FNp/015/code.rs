// Answer 0

#[test]
fn test_parse_set_class_open_with_negation_and_literal_dash() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1; // bump over the current character
                // Assuming space handling is simple, just consider next
                // character as non-space if it exists.
                return true;
            }
            false
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<_, _> {
            Err("Error".into())
        }

        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos() + 1)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos() + 1)
        }
    }

    let mut parser = TestParser {
        input: vec!['[', '^', '-', ']', 'a'],
        pos: 0,
    };

    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_open_with_lit_dash() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1; // bump over the current character
                return true;
            }
            false
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<_, _> {
            Err("Error".into())
        }

        fn span(&self) -> Span {
            Span::new(self.pos(), self.pos() + 1)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos() + 1)
        }
    }

    let mut parser = TestParser {
        input: vec!['[', '-', 'x', ']', 'y'],
        pos: 0,
    };

    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
}

