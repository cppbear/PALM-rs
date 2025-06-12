// Answer 0

#[test]
fn test_parse_hex_digits_empty_string() {
    struct TestParser {
        scratch: RefCell<String>,
        pos: Cell<Position>,
    }

    impl TestParser {
        fn new() -> Self {
            TestParser {
                scratch: RefCell::new(String::new()),
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            // Simulating passing through whitespace
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: 2 });
            true
        }

        fn char(&self) -> char {
            // Simulating an empty input scenario
            '\0' 
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("test"), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos.get(), self.pos.get())
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }
        
        fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            // The original function as provided
            let mut scratch = self.scratch.borrow_mut();
            scratch.clear();
            let start = self.pos();
            for i in 0..kind.digits() {
                if i > 0 && !self.bump_and_bump_space() {
                    return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
                }
                if !is_hex(self.char()) {
                    return Err(self.error(self.span(), ast::ErrorKind::EscapeHexInvalidDigit));
                }
                scratch.push(self.char());
            }
            self.bump_and_bump_space();
            let end = self.pos();
            let hex = scratch.as_str();
            match u32::from_str_radix(hex, 16).ok().and_then(char::from_u32) {
                None => {
                    Err(self.error(Span::new(start, end), ast::ErrorKind::EscapeHexInvalid))
                }
                Some(c) => {
                    Ok(ast::Literal {
                        span: Span::new(start, end),
                        kind: ast::LiteralKind::HexFixed(kind),
                        c: c,
                    })
                }
            }
        }
    }

    let parser = TestParser::new();
    let result = parser.parse_hex_digits(ast::HexLiteralKind::X);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::EscapeHexInvalid);
    }
}

#[test]
#[should_panic]
fn test_parse_hex_digits_invalid_digit() {
    struct TestParser {
        scratch: RefCell<String>,
        pos: Cell<Position>,
    }

    impl TestParser {
        fn new() -> Self {
            TestParser {
                scratch: RefCell::new(String::from("g")), // Invalid hex character
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            // Simulating passing through whitespace
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: 2 });
            true
        }

        fn char(&self) -> char {
            self.scratch.borrow().chars().next().unwrap() 
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from("test"), span }
        }

        fn span(&self) -> Span {
            Span::new(self.pos.get(), self.pos.get())
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }
        
        fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            // The original function as provided
            let mut scratch = self.scratch.borrow_mut();
            scratch.clear();
            let start = self.pos();
            for i in 0..kind.digits() {
                if i > 0 && !self.bump_and_bump_space() {
                    return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
                }
                if !is_hex(self.char()) {
                    return Err(self.error(self.span(), ast::ErrorKind::EscapeHexInvalidDigit));
                }
                scratch.push(self.char());
            }
            self.bump_and_bump_space();
            let end = self.pos();
            let hex = scratch.as_str();
            match u32::from_str_radix(hex, 16).ok().and_then(char::from_u32) {
                None => {
                    Err(self.error(Span::new(start, end), ast::ErrorKind::EscapeHexInvalid))
                }
                Some(c) => {
                    Ok(ast::Literal {
                        span: Span::new(start, end),
                        kind: ast::LiteralKind::HexFixed(kind),
                        c: c,
                    })
                }
            }
        }
    }

    let parser = TestParser::new();
    parser.parse_hex_digits(ast::HexLiteralKind::X);
}

