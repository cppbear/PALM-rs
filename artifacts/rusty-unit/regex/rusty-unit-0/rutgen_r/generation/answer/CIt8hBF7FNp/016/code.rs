// Answer 0

#[test]
fn test_parse_set_class_open_error_eof() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0') // Simulate EOF with '\0'
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.chars.len() {
                self.pos += 1; // Move to next character
                true
            } else {
                false // Simulate EOF
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<(ast::ClassBracketed, ast::ClassSetUnion), ast::Error> {
            Err(ast::Error::new("Unclosed class"))
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser {
        chars: vec!['[', '^'],
        pos: 0,
    };

    let result = parser.parse_set_class_open();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_open_needs_negation() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0') // Simulate EOF with '\0'
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.chars.len() {
                self.pos += 1; // Move to next character
                true
            } else {
                false // Simulate EOF
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<(ast::ClassBracketed, ast::ClassSetUnion), ast::Error> {
            Err(ast::Error::new("Unclosed class"))
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser {
        chars: vec!['[', '^', '-'],
        pos: 0,
    };

    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_open_empty_class() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap_or(&'\0') // Simulate EOF with '\0'
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.chars.len() {
                self.pos += 1; // Move to next character
                true
            } else {
                false // Simulate EOF
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<(ast::ClassBracketed, ast::ClassSetUnion), ast::Error> {
            Err(ast::Error::new("Unclosed class"))
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser {
        chars: vec!['[', ']'],
        pos: 0,
    };

    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
}

