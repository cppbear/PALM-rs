// Answer 0

#[test]
fn test_parse_decimal_invalid() {
    struct TestParser {
        input: String,
        pos: Position,
    }
    
    impl TestParser {
        fn new(pattern: &str, offset: usize, line: usize, column: usize) -> Self {
            Self {
                input: pattern.to_string(),
                pos: Position { offset, line, column },
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

        fn scratch(&self) -> RefCell<String> {
            RefCell::new(String::new())
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<u32> {
            Err(ast::Error { kind: ast::ErrorKind::DecimalInvalid, pattern: self.input.clone(), span: Span::new(self.pos, self.pos) })
        }
    }

    let mut parser = TestParser::new("99999999999", 0, 1, 1);
    let result = parser.parse_decimal();

    // Trigger condition: No valid u32 conversion
    assert!(result.is_err());
}

#[test]
fn test_parse_decimal_empty() {
    struct TestParser {
        input: String,
        pos: Position,
    }
    
    impl TestParser {
        fn new(pattern: &str, offset: usize, line: usize, column: usize) -> Self {
            Self {
                input: pattern.to_string(),
                pos: Position { offset, line, column },
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

        fn scratch(&self) -> RefCell<String> {
            RefCell::new(String::new())
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<u32> {
            Err(ast::Error { kind: ast::ErrorKind::DecimalEmpty, pattern: self.input.clone(), span: Span::new(self.pos, self.pos) })
        }
    }

    let mut parser = TestParser::new("   ", 0, 1, 1);
    let result = parser.parse_decimal();

    // Trigger condition: No digits found
    assert!(result.is_err());
}

