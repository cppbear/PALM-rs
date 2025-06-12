// Answer 0

#[test]
fn test_parse_dot() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn span_char(&self) -> usize {
            self.pos // Just returning the position as a span for simplicity
        }

        fn parse_escape(&self) -> Result<ast::Primitive, &'static str> {
            Err("Not implemented")
        }

        // Mock implementation of parse_primitive
        fn parse_primitive(&mut self) -> Result<ast::Primitive, &'static str> {
            match self.char() {
                '\\' => self.parse_escape(),
                '.' => {
                    let ast = ast::Primitive::Dot(self.span_char());
                    self.bump();
                    Ok(ast)
                }
                _ => {
                    let ast = ast::Primitive::Literal(ast::Literal {
                        span: self.span_char(),
                        kind: ast::LiteralKind::Verbatim,
                        c: self.char(),
                    });
                    self.bump();
                    Ok(ast)
                }
            }
        }
    }

    let mut parser = TestParser::new(".");
    let result = parser.parse_primitive().expect("Failed to parse dot");
    if let ast::Primitive::Dot(span) = result {
        assert_eq!(span, 0);
    } else {
        panic!("Expected Dot variant");
    }
}

#[test]
fn test_parse_start_assertion() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn span_char(&self) -> usize {
            self.pos // Just returning the position as a span for simplicity
        }

        fn parse_escape(&self) -> Result<ast::Primitive, &'static str> {
            Err("Not implemented")
        }

        // Mock implementation of parse_primitive
        fn parse_primitive(&mut self) -> Result<ast::Primitive, &'static str> {
            match self.char() {
                '^' => {
                    let ast = ast::Primitive::Assertion(ast::Assertion {
                        span: self.span_char(),
                        kind: ast::AssertionKind::StartLine,
                    });
                    self.bump();
                    Ok(ast)
                }
                _ => {
                    let ast = ast::Primitive::Literal(ast::Literal {
                        span: self.span_char(),
                        kind: ast::LiteralKind::Verbatim,
                        c: self.char(),
                    });
                    self.bump();
                    Ok(ast)
                }
            }
        }
    }

    let mut parser = TestParser::new("^");
    let result = parser.parse_primitive().expect("Failed to parse start assertion");
    if let ast::Primitive::Assertion(ref assertion) = result {
        assert_eq!(assertion.kind, ast::AssertionKind::StartLine);
        assert_eq!(assertion.span, 0);
    } else {
        panic!("Expected Assertion variant");
    }
}

#[test]
fn test_parse_end_assertion() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn span_char(&self) -> usize {
            self.pos // Just returning the position as a span for simplicity
        }

        fn parse_escape(&self) -> Result<ast::Primitive, &'static str> {
            Err("Not implemented")
        }

        // Mock implementation of parse_primitive
        fn parse_primitive(&mut self) -> Result<ast::Primitive, &'static str> {
            match self.char() {
                '$' => {
                    let ast = ast::Primitive::Assertion(ast::Assertion {
                        span: self.span_char(),
                        kind: ast::AssertionKind::EndLine,
                    });
                    self.bump();
                    Ok(ast)
                }
                _ => {
                    let ast = ast::Primitive::Literal(ast::Literal {
                        span: self.span_char(),
                        kind: ast::LiteralKind::Verbatim,
                        c: self.char(),
                    });
                    self.bump();
                    Ok(ast)
                }
            }
        }
    }

    let mut parser = TestParser::new("$");
    let result = parser.parse_primitive().expect("Failed to parse end assertion");
    if let ast::Primitive::Assertion(ref assertion) = result {
        assert_eq!(assertion.kind, ast::AssertionKind::EndLine);
        assert_eq!(assertion.span, 0);
    } else {
        panic!("Expected Assertion variant");
    }
}

#[test]
fn test_parse_literal() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn span_char(&self) -> usize {
            self.pos // Just returning the position as a span for simplicity
        }

        fn parse_escape(&self) -> Result<ast::Primitive, &'static str> {
            Err("Not implemented")
        }

        // Mock implementation of parse_primitive
        fn parse_primitive(&mut self) -> Result<ast::Primitive, &'static str> {
            let c = self.char();
            let ast = ast::Primitive::Literal(ast::Literal {
                span: self.span_char(),
                kind: ast::LiteralKind::Verbatim,
                c: c,
            });
            self.bump();
            Ok(ast)
        }
    }

    let mut parser = TestParser::new("a");
    let result = parser.parse_primitive().expect("Failed to parse literal");
    if let ast::Primitive::Literal(literal) = result {
        assert_eq!(literal.c, 'a');
        assert_eq!(literal.span, 0);
    } else {
        panic!("Expected Literal variant");
    }
}

