// Answer 0

fn test_parse_hex_unexpected_eof_x() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                    self.pos += 1;
                }
                return true;
            }
            false
        }

        fn span(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> Result<ast::Literal> {
            Err(ast::ErrorKind::EscapeUnexpectedEof.into())
        }

        fn parse_hex(&mut self) -> Result<ast::Literal> {
            assert!(self.char() == 'x');

            let hex_kind = ast::HexLiteralKind::X;
            if !self.bump_and_bump_space() {
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }
            if self.char() == '{' {
                // Placeholder for parse_hex_brace
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            } else {
                // Placeholder for parse_hex_digits
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }
        }
    }

    let mut parser = TestParser { input: "\\x", pos: 0 };
    let result = parser.parse_hex();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, ast::ErrorKind::EscapeUnexpectedEof.into());
    }
}

fn test_parse_hex_unexpected_eof_u() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                    self.pos += 1;
                }
                return true;
            }
            false
        }

        fn span(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> Result<ast::Literal> {
            Err(ast::ErrorKind::EscapeUnexpectedEof.into())
        }

        fn parse_hex(&mut self) -> Result<ast::Literal> {
            assert!(self.char() == 'u');

            let hex_kind = ast::HexLiteralKind::UnicodeShort;
            if !self.bump_and_bump_space() {
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }
            if self.char() == '{' {
                // Placeholder for parse_hex_brace
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            } else {
                // Placeholder for parse_hex_digits
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }
        }
    }

    let mut parser = TestParser { input: "\\u", pos: 0 };
    let result = parser.parse_hex();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, ast::ErrorKind::EscapeUnexpectedEof.into());
    }
}

