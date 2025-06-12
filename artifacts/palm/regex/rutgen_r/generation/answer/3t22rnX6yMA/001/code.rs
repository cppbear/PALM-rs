// Answer 0

#[test]
fn test_parse_set_class_item_literal() {
    struct TestParser {
        input: Vec<char>,
        index: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            *self.input.get(self.index).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.index += 1;
        }

        fn span_char(&self) -> usize {
            self.index
        }

        fn parse_escape(&self) -> Result<Primitive, &'static str> {
            Err("Expected escape sequence but found a literal")
        }

        fn parse_set_class_item(&mut self) -> Result<Primitive, &'static str> {
            if self.char() == '\\' {
                self.parse_escape()
            } else {
                let primitive = Primitive::Literal(ast::Literal {
                    span: self.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c: self.char(),
                });
                self.bump();
                Ok(primitive)
            }
        }
    }

    let mut parser = TestParser { input: vec!['a'], index: 0 };
    let result = parser.parse_set_class_item();
    assert!(result.is_ok());
    if let Ok(primitive) = result {
        assert_eq!(primitive, Primitive::Literal(ast::Literal {
            span: 0,
            kind: ast::LiteralKind::Verbatim,
            c: 'a',
        }));
    }
}

#[test]
fn test_parse_set_class_item_multiple_literals() {
    struct TestParser {
        input: Vec<char>,
        index: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            *self.input.get(self.index).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.index += 1;
        }

        fn span_char(&self) -> usize {
            self.index
        }

        fn parse_escape(&self) -> Result<Primitive, &'static str> {
            Err("Expected escape sequence but found a literal")
        }

        fn parse_set_class_item(&mut self) -> Result<Primitive, &'static str> {
            if self.char() == '\\' {
                self.parse_escape()
            } else {
                let primitive = Primitive::Literal(ast::Literal {
                    span: self.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c: self.char(),
                });
                self.bump();
                Ok(primitive)
            }
        }
    }

    let mut parser = TestParser { input: vec!['a', 'b', 'c'], index: 0 };
    for expected in vec!['a', 'b', 'c'] {
        let result = parser.parse_set_class_item();
        assert!(result.is_ok());
        if let Ok(primitive) = result {
            assert_eq!(primitive, Primitive::Literal(ast::Literal {
                span: parser.index - 1,
                kind: ast::LiteralKind::Verbatim,
                c: expected,
            }));
        }
    }
}

#[test]
fn test_parse_set_class_item_boundary() {
    struct TestParser {
        input: Vec<char>,
        index: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            *self.input.get(self.index).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.index += 1;
        }

        fn span_char(&self) -> usize {
            self.index
        }

        fn parse_escape(&self) -> Result<Primitive, &'static str> {
            Err("Expected escape sequence but found a literal")
        }

        fn parse_set_class_item(&mut self) -> Result<Primitive, &'static str> {
            if self.char() == '\\' {
                self.parse_escape()
            } else {
                let primitive = Primitive::Literal(ast::Literal {
                    span: self.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c: self.char(),
                });
                self.bump();
                Ok(primitive)
            }
        }
    }

    let mut parser = TestParser { input: vec![], index: 0 };
    let result = parser.parse_set_class_item();
    assert!(result.is_err());
}

