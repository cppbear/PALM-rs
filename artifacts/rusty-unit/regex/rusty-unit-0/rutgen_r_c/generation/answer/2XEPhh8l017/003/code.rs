// Answer 0

#[test]
fn test_parse_hex_digits_invalid_hex() {
    struct TestParser {
        current_char: char,
        position: Position,
    }

    impl TestParser {
        fn bump_and_bump_space(&self) -> bool {
            true // Simulating that bump and space operations are successful
        }
        
        fn char(&self) -> char {
            self.current_char
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos()) // Placeholder for span logic
        }

        fn error(&self, _span: Span, kind: ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span: self.span_char() }
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let parser = TestParser {
        current_char: 'G', // Invalid hex character
        position: Position { offset: 0, line: 1, column: 1 },
    };

    let result = parser.parse_hex_digits(ast::HexLiteralKind::X);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::EscapeHexInvalidDigit);
    }
}

