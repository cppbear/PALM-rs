// Answer 0

#[test]
fn test_parse_primitive_escape() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn span_char(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn parse_escape(&self) -> Result<Primitive, &'static str> {
            Ok(Primitive::Escape(self.span_char())) // Simulating escape parsing
        }

        fn parse_primitive(&mut self) -> Result<Primitive, &'static str> {
            match self.char() {
                '\\' => self.parse_escape(),
                '.' => {
                    let ast = Primitive::Dot(self.span_char());
                    self.bump();
                    Ok(ast)
                }
                '^' => {
                    let ast = Primitive::Assertion(ast::Assertion {
                        span: self.span_char(),
                        kind: ast::AssertionKind::StartLine,
                    });
                    self.bump();
                    Ok(ast)
                }
                '$' => {
                    let ast = Primitive::Assertion(ast::Assertion {
                        span: self.span_char(),
                        kind: ast::AssertionKind::EndLine,
                    });
                    self.bump();
                    Ok(ast)
                }
                c => {
                    let ast = Primitive::Literal(ast::Literal {
                        span: self.span_char(),
                        kind: ast::LiteralKind::Verbatim,
                        c: c,
                    });
                    self.bump();
                    Ok(ast)
                }
            }
        }
    }

    let mut parser = TestParser { input: r"\n", pos: 0 };
    let result = parser.parse_primitive();
    assert!(result.is_ok());
}

#[test]
fn test_parse_primitive_dot() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn span_char(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn parse_primitive(&mut self) -> Result<Primitive, &'static str> {
            match self.char() {
                '\\' => self.parse_escape(),
                '.' => {
                    let ast = Primitive::Dot(self.span_char());
                    self.bump();
                    Ok(ast)
                }
                _ => unimplemented!(),
            }
        }

        fn parse_escape(&self) -> Result<Primitive, &'static str> {
            Ok(Primitive::Escape(self.span_char()))
        }
    }

    let mut parser = TestParser { input: ".", pos: 0 };
    let result = parser.parse_primitive();
    assert!(result.is_ok());
}

#[test]
fn test_parse_primitive_start_line() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn span_char(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn parse_primitive(&mut self) -> Result<Primitive, &'static str> {
            match self.char() {
                '^' => {
                    let ast = Primitive::Assertion(ast::Assertion {
                        span: self.span_char(),
                        kind: ast::AssertionKind::StartLine,
                    });
                    self.bump();
                    Ok(ast)
                }
                _ => unimplemented!(),
            }
        }
    }

    let mut parser = TestParser { input: "^", pos: 0 };
    let result = parser.parse_primitive();
    assert!(result.is_ok());
}

#[test]
fn test_parse_primitive_end_line() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn span_char(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn parse_primitive(&mut self) -> Result<Primitive, &'static str> {
            match self.char() {
                '$' => {
                    let ast = Primitive::Assertion(ast::Assertion {
                        span: self.span_char(),
                        kind: ast::AssertionKind::EndLine,
                    });
                    self.bump();
                    Ok(ast)
                }
                _ => unimplemented!(),
            }
        }
    }

    let mut parser = TestParser { input: "$", pos: 0 };
    let result = parser.parse_primitive();
    assert!(result.is_ok());
}

#[test]
fn test_parse_primitive_literal() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        fn span_char(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += self.char().len_utf8();
        }

        fn parse_primitive(&mut self) -> Result<Primitive, &'static str> {
            match self.char() {
                c => {
                    let ast = Primitive::Literal(ast::Literal {
                        span: self.span_char(),
                        kind: ast::LiteralKind::Verbatim,
                        c: c,
                    });
                    self.bump();
                    Ok(ast)
                }
            }
        }
    }

    let mut parser = TestParser { input: "a", pos: 0 };
    let result = parser.parse_primitive();
    assert!(result.is_ok());
}

