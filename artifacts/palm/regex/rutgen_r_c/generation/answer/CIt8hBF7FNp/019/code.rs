// Answer 0

#[test]
fn test_parse_set_class_open_empty_class_unclosed() {
    struct DummyParser {
        pos: Position,
        input: Vec<char>,
        pointer: usize,
    }

    impl DummyParser {
        fn new(input: &str) -> Self {
            DummyParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                input: input.chars().collect(),
                pointer: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pointer]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pointer += 1;
            self.pos.offset += 1;
            true
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.input.iter().collect(),
                span: Span::new(self.pos.clone(), self.pos.clone()),
            }
        }
    }

    let parser = DummyParser::new("[^");

    let result = parser.parse_set_class_open();

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassUnclosed);
    }
}

#[test]
fn test_parse_set_class_open_single_negation() {
    struct DummyParser {
        pos: Position,
        input: Vec<char>,
        pointer: usize,
    }

    impl DummyParser {
        fn new(input: &str) -> Self {
            DummyParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                input: input.chars().collect(),
                pointer: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pointer]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pointer += 1;
            self.pos.offset += 1;
            true
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.input.iter().collect(),
                span: Span::new(self.pos.clone(), self.pos.clone()),
            }
        }
    }

    let parser = DummyParser::new("[^-]");

    let result = parser.parse_set_class_open();

    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_open_multiple_negation() {
    struct DummyParser {
        pos: Position,
        input: Vec<char>,
        pointer: usize,
    }

    impl DummyParser {
        fn new(input: &str) -> Self {
            DummyParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                input: input.chars().collect(),
                pointer: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pointer]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pointer += 1;
            self.pos.offset += 1;
            true
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.input.iter().collect(),
                span: Span::new(self.pos.clone(), self.pos.clone()),
            }
        }
    }

    let parser = DummyParser::new("[-^-]");

    let result = parser.parse_set_class_open();

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassUnclosed);
    }
}

