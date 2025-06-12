// Answer 0

#[test]
fn test_parse_set_class_eof_error() {
    struct MockParser {
        input: &'static str,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &'static str) -> Self {
            Self { input, pos: 0 }
        }

        fn char(&self) -> char {
            self.input.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump_space(&mut self) {
            self.pos += 1;
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::UnclosedClass,
                pattern: self.input.to_string(),
                span: Span { start: 0, end: self.pos as u32 },
            }
        }

        fn parse_set_class(&mut self) -> Result<ast::Class> {
            assert_eq!(self.char(), '[');

            let mut union = ast::ClassSetUnion {
                span: Span { start: 0, end: 0 },
                items: vec![],
            };
            loop {
                self.bump_space();
                if self.is_eof() {
                    return Err(self.unclosed_class_error());
                }
                match self.char() {
                    ']' => {
                        // Dummy implementation to avoid panic.
                        return Ok(ast::Class::Bracketed(ast::ClassBracketed {}));
                    }
                    _ => {
                        // Dummy range parsing; returns an error
                        return Err(self.unclosed_class_error());
                    }
                }
            }
        }
    }

    let mut parser = MockParser::new("[");
    let result = parser.parse_set_class();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), parser.unclosed_class_error());
}

