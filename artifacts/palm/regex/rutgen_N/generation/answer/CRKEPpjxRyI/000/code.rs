// Answer 0

#[test]
fn test_parse_set_class_range_valid_literal_range() {
    struct TestParser {
        input: String,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                pos: 0,
            }
        }

        fn parse_set_class_item(&self) -> Result<ast::ClassSetItem> {
            // Simulated parse_set_class_item for testing
            if self.input.chars().nth(self.pos).unwrap() == 'a' {
                Ok(ast::ClassSetItem::Literal('a'))
            } else {
                Err("Invalid item".into())
            }
        }

        fn bump_space(&mut self) {
            // Simulate bumping over space
            self.pos += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap()
        }

        fn peek_space(&self) -> Option<char> {
            self.input.chars().nth(self.pos + 1)
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1; // Bump past '-'
            self.bump_space();
            true
        }

        fn unclosed_class_error(&self) -> &'static str {
            "Unclosed class error"
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> &'static str {
            "Range invalid error"
        }
    }

    let mut parser = TestParser::new("a-b");
    let result = parser.parse_set_class_range();
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Unclosed class error")]
fn test_parse_set_class_range_unclosed_class() {
    struct TestParser {
        input: String,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                pos: 0,
            }
        }

        fn parse_set_class_item(&self) -> Result<ast::ClassSetItem> {
            // Same logic as before
            Ok(ast::ClassSetItem::Literal('a'))
        }

        fn bump_space(&mut self) {
            self.pos += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap()
        }

        fn peek_space(&self) -> Option<char> {
            self.input.chars().nth(self.pos + 1)
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1; // Simulate bumping past '-'
            false // Simulate unclosed class scenario
        }

        fn unclosed_class_error(&self) -> &'static str {
            "Unclosed class error"
        }
    }

    let parser = TestParser::new("a-");
    parser.parse_set_class_range().unwrap();
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    struct TestParser {
        input: String,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                pos: 0,
            }
        }

        fn parse_set_class_item(&self) -> Result<ast::ClassSetItem> {
            // Simulated parsing logic for items
            Ok(ast::ClassSetItem::Literal('z'))
        }

        fn bump_space(&mut self) {
            self.pos += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap()
        }

        fn peek_space(&self) -> Option<char> {
            self.input.chars().nth(self.pos + 1)
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1; // Simulate bumping past '-'
            self.bump_space();
            true
        }

        fn unclosed_class_error(&self) -> &'static str {
            "Unclosed class error"
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> &'static str {
            "Range invalid error"
        }
    }

    let mut parser = TestParser::new("z-a"); // Invalid range
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
}

