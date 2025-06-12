// Answer 0

#[test]
fn test_parse_hex_brace_invalid_digit() {
    struct TestParser {
        pos: usize,
        input: Vec<char>,
        error: Option<ast::ErrorKind>,
        scratch: Vec<char>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                pos: 0,
                input: input.chars().collect(),
                error: None,
                scratch: Vec::new(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulate advancing the position, assuming space handling as well.
            self.pos += 1;
            self.pos < self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            self.error = Some(kind);
            ast::Error { span, kind }
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &Self {
            self
        }

        fn parse_hex_brace(&mut self, kind: ast::HexLiteralKind) -> Result<ast::Literal> {
            // The content of the tested method would go here.
            // For brevity, we will assume it's an inline copy of the original method here.
            let mut scratch = self.scratch.clone();
            scratch.clear();

            let brace_pos = self.pos();
            let start = self.span_char().end;
            while self.bump_and_bump_space() && self.char() != '}' {
                if !self.char().is_digit(16) { // Simulating is_hex check
                    return Err(self.error(
                        self.span_char(),
                        ast::ErrorKind::EscapeHexInvalidDigit,
                    ));
                }
                scratch.push(self.char());
            }
            if self.is_eof() {
                return Err(self.error(
                    Span::new(brace_pos, self.pos()),
                    ast::ErrorKind::EscapeUnexpectedEof,
                ));
            }
            let end = self.pos();
            let hex = scratch.as_str();
            assert_eq!(self.char(), '}');
            self.pos += 1; // Simulating bump

            if hex.is_empty() {
                return Err(self.error(
                    Span::new(brace_pos, self.pos()),
                    ast::ErrorKind::EscapeHexEmpty,
                ));
            }
            match u32::from_str_radix(hex, 16).ok().and_then(char::from_u32) {
                None => Err(self.error(
                    Span::new(start, end),
                    ast::ErrorKind::EscapeHexInvalid,
                )),
                Some(c) => Ok(ast::Literal {
                    span: Span::new(start, self.pos()),
                    kind: ast::LiteralKind::HexBrace(kind),
                    c: c,
                }),
            }
        }
    }

    let mut parser = TestParser::new("{G}"); // 'G' is not a hex digit.
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind); // Use an appropriate variant.

    assert_eq!(result.is_err(), true);
    assert_eq!(parser.error, Some(ast::ErrorKind::EscapeHexInvalidDigit));
}

