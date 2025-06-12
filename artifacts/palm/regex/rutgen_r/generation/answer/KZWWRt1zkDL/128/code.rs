// Answer 0

#[test]
fn test_parse_escape_assertion_start_text() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
        octal: bool,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                position: 0,
                octal: false,
            }
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<Primitive> {
            Err(ast::ErrorKind::EscapeUnrecognized.into())
        }

        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos() + 1)
        }
    }

    let mut parser = MockParser::new("\\A");
    assert_eq!(parser.parse_escape(), Ok(Primitive::Assertion(ast::Assertion {
        span: parser.span_char(),
        kind: ast::AssertionKind::StartText,
    })));
}

#[test]
fn test_parse_escape_unrecognized_escape() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
        octal: bool,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                position: 0,
                octal: false,
            }
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<Primitive> {
            Err(ast::ErrorKind::EscapeUnrecognized.into())
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos() + 1)
        }
    }

    let mut parser = MockParser::new("\\z");
    assert_eq!(parser.parse_escape(), Err(ast::ErrorKind::EscapeUnrecognized.into()));
}

#[test]
fn test_parse_escape_invalid_octal() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
        octal: bool,
    }

    impl MockParser {
        fn new(input: &str, octal: bool) -> Self {
            MockParser {
                input: input.chars().collect(),
                position: 0,
                octal,
            }
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<Primitive> {
            Err(ast::ErrorKind::EscapeUnrecognized.into())
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos(), self.pos() + 1)
        }
    }

    // Testing with '8' which is invalid for octal when octal is false
    let mut parser = MockParser::new("\\8", false);
    assert_eq!(parser.parse_escape(), Err(ast::ErrorKind::UnsupportedBackreference.into()));
}

