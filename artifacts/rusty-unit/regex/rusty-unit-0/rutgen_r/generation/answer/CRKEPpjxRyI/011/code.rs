// Answer 0

#[test]
fn test_parse_set_class_range_invalid_escape() {
    struct TestParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(chars: Vec<char>) -> Self {
            TestParser { chars, pos: 0 }
        }

        fn parse_set_class_item(&mut self) -> Result<ast::ClassSetItem> {
            // Successful item parsing simulation
            let c = self.chars.get(self.pos).ok_or_else(|| self.unclosed_class_error())?;
            self.pos += 1;
            Ok(ast::ClassSetItem::Literal(*c))
        }

        fn bump_space(&mut self) {
            // Simulate bumping past space
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn peek_space(&self) -> Option<char> {
            if self.pos + 1 < self.chars.len() {
                Some(self.chars[self.pos + 1])
            } else {
                None
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            // Simulate a failed bump
            false
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error::new("Unclosed character class")
        }
    }

    let mut parser = TestParser::new(vec!['a', '-', 'b', ']']);
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message, "Unclosed character class");
}

#[test]
#[should_panic]
fn test_parse_set_class_range_eof() {
    struct TestParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(chars: Vec<char>) -> Self {
            TestParser { chars, pos: 0 }
        }

        fn parse_set_class_item(&mut self) -> Result<ast::ClassSetItem> {
            // Simulate successful item parsing
            if self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                self.pos += 1;
                Ok(ast::ClassSetItem::Literal(c))
            } else {
                Err(self.unclosed_class_error())
            }
        }

        fn bump_space(&mut self) {
            // Simulate bumping past space
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn peek_space(&self) -> Option<char> {
            None
        }

        fn bump_and_bump_space(&mut self) -> bool {
            false
        }

        fn unclosed_class_error(&self) -> ast::Error {
            panic!("Unclosed character class");
        }
    }

    let mut parser = TestParser::new(vec!['a']);
    parser.parse_set_class_range(); // Should panic
}

