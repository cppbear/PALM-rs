// Answer 0

#[test]
fn test_parse_hex_x() {
    struct MockParser {
        char: char,
        pos: Position,
    }
    
    impl MockParser {
        fn bump_and_bump_space(&self) -> bool {
            true
        }
        
        fn char(&self) -> char {
            self.char
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span { start: self.pos, end: self.pos }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            let hex_value = "FF"; // Example valid hex for \xFF
            let c = char::from_u32(u32::from_str_radix(hex_value, 16).unwrap()).unwrap();
            Ok(ast::Literal {
                span: Span { start: 0, end: 2 },
                kind: ast::LiteralKind::HexFixed(kind),
                c,
            })
        }
    }

    let mock_parser = MockParser { char: 'x', pos: 0 };
    let result = mock_parser.parse_hex();
    assert!(result.is_ok());
}

#[test]
fn test_parse_hex_u() {
    struct MockParser {
        char: char,
        pos: Position,
    }
    
    impl MockParser {
        fn bump_and_bump_space(&self) -> bool {
            true
        }
        
        fn char(&self) -> char {
            self.char
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span { start: self.pos, end: self.pos }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            let hex_value = "1234"; // Example valid hex for \u1234
            let c = char::from_u32(u32::from_str_radix(hex_value, 16).unwrap()).unwrap();
            Ok(ast::Literal {
                span: Span { start: 0, end: 4 },
                kind: ast::LiteralKind::HexFixed(kind),
                c,
            })
        }
    }

    let mock_parser = MockParser { char: 'u', pos: 0 };
    let result = mock_parser.parse_hex();
    assert!(result.is_ok());
}

#[test]
fn test_parse_hex_brace() {
    struct MockParser {
        char: char,
        pos: Position,
    }
    
    impl MockParser {
        fn bump_and_bump_space(&self) -> bool {
            true
        }

        fn char(&self) -> char {
            self.char
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span { start: self.pos, end: self.pos }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn parse_hex_brace(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            let hex_value = "ABCD"; // Example valid hex for \u{ABCD}
            let c = char::from_u32(u32::from_str_radix(hex_value, 16).unwrap()).unwrap();
            Ok(ast::Literal {
                span: Span { start: 0, end: 4 },
                kind: ast::LiteralKind::HexBrace(kind),
                c,
            })
        }
    }

    let mock_parser = MockParser { char: 'u', pos: 0 };
    let result = mock_parser.parse_hex();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_hex_invalid_character() {
    struct MockParser {
        char: char,
        pos: Position,
    }

    impl MockParser {
        fn bump_and_bump_space(&self) -> bool {
            true
        }

        fn char(&self) -> char {
            self.char
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            Span { start: self.pos, end: self.pos }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::new(), span }
        }

        fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            // This will trigger an invalid character panic
            Err(self.error(Span::new(0, 1), ast::ErrorKind::EscapeHexInvalidDigit))
        }
    }

    let mock_parser = MockParser { char: 'x', pos: 0 };
    let _ = mock_parser.parse_hex();
}

