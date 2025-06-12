// Answer 0

#[test]
fn test_parse_decimal_valid() {
    struct TestParser {
        pos: Cell<Position>,
        scratch: RefCell<String>,
    }

    impl TestParser {
        fn new() -> Self {
            TestParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                scratch: RefCell::new(String::new()),
            }
        }

        fn bump(&self) {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: self.pos.get().line, column: self.pos.get().column + 1 });
        }

        fn char(&self) -> char {
            '1' // Simulating a digit character for testing.
        }

        fn is_eof(&self) -> bool {
            false // Simulating that we're not at the end of the input.
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }
    }

    let parser = TestParser::new();
    let result = parser.parse_decimal(); // Assuming the method is implemented within TestParser context.
    assert_eq!(result, Ok(1));
}

#[test]
fn test_parse_decimal_empty() {
    struct TestParser {
        pos: Cell<Position>,
        scratch: RefCell<String>,
    }

    impl TestParser {
        fn new() -> Self {
            TestParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                scratch: RefCell::new(String::new()),
            }
        }

        fn bump(&self) {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: self.pos.get().line, column: self.pos.get().column + 1 });
        }

        fn char(&self) -> char {
            ' ' // Simulating a whitespace character.
        }

        fn is_eof(&self) -> bool {
            false // Simulating that we're not at the end of the input.
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: String::from(""),
                span,
            }
        }
    }

    let parser = TestParser::new();
    let result = parser.parse_decimal();
    let expected_error = ast::ErrorKind::DecimalEmpty;
    assert_eq!(result, Err(parser.error(Span::new(parser.pos(), parser.pos()), expected_error)));
}

#[test]
fn test_parse_decimal_invalid() {
    struct TestParser {
        pos: Cell<Position>,
        scratch: RefCell<String>,
    }

    impl TestParser {
        fn new() -> Self {
            TestParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                scratch: RefCell::new(String::new()),
            }
        }

        fn bump(&self) {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: self.pos.get().line, column: self.pos.get().column + 1 });
        }

        fn char(&self) -> char {
            'a' // Simulating an invalid character (not a digit).
        }

        fn is_eof(&self) -> bool {
            false // Simulating that we're not at the end of the input.
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: String::from(""),
                span,
            }
        }
    }

    let parser = TestParser::new();
    let result = parser.parse_decimal();
    let expected_error = ast::ErrorKind::DecimalInvalid;
    assert_eq!(result, Err(parser.error(Span::new(parser.pos(), parser.pos()), expected_error)));
}

