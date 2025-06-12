// Answer 0

#[test]
fn test_parse_set_class_range_valid_range() {
    struct TestContext {
        input: Vec<char>,
        pos: usize,
    }

    impl TestContext {
        fn parse_set_class_item(&self) -> Result<ast::ClassSetItem> {
            // Simulate parsing a valid set class item
            Ok(ast::ClassSetItem::Literal(self.input[self.pos]))
        }
        
        fn bump_space(&mut self) {
            self.pos += 1; // Simulating bumping space by incrementing position
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn peek_space(&self) -> Option<char> {
            if self.pos + 1 < self.input.len() {
                Some(self.input[self.pos + 1])
            } else {
                None
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1; // Simulating bumping and bumping space
            !self.is_eof()
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error::UnclosedClass
        }
        
        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::RangeError(span, kind)
        }
    }

    let mut ctx = TestContext {
        input: vec!['a', '-', 'z'],
        pos: 0,
    };
    
    let result = ctx.parse_set_class_range();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    struct TestContext {
        input: Vec<char>,
        pos: usize,
    }

    impl TestContext {
        fn parse_set_class_item(&self) -> Result<ast::ClassSetItem> {
            // Simulating parsing an invalid set class item
            Ok(ast::ClassSetItem::Literal(self.input[self.pos]))
        }
        
        fn bump_space(&mut self) {
            self.pos += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn peek_space(&self) -> Option<char> {
            if self.pos + 1 < self.input.len() {
                Some(self.input[self.pos + 1])
            } else {
                None
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1;
            !self.is_eof()
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error::UnclosedClass
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::RangeError(span, kind)
        }
    }

    let mut ctx = TestContext {
        input: vec!['a', '-', 'b'],
        pos: 0,
    };

    let result = ctx.parse_set_class_range();
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_parse_set_class_range_unclosed_class() {
    struct TestContext {
        input: Vec<char>,
        pos: usize,
    }

    impl TestContext {
        fn parse_set_class_item(&self) -> Result<ast::ClassSetItem> {
            // Simulating parsing a valid set class item
            Ok(ast::ClassSetItem::Literal(self.input[self.pos]))
        }

        fn bump_space(&mut self) {
            self.pos += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn peek_space(&self) -> Option<char> {
            if self.pos + 1 < self.input.len() {
                Some(self.input[self.pos + 1])
            } else {
                None
            }
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.pos += 1;
            false // Simulating failure on bump
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error::UnclosedClass
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error::RangeError(span, kind)
        }
    }

    let mut ctx = TestContext {
        input: vec!['a', '-', 'b'],
        pos: 0,
    };

    ctx.parse_set_class_range();
}

