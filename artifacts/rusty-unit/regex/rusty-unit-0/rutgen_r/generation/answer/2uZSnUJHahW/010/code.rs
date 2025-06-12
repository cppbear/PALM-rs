// Answer 0

#[test]
fn test_parse_hex_brace_invalid_hex() {
    struct TestParser {
        pos: usize,
        input: String,
        error: Option<ast::ErrorKind>,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn scratch(&self) -> &mut Vec<char> {
            &mut vec![]
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap()
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::ErrorKind {
            kind
        }
    }

    let kind = ast::HexLiteralKind::Hex; // Assuming HexLiteralKind::Hex exists
    let mut parser = TestParser { pos: 0, input: "{g}".to_string(), error: None };
    let result = parser.parse_hex_brace(kind);

    assert_eq!(result, Err(parser.error(parser.span_char(), ast::ErrorKind::EscapeHexInvalid)));
}

#[test]
fn test_parse_hex_brace_empty_hex() {
    struct TestParser {
        pos: usize,
        input: String,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap()
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::ErrorKind {
            kind
        }
    }

    let kind = ast::HexLiteralKind::Hex; // Assuming HexLiteralKind::Hex exists
    let mut parser = TestParser { pos: 0, input: "{}".to_string() , };
    let result = parser.parse_hex_brace(kind);

    assert_eq!(result, Err(parser.error(parser.span_char(), ast::ErrorKind::EscapeHexEmpty)));
}

#[test]
fn test_parse_hex_brace_invalid_hex_string() {
    struct TestParser {
        pos: usize,
        input: String,
        error: Option<ast::ErrorKind>,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap()
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::ErrorKind {
            kind
        }
    }

    let kind = ast::HexLiteralKind::Hex; // Assuming HexLiteralKind::Hex exists
    let mut parser = TestParser { pos: 0, input: "{xyz}".to_string(), error: None };
    let result = parser.parse_hex_brace(kind);

    assert_eq!(result, Err(parser.error(parser.span_char(), ast::ErrorKind::EscapeHexInvalid)));
}

