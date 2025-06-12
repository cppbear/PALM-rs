// Answer 0

#[test]
fn test_parse_set_class_range_valid() {
    struct MockParser {
        input: Vec<char>,
        index: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                index: 0,
            }
        }

        fn bump_space(&mut self) {
            while self.index < self.input.len() && self.input[self.index].is_whitespace() {
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
            self.index += 1;
            self.bump_space();
            !self.is_eof()
        }

        fn parse_set_class_item(&self) -> Result<ast::ClassSetItem> {
            // Assuming implementation returns Ok with a dummy ClassSetItem
            Ok(ast::ClassSetItem::Literal('a')) // Replace with actual implementation for real case
        }

        fn unclosed_class_error(&self) -> Error {
            Error::new("Unclosed class") // Replace with actual error handling
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Error {
            Error::new(format!("Error at {:?}: {:?}", span, kind)) // Replace with actual error handling
        }
    }

    impl MockParser {
        fn parse_set_class_range(&mut self) -> Result<ast::ClassSetItem> {
            let prim1 = self.parse_set_class_item()?;
            self.bump_space();
            if self.is_eof() {
                return Err(self.unclosed_class_error());
            }
            if self.char() != '-' || self.peek_space() == Some(']') {
                return prim1.into_class_set_item(self);
            }
            if !self.bump_and_bump_space() {
                return Err(self.unclosed_class_error());
            }
            let prim2 = self.parse_set_class_item()?;
            let range = ast::ClassSetRange {
                span: Span::new(prim1.span().start, prim2.span().end),
                start: prim1.into_class_literal(self)?,
                end: prim2.into_class_literal(self)?,
            };
            if !range.is_valid() {
                return Err(self.error(range.span, ast::ErrorKind::ClassRangeInvalid));
            }
            Ok(ast::ClassSetItem::Range(range))
        }
    }

    let mut parser = MockParser::new("a - b ]"); // Valid input for parsing a range
    let result = parser.parse_set_class_range();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_range_invalid_unclosed() {
    struct MockParser {
        input: Vec<char>,
        index: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                index: 0,
            }
        }

        fn bump_space(&mut self) {
            while self.index < self.input.len() && self.input[self.index].is_whitespace() {
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
            self.index += 1;
            self.bump_space();
            !self.is_eof()
        }

        fn parse_set_class_item(&self) -> Result<ast::ClassSetItem> {
            Ok(ast::ClassSetItem::Literal('a')) // Replace with actual implementation for real case
        }

        fn unclosed_class_error(&self) -> Error {
            Error::new("Unclosed class") // Replace with actual error handling
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Error {
            Error::new(format!("Error at {:?}: {:?}", span, kind)) // Replace with actual error handling
        }
    }

    impl MockParser {
        fn parse_set_class_range(&mut self) -> Result<ast::ClassSetItem> {
            let prim1 = self.parse_set_class_item()?;
            self.bump_space();
            if self.is_eof() {
                return Err(self.unclosed_class_error());
            }
            if self.char() != '-' || self.peek_space() == Some(']') {
                return prim1.into_class_set_item(self);
            }
            if !self.bump_and_bump_space() {
                return Err(self.unclosed_class_error());
            }
            let prim2 = self.parse_set_class_item()?;
            let range = ast::ClassSetRange {
                span: Span::new(prim1.span().start, prim2.span().end),
                start: prim1.into_class_literal(self)?,
                end: prim2.into_class_literal(self)?,
            };
            if !range.is_valid() {
                return Err(self.error(range.span, ast::ErrorKind::ClassRangeInvalid));
            }
            Ok(ast::ClassSetItem::Range(range))
        }
    }

    let mut parser = MockParser::new("a - ]"); // Invalid input for parsing a range (unclosed)
    let result = parser.parse_set_class_range();
    assert!(result.is_err());
}

