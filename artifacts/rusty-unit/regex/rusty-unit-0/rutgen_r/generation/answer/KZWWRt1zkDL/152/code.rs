// Answer 0

#[test]
fn test_parse_escape_valid_hex() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &Self {
            self
        }

        fn parse_hex(&self) -> Result<ast::Literal, ast::ErrorKind> {
            // Mock implementation - assume valid hex parsing
            Ok(ast::Literal {
                span: Span::new(0, 2),
                kind: ast::LiteralKind::Punctuation,
                c: 'x',
            })
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = TestParser {
        input: vec!['\\', 'x', '3', '1'], // Input with valid hex escape
        pos: 0,
        octal: false,
    };

    let result = parser.parse_escape();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_escape_invalid_escape_sequence() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &Self {
            self
        }

        fn parse_hex(&self) -> Result<ast::Literal, ast::ErrorKind> {
            // Mock implementation - assume error in hex parsing
            Err(ast::ErrorKind::EscapeUnrecognized)
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = TestParser {
        input: vec!['\\', 'x', 'G'], // Invalid hex escape sequence
        pos: 0,
        octal: false,
    };

    let _ = parser.parse_escape(); // This should trigger a panic.
}

