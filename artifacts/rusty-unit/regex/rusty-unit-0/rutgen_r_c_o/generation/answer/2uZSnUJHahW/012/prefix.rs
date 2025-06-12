// Answer 0

#[test]
#[should_panic]
fn test_parse_hex_brace_unexpected_eof() {
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

        fn bump_and_bump_space(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            true
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }

        fn char(&self) -> char {
            '}' // Simulating a closing brace character
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }
    }

    let parser = TestParser::new();
    let kind = ast::HexLiteralKind::X;
    parser.parse_hex_brace(kind).unwrap();
}

#[test]
#[should_panic]
fn test_parse_hex_brace_empty_hex() {
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
        
        fn bump_and_bump_space(&self) -> bool {
            true // Simulating space handling, leading to empty hex
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }

        fn char(&self) -> char {
            '}' // Simulating ending without any hex input
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }
    }

    let parser = TestParser::new();
    let kind = ast::HexLiteralKind::UnicodeShort;
    parser.parse_hex_brace(kind).unwrap();
}

