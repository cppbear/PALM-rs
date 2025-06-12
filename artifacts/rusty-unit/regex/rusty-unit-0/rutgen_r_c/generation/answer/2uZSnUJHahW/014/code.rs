// Answer 0

#[test]
fn test_parse_hex_brace_valid() {
    struct FakeParser {
        scratch: RefCell<String>,
        pos: Position,
        char: char,
        eof: bool,
    }

    impl FakeParser {
        fn new() -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                pos: Position { offset: 0, line: 1, column: 1 },
                char: '1',
                eof: false,
            }
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            // Simulating successful bump
            self.pos.offset += 1;
            true
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind,
                pattern: String::new(),
                span,
            }
        }

        fn char(&self) -> char {
            self.char
        }

        fn bump(&mut self) {
            self.char = '}';
            self.pos.offset += 1;
        }
    }

    let parser = FakeParser::new();
    let parser_i = ParserI {
        parser,
        pattern: "{1}".as_ref(),
    };

    let result = parser_i.parse_hex_brace(ast::HexLiteralKind::X);
    
    assert!(result.is_ok());
    if let Ok(ast::Literal { span, kind, c }) = result {
        assert_eq!(kind, ast::LiteralKind::HexBrace(ast::HexLiteralKind::X));
        assert_eq!(c, '1');
    }
}

#[test]
fn test_parse_hex_brace_empty_hex() {
    struct FakeParser {
        scratch: RefCell<String>,
        pos: Position,
        char: char,
        eof: bool,
    }

    impl FakeParser {
        fn new() -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                pos: Position { offset: 0, line: 1, column: 1 },
                char: '}',
                eof: false,
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulating successful bump
            self.pos.offset += 1;
            false // simulating no successful bump
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind,
                pattern: String::new(),
                span,
            }
        }

        fn char(&self) -> char {
            self.char
        }

        fn bump(&mut self) {
            self.char = '}';
            self.pos.offset += 1;
        }
    }

    let parser = FakeParser::new();
    let parser_i = ParserI {
        parser,
        pattern: "{}".as_ref(),
    };

    let result = parser_i.parse_hex_brace(ast::HexLiteralKind::X);
    
    assert!(result.is_err());
    if let Err(Error { kind, .. }) = result {
        assert_eq!(kind, ErrorKind::EscapeHexEmpty);
    }
}

#[test]
fn test_parse_hex_brace_invalid_character() {
    struct FakeParser {
        scratch: RefCell<String>,
        pos: Position,
        char: char,
        eof: bool,
    }

    impl FakeParser {
        fn new() -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                pos: Position { offset: 0, line: 1, column: 1 },
                char: 'G', // Invalid hex character
                eof: false,
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulating successful bump
            self.pos.offset += 1;
            true
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind,
                pattern: String::new(),
                span,
            }
        }

        fn char(&self) -> char {
            self.char
        }

        fn bump(&mut self) {
            self.char = '}';
            self.pos.offset += 1;
        }
    }

    let parser = FakeParser::new();
    let parser_i = ParserI {
        parser,
        pattern: "{G}".as_ref(),
    };

    let result = parser_i.parse_hex_brace(ast::HexLiteralKind::X);
    
    assert!(result.is_err());
    if let Err(Error { kind, .. }) = result {
        assert_eq!(kind, ErrorKind::EscapeHexInvalidDigit);
    }
}

#[test]
#[should_panic]
fn test_parse_hex_brace_unexpected_eof() {
    struct FakeParser {
        scratch: RefCell<String>,
        pos: Position,
        char: char,
        eof: bool,
    }

    impl FakeParser {
        fn new() -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                pos: Position { offset: 0, line: 1, column: 1 },
                char: '1',
                eof: true, // Here we simulate EOF
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulating successful bump
            self.pos.offset += 1;
            true
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind,
                pattern: String::new(),
                span,
            }
        }

        fn char(&self) -> char {
            self.char
        }
        
        fn bump(&mut self) {
            // Not called in the test
        }
    }

    let parser = FakeParser::new();
    let parser_i = ParserI {
        parser,
        pattern: "{1".as_ref(), // Missing end brace
    };

    parser_i.parse_hex_brace(ast::HexLiteralKind::X);
}

