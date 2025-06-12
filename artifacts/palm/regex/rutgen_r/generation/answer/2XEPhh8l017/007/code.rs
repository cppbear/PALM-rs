// Answer 0

#[test]
fn test_parse_hex_digits_invalid_first_digit() {
    struct TestParser {
        pos: usize,
        input: Vec<char>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                pos: 0,
                input: input.chars().collect(),
            }
        }

        fn bump(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            let res = self.pos < self.input.len();
            self.bump();
            res
        }

        fn error(&self) -> ast::ErrorKind {
            ast::ErrorKind::EscapeHexInvalidDigit
        }

        fn parser(&self) -> &Self {
            self
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let kind = ast::HexLiteralKind::new(2);
    let mut parser = TestParser::new("g"); // 'g' is not a hex digit

    let result = parser.parse_hex_digits(kind);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind, ast::ErrorKind::EscapeHexInvalidDigit);
}

#[test]
fn test_parse_hex_digits_empty_input() {
    struct TestParser {
        pos: usize,
        input: Vec<char>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                pos: 0,
                input: input.chars().collect(),
            }
        }

        fn bump(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            let res = self.pos < self.input.len();
            self.bump();
            res
        }

        fn error(&self) -> ast::ErrorKind {
            ast::ErrorKind::EscapeHexInvalidDigit
        }

        fn parser(&self) -> &Self {
            self
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let kind = ast::HexLiteralKind::new(2);
    let mut parser = TestParser::new(""); // empty input

    let result = parser.parse_hex_digits(kind);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind, ast::ErrorKind::EscapeUnexpectedEof);
}

