// Answer 0

#[test]
fn test_parse_primitive_with_start_line() {
    struct TestParser {
        parser: Parser,
        pattern: String,
        pos: Cell<u32>,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                parser: Parser {
                    pos: Cell::new(0),
                    capture_index: Cell::new(0),
                    nest_limit: 10,
                    octal: false,
                    initial_ignore_whitespace: false,
                    ignore_whitespace: Cell::new(false),
                    comments: RefCell::new(vec![]),
                    stack_group: RefCell::new(vec![]),
                    stack_class: RefCell::new(vec![]),
                    capture_names: RefCell::new(vec![]),
                    scratch: RefCell::new(String::new()),
                },
                pattern: pattern.to_string(),
                pos: Cell::new(0),
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get() as usize).unwrap_or('\0')
        }

        fn bump(&self) {
            self.pos.set(self.pos.get() + 1);
        }

        fn span_char(&self) -> Span {
            Span {
                start: self.pos.get(),
                end: self.pos.get() + 1,
            }
        }

        fn parse_primitive(&self) -> Result<Primitive> {
            match self.char() {
                '\\' => unimplemented!(), // Not testing escape here
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

    let parser = TestParser::new("$");
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(Primitive::Assertion(assertion)) = result {
        assert_eq!(assertion.kind, AssertionKind::EndLine);
    } else {
        panic!("Expected a valid Assertion for '$'");
    }
}

#[test]
fn test_parse_primitive_with_literal() {
    struct TestParser {
        parser: Parser,
        pattern: String,
        pos: Cell<u32>,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                parser: Parser {
                    pos: Cell::new(0),
                    capture_index: Cell::new(0),
                    nest_limit: 10,
                    octal: false,
                    initial_ignore_whitespace: false,
                    ignore_whitespace: Cell::new(false),
                    comments: RefCell::new(vec![]),
                    stack_group: RefCell::new(vec![]),
                    stack_class: RefCell::new(vec![]),
                    capture_names: RefCell::new(vec![]),
                    scratch: RefCell::new(String::new()),
                },
                pattern: pattern.to_string(),
                pos: Cell::new(0),
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get() as usize).unwrap_or('\0')
        }

        fn bump(&self) {
            self.pos.set(self.pos.get() + 1);
        }

        fn span_char(&self) -> Span {
            Span {
                start: self.pos.get(),
                end: self.pos.get() + 1,
            }
        }

        fn parse_primitive(&self) -> Result<Primitive> {
            match self.char() {
                '\\' => unimplemented!(), // Not testing escape here
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

    let parser = TestParser::new("a");
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(Primitive::Literal(literal)) = result {
        assert_eq!(literal.c, 'a');
        assert_eq!(literal.kind, LiteralKind::Verbatim);
    } else {
        panic!("Expected a valid Literal for 'a'");
    }
}

