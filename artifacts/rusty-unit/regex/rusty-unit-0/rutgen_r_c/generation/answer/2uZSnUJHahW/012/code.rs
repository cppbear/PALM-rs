// Answer 0

#[test]
fn test_parse_hex_brace_eof_error() {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::result;

    struct MockParser {
        scratch: RefCell<String>,
        pos: Cell<Position>,
        eof: Cell<bool>,
        char_at_pos: char,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                eof: Cell::new(false),
                char_at_pos: '{',
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            self.eof.get()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("mock_pattern"), span }
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos.get(), self.pos.get())
        }

        fn bump(&self) {
            // Mock bump, updating the position
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: self.pos.get().column + 1 });
        }

        fn char(&self) -> char {
            self.char_at_pos
        }

        fn set_eof(&self, eof: bool) {
            self.eof.set(eof);
        }
    }

    let mut parser = MockParser::new();
    parser.set_eof(true);

    let parser_i = ParserI {
        parser: Rc::new(parser),
        pattern: "{123}".as_ref(),
    };

    let result = parser_i.parse_hex_brace(ast::HexLiteralKind::X);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::EscapeUnexpectedEof);
    }
}

