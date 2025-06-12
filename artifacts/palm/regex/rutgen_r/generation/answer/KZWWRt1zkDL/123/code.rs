// Answer 0

#[test]
fn test_parse_escape_backslash_with_meta_character() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn parser(&self) -> &Self {
            self
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> std::result::Result<Primitive, ()> {
            Err(())
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = TestParser {
        input: vec!['\\', 'a'],
        pos: 0,
        octal: false,
    };

    let result = parser.parse_escape();
    assert!(result.is_ok());
    if let Ok(Primitive::Literal(literal)) = result {
        assert_eq!(literal.kind, ast::LiteralKind::Punctuation);
        assert_eq!(literal.c, 'a');
    }
}

#[test]
fn test_parse_escape_unrecognized_escape() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn parser(&self) -> &Self {
            self
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> std::result::Result<Primitive, ()> {
            Err(())
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = TestParser {
        input: vec!['\\', 'X'],
        pos: 0,
    };

    let result = parser.parse_escape();
    assert!(result.is_err());
}

#[test]
fn test_parse_escape_perl_class() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() - 1 {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn parser(&self) -> &Self {
            self
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> std::result::Result<Primitive, ()> {
            Err(())
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = TestParser {
        input: vec!['\\', 'd'],
        pos: 0,
    };

    let result = parser.parse_escape();
    assert!(result.is_ok());
}

