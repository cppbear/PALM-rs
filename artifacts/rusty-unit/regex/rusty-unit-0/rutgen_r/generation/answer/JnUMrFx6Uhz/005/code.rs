// Answer 0

#[test]
fn test_parse_flags_with_duplicate_flags() {
    struct TestParser {
        chars: Vec<char>,
        position: usize,
    }

    impl TestParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, position: 0 }
        }

        fn char(&self) -> char {
            self.chars[self.position]
        }

        fn bump(&mut self) -> bool {
            if self.position < self.chars.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> ast::Span {
            ast::Span { start: self.position as u32, end: (self.position + 1) as u32 }
        }

        fn span_char(&self) -> ast::Span {
            self.span()
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> std::result::Result<ast::Flags, ast::ErrorKind> {
            Err(kind)
        }

        fn parse_flag(&self) -> Result<ast::Flag, ()> {
            Err(())
        }

        fn pos(&self) -> usize {
            self.position
        }
    }

    let mut parser = TestParser::new(vec!['-', 'a', '-', 'b', ':']);
    let result = parser.parse_flags();
    assert!(result.is_err());
}

#[test]
fn test_parse_flags_with_dangling_negation() {
    struct TestParser {
        chars: Vec<char>,
        position: usize,
    }

    impl TestParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, position: 0 }
        }

        fn char(&self) -> char {
            self.chars[self.position]
        }

        fn bump(&mut self) -> bool {
            if self.position < self.chars.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> ast::Span {
            ast::Span { start: self.position as u32, end: (self.position + 1) as u32 }
        }

        fn span_char(&self) -> ast::Span {
            self.span()
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> std::result::Result<ast::Flags, ast::ErrorKind> {
            Err(kind)
        }

        fn parse_flag(&self) -> Result<ast::Flag, ()> {
            Err(())
        }

        fn pos(&self) -> usize {
            self.position
        }
    }

    let mut parser = TestParser::new(vec!['-', ':']);
    let result = parser.parse_flags();
    assert!(result.is_err());
}

#[test]
fn test_parse_flags_with_no_flags_found() {
    struct TestParser {
        chars: Vec<char>,
        position: usize,
    }

    impl TestParser {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, position: 0 }
        }

        fn char(&self) -> char {
            self.chars[self.position]
        }

        fn bump(&mut self) -> bool {
            if self.position < self.chars.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> ast::Span {
            ast::Span { start: self.position as u32, end: (self.position + 1) as u32 }
        }

        fn span_char(&self) -> ast::Span {
            self.span()
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> std::result::Result<ast::Flags, ast::ErrorKind> {
            Err(kind)
        }

        fn parse_flag(&self) -> Result<ast::Flag, ()> {
            Err(())
        }

        fn pos(&self) -> usize {
            self.position
        }
    }

    let mut parser = TestParser::new(vec![':']);
    let result = parser.parse_flags();
    assert!(result.is_err());
}

