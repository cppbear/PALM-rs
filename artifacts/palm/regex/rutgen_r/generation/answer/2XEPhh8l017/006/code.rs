// Answer 0

#[test]
fn test_parse_hex_digits_2_digit_invalid() {
    struct TestParser {
        input: String,
        pos: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn scratch(&self) -> Vec<char> {
            Vec::new()
        }
        
        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1;
            true
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn error(&self, _span: ast::Span, _kind: ast::ErrorKind) -> Result<ast::Literal, ast::ErrorKind> {
            Err(ast::ErrorKind::EscapeHexInvalid)
        }
    }

    let parser = TestParser { input: String::from("\\xZZ"), pos: 0 };
    let result = parser.parse_hex_digits(ast::HexLiteralKind::Hex2);
    assert_eq!(result, Err(ast::ErrorKind::EscapeHexInvalid));
}

#[test]
fn test_parse_hex_digits_4_digit_invalid() {
    struct TestParser {
        input: String,
        pos: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn scratch(&self) -> Vec<char> {
            Vec::new()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1;
            true
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn error(&self, _span: ast::Span, _kind: ast::ErrorKind) -> Result<ast::Literal, ast::ErrorKind> {
            Err(ast::ErrorKind::EscapeHexInvalid)
        }
    }

    let parser = TestParser { input: String::from("\\uZZZZ"), pos: 0 };
    let result = parser.parse_hex_digits(ast::HexLiteralKind::Hex4);
    assert_eq!(result, Err(ast::ErrorKind::EscapeHexInvalid));
}

#[test]
fn test_parse_hex_digits_8_digit_invalid() {
    struct TestParser {
        input: String,
        pos: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn scratch(&self) -> Vec<char> {
            Vec::new()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1;
            true
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn error(&self, _span: ast::Span, _kind: ast::ErrorKind) -> Result<ast::Literal, ast::ErrorKind> {
            Err(ast::ErrorKind::EscapeHexInvalid)
        }
    }

    let parser = TestParser { input: String::from("\\UNNNNNZZZ"), pos: 0 };
    let result = parser.parse_hex_digits(ast::HexLiteralKind::Hex8);
    assert_eq!(result, Err(ast::ErrorKind::EscapeHexInvalid));
}

