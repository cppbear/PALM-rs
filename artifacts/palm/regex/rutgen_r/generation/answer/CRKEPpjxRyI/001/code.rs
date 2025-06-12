// Answer 0

#[test]
fn test_parse_set_class_range_invalid_escape() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
            }
        }

        fn parse_set_class_item(&mut self) -> Result<ast::ClassSetItem> {
            Err(ast::ErrorKind::InvalidEscape.into())
        }

        fn bump_space(&mut self) {
            self.position += 1;
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn peek_space(&self) -> Option<char> {
            self.input.get(self.position + 1).cloned()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if !self.is_eof() {
                self.position += 1; // bump current character
            }
            self.position < self.input.len() && {
                self.bump_space();
                true
            }
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error::new("Unclosed character class")
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new(&format!("Error: {:?} at {:?}", kind, span))
        }
    }

    let mut parser = MockParser::new("[a-z]");
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_range_unclosed_class() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
            }
        }

        fn parse_set_class_item(&mut self) -> Result<ast::ClassSetItem> {
            Err(ast::ErrorKind::ClassRangeInvalid.into())
        }

        fn bump_space(&mut self) {
            self.position += 1;
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn peek_space(&self) -> Option<char> {
            self.input.get(self.position + 1).cloned()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if !self.is_eof() {
                self.position += 1; // bump current character
            }
            self.position < self.input.len() && {
                self.bump_space();
                true
            }
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error::new("Unclosed character class")
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new(&format!("Error: {:?} at {:?}", kind, span))
        }
    }

    let mut parser = MockParser::new("[a-");
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
            }
        }

        fn parse_set_class_item(&mut self) -> Result<ast::ClassSetItem> {
            Ok(ast::ClassSetItem::Literal('a'))
        }

        fn bump_space(&mut self) {
            self.position += 1;
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn peek_space(&self) -> Option<char> {
            self.input.get(self.position + 1).cloned()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if !self.is_eof() {
                self.position += 1; // bump current character
            }
            self.position < self.input.len() && {
                self.bump_space();
                true
            }
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error::new("Unclosed character class")
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new(&format!("Error: {:?} at {:?}", kind, span))
        }
    }

    let mut parser = MockParser::new("[a-z");
    parser.position = 3; // Simulate position after `a-`
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
}

