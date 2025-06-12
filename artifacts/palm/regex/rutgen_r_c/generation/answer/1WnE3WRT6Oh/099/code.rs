// Answer 0

#[test]
fn test_parse_decimal_empty_string() {
    struct TestParser {
        input: String,
        pos: Position,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos.offset..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.pos.offset += 1;
            }
        }

        fn bump_and_bump_space(&mut self) {
            self.bump();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<u32> {
            Err(ast::Error { kind, pattern: self.input.clone(), span })
        }
    }

    let mut parser = TestParser::new("     "); // Whitespace only.
    let result = parser.parse_decimal();
    assert_eq!(result, Err(parser.error(Span::new(parser.pos(), parser.pos()), ast::ErrorKind::DecimalEmpty)));
}

#[test]
fn test_parse_decimal_invalid_characters() {
    struct TestParser {
        input: String,
        pos: Position,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos.offset..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.pos.offset += 1;
            }
        }

        fn bump_and_bump_space(&mut self) {
            self.bump();
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<u32> {
            Err(ast::Error { kind, pattern: self.input.clone(), span })
        }
    }

    let mut parser = TestParser::new("12a34"); // Invalid character 'a'.
    parser.bump(); // skip '1'
    parser.bump(); // skip '2'
    let result = parser.parse_decimal();
    assert_eq!(result, Err(parser.error(Span::new(Position { offset: 0, line: 1, column: 1 }, parser.pos()), ast::ErrorKind::DecimalInvalid)));
}

