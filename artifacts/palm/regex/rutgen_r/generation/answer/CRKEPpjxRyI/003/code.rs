// Answer 0

#[test]
fn test_parse_set_class_range_valid_literal() {
    struct TestParser {
        input: Vec<char>,
        index: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                index: 0,
            }
        }

        fn parse_set_class_item(&mut self) -> Result<ast::ClassSetItem> {
            let c = self.next_char()?;
            Ok(ast::ClassSetItem::Literal(c))
        }

        fn bump_space(&mut self) {
            // Implementation that bumps the index past whitespace
            while self.index < self.input.len() && self.char().is_whitespace() {
                self.index += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.index]
        }

        fn peek_space(&self) -> Option<char> {
            if self.index + 1 < self.input.len() {
                Some(self.input[self.index + 1])
            } else {
                None
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1; // bump past the '-'
            self.bump_space();
            true
        }

        fn next_char(&mut self) -> Result<char> {
            if self.is_eof() {
                Err(Error::new("EOF"))
            } else {
                let c = self.char();
                self.index += 1;
                Ok(c)
            }
        }

        fn unclosed_class_error(&self) -> Error {
            Error::new("Unclosed character class")
        }
    }

    let mut parser = TestParser::new("a-c");
    let result = parser.parse_set_class_range();
    assert!(result.is_ok());
    if let Ok(item) = result {
        // Add assertions to check that the item is as expected.
    }
}

#[test]
fn test_parse_set_class_range_valid_range() {
    struct TestParser {
        input: Vec<char>,
        index: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                index: 0,
            }
        }

        fn parse_set_class_item(&mut self) -> Result<ast::ClassSetItem> {
            let c = self.next_char()?;
            Ok(ast::ClassSetItem::Literal(c))
        }

        fn bump_space(&mut self) {
            while self.index < self.input.len() && self.char().is_whitespace() {
                self.index += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.index]
        }

        fn peek_space(&self) -> Option<char> {
            if self.index + 1 < self.input.len() {
                Some(self.input[self.index + 1])
            } else {
                None
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1; // bump past the '-'
            self.bump_space();
            true
        }

        fn next_char(&mut self) -> Result<char> {
            if self.is_eof() {
                Err(Error::new("EOF"))
            } else {
                let c = self.char();
                self.index += 1;
                Ok(c)
            }
        }

        fn unclosed_class_error(&self) -> Error {
            Error::new("Unclosed character class")
        }
    }

    let mut parser = TestParser::new("a-b");
    let result = parser.parse_set_class_range();
    assert!(result.is_ok());
    if let Ok(item) = result {
        // Add assertions to check that the item is as expected.
    }
}

#[test]
fn test_parse_set_class_range_edge_case() {
    struct TestParser {
        input: Vec<char>,
        index: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                index: 0,
            }
        }

        fn parse_set_class_item(&mut self) -> Result<ast::ClassSetItem> {
            let c = self.next_char()?;
            Ok(ast::ClassSetItem::Literal(c))
        }

        fn bump_space(&mut self) {
            while self.index < self.input.len() && self.char().is_whitespace() {
                self.index += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.index]
        }

        fn peek_space(&self) -> Option<char> {
            if self.index + 1 < self.input.len() {
                Some(self.input[self.index + 1])
            } else {
                None
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.index += 1; // bump past the '-'
            self.bump_space();
            true
        }

        fn next_char(&mut self) -> Result<char> {
            if self.is_eof() {
                Err(Error::new("EOF"))
            } else {
                let c = self.char();
                self.index += 1;
                Ok(c)
            }
        }

        fn unclosed_class_error(&self) -> Error {
            Error::new("Unclosed character class")
        }
    }

    let mut parser = TestParser::new("x-y");
    let result = parser.parse_set_class_range();
    assert!(result.is_ok());
    if let Ok(item) = result {
        // Add assertions to check the range is valid and items are as expected.
    }
}

