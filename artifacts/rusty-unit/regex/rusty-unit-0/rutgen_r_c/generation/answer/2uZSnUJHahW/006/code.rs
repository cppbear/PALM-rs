// Answer 0

#[test]
fn test_parse_hex_brace_invalid_digit() {
    struct MockParser {
        scratch: RefCell<String>,
        position: Position,
        current_char: char,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                position: Position { offset: 0, line: 1, column: 1 },
                current_char: '1', // Start with a valid hex digit
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            self.current_char = 'g'; // Move to a non-hex character
            true
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn pos(&self) -> Position {
            self.position.clone()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Error {
            Error { kind, pattern: String::from("{g}"), span } // Example pattern with invalid hex
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos()) // Span covering the current position
        }

        fn is_eof(&self) -> bool {
            false // Not at the end of the input
        }
    }

    let parser = MockParser::new();
    let kind = ast::HexLiteralKind::X; // Using any HexLiteralKind

    let result = parser.parse_hex_brace(kind);

    match result {
        Err(err) => {
            assert_eq!(err.kind, ast::ErrorKind::EscapeHexInvalidDigit);
        },
        _ => panic!("Expected an error for invalid hex digit, but got a successful result."),
    }
}

#[test]
fn test_parse_hex_brace_empty_hex() {
    struct MockParser {
        scratch: RefCell<String>,
        position: Position,
        current_char: char,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                position: Position { offset: 0, line: 1, column: 1 },
                current_char: '}', // Position at closing brace immediately after
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            true // Keep returning true to adhere to the constraint
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn pos(&self) -> Position {
            self.position.clone()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Error {
            Error { kind, pattern: String::from("{}"), span }
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos())
        }

        fn is_eof(&self) -> bool {
            false // Not at EOF
        }
    }

    let parser = MockParser::new();
    let kind = ast::HexLiteralKind::X;

    let result = parser.parse_hex_brace(kind);

    match result {
        Err(err) => {
            assert_eq!(err.kind, ast::ErrorKind::EscapeHexEmpty);
        },
        _ => panic!("Expected an error for empty hex, but got a successful result."),
    }
}

#[test]
fn test_parse_hex_brace_unexpected_eof() {
    struct MockParser {
        scratch: RefCell<String>,
        position: Position,
        current_char: char,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                scratch: RefCell::new(String::new()),
                position: Position { offset: 0, line: 1, column: 1 },
                current_char: '1', // Start with a valid hex digit
            }
        }

        fn bump_and_bump_space(&self) -> bool {
            false // Simulate being at EOF directly after the opening brace
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn pos(&self) -> Position {
            self.position.clone()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Error {
            Error { kind, pattern: String::from("{1"), span } // Incomplete hex
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos()) // Returning current position for span
        }

        fn is_eof(&self) -> bool {
            true // Now at EOF
        }
    }

    let parser = MockParser::new();
    let kind = ast::HexLiteralKind::X;

    let result = parser.parse_hex_brace(kind);

    match result {
        Err(err) => {
            assert_eq!(err.kind, ast::ErrorKind::EscapeUnexpectedEof);
        },
        _ => panic!("Expected an error for unexpected EOF, but got a successful result."),
    }
}

