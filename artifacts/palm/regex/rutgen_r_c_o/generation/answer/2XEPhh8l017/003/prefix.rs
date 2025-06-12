// Answer 0

#[test]
#[should_panic]
fn test_parse_hex_digits_invalid_digit() {
    struct ParserMock {
        pos: Cell<Position>,
        scratch: RefCell<String>,
    }

    impl ParserMock {
        fn new() -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                scratch: RefCell::new(String::new()),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            // Mock implementation returning true
            true
        }

        fn char(&self) -> char {
            // Return a non-hex character, e.g., 'g'
            'g'
        }

        fn span_char(&self) -> Span {
            Span { start: self.pos.get(), end: self.pos.get() }
        }

        fn error(&self, _span: Span, _error_kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
                pattern: String::from("mock pattern"),
                span: Span { start: self.pos.get(), end: self.pos.get() },
            }
        }

        fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            use ast::HexLiteralKind;

            let mut scratch = self.scratch.borrow_mut();
            scratch.clear();

            let start = self.pos.get();
            for i in 0..kind.digits() {
                if i > 0 && !self.bump_and_bump_space() {
                    return Err(self.error(self.span_char(), ast::ErrorKind::EscapeUnexpectedEof));
                }
                if !is_hex(self.char()) {
                    return Err(self.error(self.span_char(), ast::ErrorKind::EscapeHexInvalidDigit));
                }
                scratch.push(self.char());
            }
            // Final bump
            self.bump_and_bump_space();
            let end = self.pos.get();
            // Rest of the function would go here
            unimplemented!()
        }
    }

    let parser = ParserMock::new();
    let _ = parser.parse_hex_digits(ast::HexLiteralKind::X); // This should panic
}

#[test]
#[should_panic]
fn test_parse_hex_digits_invalid_digit_unicode_short() {
    struct ParserMock {
        pos: Cell<Position>,
        scratch: RefCell<String>,
    }

    impl ParserMock {
        fn new() -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                scratch: RefCell::new(String::new()),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            // Mock implementation returning true
            true
        }

        fn char(&self) -> char {
            // Return a non-hex character, e.g., 'k'
            'k'
        }

        fn span_char(&self) -> Span {
            Span { start: self.pos.get(), end: self.pos.get() }
        }

        fn error(&self, _span: Span, _error_kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
                pattern: String::from("mock pattern"),
                span: Span { start: self.pos.get(), end: self.pos.get() },
            }
        }

        fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            use ast::HexLiteralKind;

            let mut scratch = self.scratch.borrow_mut();
            scratch.clear();

            let start = self.pos.get();
            for i in 0..kind.digits() {
                if i > 0 && !self.bump_and_bump_space() {
                    return Err(self.error(self.span_char(), ast::ErrorKind::EscapeUnexpectedEof));
                }
                if !is_hex(self.char()) {
                    return Err(self.error(self.span_char(), ast::ErrorKind::EscapeHexInvalidDigit));
                }
                scratch.push(self.char());
            }
            // Final bump
            self.bump_and_bump_space();
            let end = self.pos.get();
            // Rest of the function would go here
            unimplemented!()
        }
    }

    let parser = ParserMock::new();
    let _ = parser.parse_hex_digits(ast::HexLiteralKind::UnicodeShort); // This should panic
}

#[test]
#[should_panic]
fn test_parse_hex_digits_invalid_digit_unicode_long() {
    struct ParserMock {
        pos: Cell<Position>,
        scratch: RefCell<String>,
    }

    impl ParserMock {
        fn new() -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                scratch: RefCell::new(String::new()),
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            // Mock implementation returning true
            true
        }

        fn char(&self) -> char {
            // Return a non-hex character, e.g., '!'
            '!'
        }

        fn span_char(&self) -> Span {
            Span { start: self.pos.get(), end: self.pos.get() }
        }

        fn error(&self, _span: Span, _error_kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
                pattern: String::from("mock pattern"),
                span: Span { start: self.pos.get(), end: self.pos.get() },
            }
        }

        fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            use ast::HexLiteralKind;

            let mut scratch = self.scratch.borrow_mut();
            scratch.clear();

            let start = self.pos.get();
            for i in 0..kind.digits() {
                if i > 0 && !self.bump_and_bump_space() {
                    return Err(self.error(self.span_char(), ast::ErrorKind::EscapeUnexpectedEof));
                }
                if !is_hex(self.char()) {
                    return Err(self.error(self.span_char(), ast::ErrorKind::EscapeHexInvalidDigit));
                }
                scratch.push(self.char());
            }
            // Final bump
            self.bump_and_bump_space();
            let end = self.pos.get();
            // Rest of the function would go here
            unimplemented!()
        }
    }

    let parser = ParserMock::new();
    let _ = parser.parse_hex_digits(ast::HexLiteralKind::UnicodeLong); // This should panic
}

