// Answer 0

#[test]
fn test_parse_hex_brace_valid() {
    // Create a mock parser and input for testing
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self { input, pos: 0 }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos..].chars().next().unwrap().is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1;
                return true;
            }
            false
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn span_char(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }

        fn error(&self, span: (usize, usize), kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }
        
        fn scratch(&self) -> &mut Vec<char> {
            &mut vec![]
        }
        
        fn pos(&self) -> usize {
            self.pos
        }
    }

    let mut parser = MockParser::new("{1f}");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_ok());
    let literal = result.unwrap();
    assert_eq!(literal.c, 'ǿ');
}

#[test]
#[should_panic]
fn test_parse_hex_brace_empty() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self { input, pos: 0 }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos..].chars().next().unwrap().is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1;
                return true;
            }
            false
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn span_char(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }
        
        fn error(&self, span: (usize, usize), kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }

        fn pos(&self) -> usize {
            self.pos
        }
    }

    let mut parser = MockParser::new("{}");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    // Expecting this to panic as the hex is empty
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_brace_invalid_character() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self { input, pos: 0 }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos..].chars().next().unwrap().is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1;
                return true;
            }
            false
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn span_char(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }
        
        fn error(&self, span: (usize, usize), kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }

        fn pos(&self) -> usize {
            self.pos
        }
    }

    let mut parser = MockParser::new("{1g}");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_brace_unexpected_eof() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self { input, pos: 0 }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos..].chars().next().unwrap().is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1;
                return true;
            }
            false
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn span_char(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }

        fn error(&self, span: (usize, usize), kind: ast::ErrorKind) -> ast::Error {
            ast::Error { span, kind }
        }

        fn pos(&self) -> usize {
            self.pos
        }
    }

    let mut parser = MockParser::new("{123");
    let result = parser.parse_hex_brace(ast::HexLiteralKind::SomeKind);
    assert!(result.is_err());
}

