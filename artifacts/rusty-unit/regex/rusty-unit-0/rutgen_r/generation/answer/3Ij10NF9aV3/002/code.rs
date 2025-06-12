// Answer 0

#[test]
fn test_push_group_left_set() {
    struct TestParser {
        input: Vec<char>,
        index: usize,
        ignore_whitespace: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.index]
        }

        fn parse_group(&mut self) -> Result<Either<ast::Flags, ast::Group>> {
            // Simulate parsing a group with flags
            if self.char() == '(' {
                self.index += 1; // Move past the '('
                return Ok(Either::Left(ast::Flags {}));
            }
            Err("Parse error".into())
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn parser(&self) -> &Self {
            self
        }

        fn span(&self) -> ast::Span {
            ast::Span {}
        }
    }

    let mut parser = TestParser {
        input: vec!['(', ')'],
        index: 0,
        ignore_whitespace: false,
    };

    let concat = ast::Concat {
        span: parser.span(),
        asts: vec![],
    };

    let result = parser.push_group(concat).unwrap();
    assert_eq!(result.asts.len(), 1);
}

#[test]
fn test_push_group_right_group() {
    struct TestParser {
        input: Vec<char>,
        index: usize,
        ignore_whitespace: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.index]
        }

        fn parse_group(&mut self) -> Result<Either<ast::Flags, ast::Group>> {
            // Simulate parsing a group
            if self.char() == '(' {
                self.index += 1; // Move past the '('
                return Ok(Either::Right(ast::Group {}));
            }
            Err("Parse error".into())
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn parser(&self) -> &Self {
            self
        }

        fn span(&self) -> ast::Span {
            ast::Span {}
        }
    }

    let mut parser = TestParser {
        input: vec!['(', ')'],
        index: 0,
        ignore_whitespace: false,
    };

    let concat = ast::Concat {
        span: parser.span(),
        asts: vec![],
    };

    let result = parser.push_group(concat).unwrap();
    assert!(result.asts.is_empty());
}

