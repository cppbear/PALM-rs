// Answer 0

#[test]
fn test_parse_unicode_class_with_single_letter() {
    struct SimpleParser {
        pos: Position,
        input: String,
        index: usize,
    }

    impl SimpleParser {
        fn new(input: &str) -> Self {
            SimpleParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                input: input.to_string(),
                index: 0,
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.index).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Bump index to the next character, skipping spaces
            while self.index < self.input.len() && self.input[self.index..].starts_with(char::is_whitespace) {
                self.index += 1;
            }
            if self.index < self.input.len() {
                self.index += 1;
                return true;
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len() 
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<ast::ClassUnicode> {
            Err(ast::Error { 
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: self.input.clone(),
                span: self.span(),
            })
        }
    }

    let mut parser = SimpleParser::new("p");
    let result = parser.parse_unicode_class();
    assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
}

#[test]
#[should_panic(expected = "EscapeUnexpectedEof")]
fn test_parse_unicode_class_eof() {
    struct SimpleParser {
        pos: Position,
        input: String,
        index: usize,
    }

    impl SimpleParser {
        fn new(input: &str) -> Self {
            SimpleParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                input: input.to_string(),
                index: 0,
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.index).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulate failure to bump properly
            if self.index < self.input.len() {
                self.index += 1;
                return false;
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len() 
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<ast::ClassUnicode> {
            Err(ast::Error { 
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: self.input.clone(),
                span: self.span(),
            })
        }
    }

    let mut parser = SimpleParser::new("p");
    // This call should panic due to EOF.
    let _ = parser.parse_unicode_class();
}

#[test]
fn test_parse_unicode_class_with_negation() {
    struct SimpleParser {
        pos: Position,
        input: String,
        index: usize,
    }

    impl SimpleParser {
        fn new(input: &str) -> Self {
            SimpleParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                input: input.to_string(),
                index: 0,
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.index).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.index < self.input.len() {
                self.index += 1;
                return true;
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len() 
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<ast::ClassUnicode> {
            Err(ast::Error { 
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: self.input.clone(),
                span: self.span(),
            })
        }
    }

    let mut parser = SimpleParser::new("P{Greek}");
    parser.bump_and_bump_space();
    let result = parser.parse_unicode_class();
    assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
}

#[test]
#[should_panic(expected = "EscapeUnexpectedEof")]
fn test_parse_unicode_class_with_no_closing_brace() {
    struct SimpleParser {
        pos: Position,
        input: String,
        index: usize,
    }

    impl SimpleParser {
        fn new(input: &str) -> Self {
            SimpleParser {
                pos: Position { offset: 0, line: 1, column: 1 },
                input: input.to_string(),
                index: 0,
            }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.index).unwrap_or('\0')
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.index < self.input.len() {
                return false; // Forces the condition to fail
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len() 
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<ast::ClassUnicode> {
            Err(ast::Error { 
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: self.input.clone(),
                span: self.span(),
            })
        }
    }

    let mut parser = SimpleParser::new("p{Greek");
    parser.bump_and_bump_space();
    // This call should panic.
    let _ = parser.parse_unicode_class();
}

