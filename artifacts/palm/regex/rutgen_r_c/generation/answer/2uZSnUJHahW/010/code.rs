// Answer 0

#[test]
fn test_parse_hex_brace_empty_hex() {
    struct MockParser {
        scratch: RefCell<String>,
        pos: Cell<Position>,
        input: String,
    }

    impl MockParser {
        pub fn new(input: &str) -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                input: input.to_string(),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: 1 });
            true
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.get().offset).unwrap_or('}')
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.input.clone(), span: Span::new(self.pos.get(), self.pos.get()) }
        }

        fn is_eof(&self) -> bool {
            self.char() == '\0' || self.pos.get().offset >= self.input.len()
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos.get(), self.pos.get())
        }

        fn parser(&self) -> &MockParser {
            self
        }
    }

    let parser = MockParser::new("{");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::X);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind, ast::ErrorKind::EscapeHexEmpty);
    }
}

#[test]
fn test_parse_hex_brace_invalid_hex() {
    struct MockParser {
        scratch: RefCell<String>,
        pos: Cell<Position>,
        input: String,
    }

    impl MockParser {
        pub fn new(input: &str) -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                input: input.to_string(),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: 1 });
            true
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos.get().offset).unwrap_or('}')
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.input.clone(), span: Span::new(self.pos.get(), self.pos.get()) }
        }

        fn is_eof(&self) -> bool {
            self.char() == '\0' || self.pos.get().offset >= self.input.len()
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos.get(), self.pos.get())
        }

        fn parser(&self) -> &MockParser {
            self
        }
    }

    let parser = MockParser::new("{G}");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::X);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind, ast::ErrorKind::EscapeHexInvalid);
    }
}

