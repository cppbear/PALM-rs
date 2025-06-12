// Answer 0

#[test]
fn test_parse_set_class_open_valid() {
    struct MockParser {
        input: Vec<char>,
        pos: Position,
    }
    
    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos.offset).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char() != '\0' {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let mut parser = MockParser::new("[abc]");
    let result = parser.parse_set_class_open();

    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "ClassUnclosed")]
fn test_parse_set_class_open_unclosed() {
    struct MockParser {
        input: Vec<char>,
        pos: Position,
    }
    
    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos.offset).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char() != '\0' {
                self.pos.offset += 1;
                false
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let mut parser = MockParser::new("[");
    let result = parser.parse_set_class_open();
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "ClassUnclosed")]
fn test_parse_set_class_open_empty_class() {
    struct MockParser {
        input: Vec<char>,
        pos: Position,
    }
    
    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos.offset).unwrap_or(&'\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.char() == '-' {
                self.pos.offset += 1;
                false
            } else {
                self.pos.offset += 1;
                true
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }
    }

    let mut parser = MockParser::new("[--]");
    let result = parser.parse_set_class_open();

    assert!(result.is_err());
}

