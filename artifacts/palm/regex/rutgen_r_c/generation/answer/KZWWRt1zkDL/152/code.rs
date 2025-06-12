// Answer 0

#[test]
fn test_parse_escape_with_valid_hex() {
    struct FakeParser {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl FakeParser {
        fn new(pattern: &str) -> Self {
            FakeParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                octal: true,
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn parse_hex(&self) -> Result<ast::Literal> {
            Ok(ast::Literal {
                span: Span::new(self.pos.clone(), self.pos.clone()),
                kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
                c: 'a',
            })
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos.clone(), self.pos.clone()),
            }
        }
    }

    let parser = FakeParser::new(r"\x");
    let result = parser.parse_escape();
    assert!(result.is_ok());
    match result {
        Ok(Primitive::Literal(lit)) => {
            assert_eq!(lit.kind, ast::LiteralKind::HexFixed(ast::HexLiteralKind::X));
            assert_eq!(lit.c, 'a');  // Assuming 'a' was the parsed character
        },
        _ => panic!("Expected a successful parse with a Primitive::Literal"),
    }
}

#[test]
fn test_parse_escape_with_supported_hex_u() {
    struct FakeParser {
        pattern: String,
        pos: Position,
        octal: bool,
    }

    impl FakeParser {
        fn new(pattern: &str) -> Self {
            FakeParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                octal: true,
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn parse_hex(&self) -> Result<ast::Literal> {
            Ok(ast::Literal {
                span: Span::new(self.pos.clone(), self.pos.clone()),
                kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::U),
                c: 'A',
            })
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: self.pattern.clone(),
                span: Span::new(self.pos.clone(), self.pos.clone()),
            }
        }
    }

    let parser = FakeParser::new(r"\u");
    let result = parser.parse_escape();
    assert!(result.is_ok());
    match result {
        Ok(Primitive::Literal(lit)) => {
            assert_eq!(lit.kind, ast::LiteralKind::HexFixed(ast::HexLiteralKind::U));
            assert_eq!(lit.c, 'A');  // Assuming 'A' was the parsed character
        },
        _ => panic!("Expected a successful parse with a Primitive::Literal"),
    }
}

