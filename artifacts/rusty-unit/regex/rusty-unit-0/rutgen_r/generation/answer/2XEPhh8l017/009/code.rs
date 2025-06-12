// Answer 0

#[test]
fn test_parse_hex_digits_invalid_length() {
    struct ParserMock {
        pos: usize,
        input: Vec<char>,
    }

    impl ParserMock {
        fn new(input: &str) -> Self {
            Self {
                pos: 0,
                input: input.chars().collect(),
            }
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> Result<ast::Literal> {
            Err(kind)
        }

        fn scratch(&self) -> &mut Vec<char> {
            &mut vec![]
        }
    }

    struct HexLiteralKind {
        digits: usize,
    }

    impl HexLiteralKind {
        fn digits(&self) -> usize {
            self.digits
        }
    }

    impl ParserMock {
        fn parse_hex_digits(&mut self, kind: HexLiteralKind) -> Result<ast::Literal> {
            let mut scratch = self.scratch();
            scratch.clear();

            let start = self.pos();
            for i in 0..kind.digits() {
                if i > 0 && self.pos() >= self.input.len() {
                    return self.error(ast::Span::new(start, start), ast::ErrorKind::EscapeUnexpectedEof);
                }
                if !is_hex(self.char()) {
                    return self.error(ast::Span::new(start, start + 1), ast::ErrorKind::EscapeHexInvalidDigit);
                }
                scratch.push(self.char());
                self.bump();
            }
            self.bump(); // Final bump past the literal
            let end = self.pos();
            let hex = scratch.iter().collect::<String>();
            match u32::from_str_radix(&hex, 16).ok().and_then(std::char::from_u32) {
                None => self.error(ast::Span::new(start, end), ast::ErrorKind::EscapeHexInvalid),
                Some(c) => Ok(ast::Literal {
                    span: ast::Span::new(start, end),
                    kind: ast::LiteralKind::HexFixed(kind),
                    c: c,
                }),
            }
        }
    }

    // Test cases
    let mut parser = ParserMock::new("GHIJ");
    let hex_kind = HexLiteralKind { digits: 4 }; // Expecting 4 hex digits, but input is invalid
    let result = parser.parse_hex_digits(hex_kind);
    assert!(result.is_err());

    let mut parser_invalid = ParserMock::new("ZZZZZZZZ");
    let hex_kind_invalid = HexLiteralKind { digits: 8 }; // Input of 8 invalid hex digits
    let result_invalid = parser_invalid.parse_hex_digits(hex_kind_invalid);
    assert!(result_invalid.is_err());
}

