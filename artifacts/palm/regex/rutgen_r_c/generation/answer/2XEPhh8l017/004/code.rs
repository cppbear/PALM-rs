// Answer 0

#[test]
fn test_parse_hex_digits_unexpected_eof() {
    struct TestParser {
        pos: Position,
        scratch: String,
    }

    impl TestParser {
        fn new() -> Self {
            TestParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                scratch: String::new(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            false // simulate the condition where this returns false
        }

        fn char(&self) -> char {
            'g' // simulate a non-hex character
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn span(&self) -> Span {
            Span::new(self.pos.clone(), self.pos.clone())
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Error {
            Error {
                kind,
                pattern: String::from(""),
                span,
            }
        }

        fn parse_hex_digits(&mut self, kind: HexLiteralKind) -> Result<ast::Literal> {
            let mut scratch = self.scratch.clone();
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
            self.pos.offset += 1; // simulate the bump
            let end = self.pos();
            // skip the hex checking for simplicity
            Ok(ast::Literal {
                span: Span::new(start, end),
                kind: LiteralKind::HexFixed(kind),
                c: 'a', // placeholder character
            })
        }
    }

    let mut parser = TestParser::new();
    let result = parser.parse_hex_digits(HexLiteralKind::X);

    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ast::ErrorKind::EscapeUnexpectedEof);
    }
}

