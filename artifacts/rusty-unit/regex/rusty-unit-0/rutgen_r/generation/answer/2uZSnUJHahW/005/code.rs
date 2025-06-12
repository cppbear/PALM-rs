// Answer 0

#[test]
fn test_parse_hex_brace_valid() {
    struct MockParser {
        input: Vec<char>,
        index: usize,
        scratch: String,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                index: 0,
                scratch: String::new(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1;
            true
        }

        fn char(&self) -> char {
            if self.index < self.input.len() {
                self.input[self.index]
            } else {
                '\0'
            }
        }

        fn pos(&self) -> usize {
            self.index
        }

        fn parser(&self) -> &Self {
            self
        }

        fn span_char(&self) -> Span {
            Span::new(self.index, self.index + 1)
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<ast::Literal> {
            Err("Error".into())
        }
    }

    let mut parser = MockParser::new("{12}");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_ok());
    let literal = result.unwrap();
    assert_eq!(literal.c, '𐍈'); // Same as char::from_u32(0x12)
}

#[test]
#[should_panic]
fn test_parse_hex_brace_empty_hex() {
    struct MockParser {
        input: Vec<char>,
        index: usize,
        scratch: String,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                index: 0,
                scratch: String::new(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1;
            true
        }

        fn char(&self) -> char {
            if self.index < self.input.len() {
                self.input[self.index]
            } else {
                '\0'
            }
        }

        fn pos(&self) -> usize {
            self.index
        }

        fn parser(&self) -> &Self {
            self
        }

        fn span_char(&self) -> Span {
            Span::new(self.index, self.index + 1)
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<ast::Literal> {
            Err("Error".into())
        }
    }

    let mut parser = MockParser::new("{}");
    parser.parse_hex_brace(ast::HexLiteralKind::SomeKind).unwrap();
}

#[test]
fn test_parse_hex_brace_invalid_char() {
    struct MockParser {
        input: Vec<char>,
        index: usize,
        scratch: String,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                index: 0,
                scratch: String::new(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1;
            true
        }

        fn char(&self) -> char {
            if self.index < self.input.len() {
                self.input[self.index]
            } else {
                '\0'
            }
        }

        fn pos(&self) -> usize {
            self.index
        }

        fn parser(&self) -> &Self {
            self
        }

        fn span_char(&self) -> Span {
            Span::new(self.index, self.index + 1)
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<ast::Literal> {
            Err("Error".into())
        }
    }

    let mut parser = MockParser::new("{G}");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_brace_unexpected_eof() {
    struct MockParser {
        input: Vec<char>,
        index: usize,
        scratch: String,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                index: 0,
                scratch: String::new(),
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1;
            true
        }

        fn char(&self) -> char {
            if self.index < self.input.len() {
                self.input[self.index]
            } else {
                '\0'
            }
        }

        fn pos(&self) -> usize {
            self.index
        }

        fn parser(&self) -> &Self {
            self
        }

        fn span_char(&self) -> Span {
            Span::new(self.index, self.index + 1)
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<ast::Literal> {
            Err("Error".into())
        }
    }

    let mut parser = MockParser::new("{1"); // No closing brace
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_err());
}

