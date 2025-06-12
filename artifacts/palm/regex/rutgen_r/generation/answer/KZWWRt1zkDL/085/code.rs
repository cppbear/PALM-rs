// Answer 0

#[test]
fn test_parse_escape_unsupported_backreference() {
    struct Parser {
        pos: usize,
        input: Vec<char>,
        octal: bool,
    }

    impl Parser {
        fn new(input: &str, octal: bool) -> Self {
            Parser {
                pos: 0,
                input: input.chars().collect(),
                octal,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, _: Span, _: ast::ErrorKind) -> Result<Primitive> {
            Err("error".into())
        }

        // Implement dummy versions of methods for tests
        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }
    }

    let mut parser = Parser::new("\\89", false); // Invalid octal
    parser.bump(); // Move past the escape character
    let result = parser.parse_escape();
    assert!(result.is_err());
}

#[test]
fn test_parse_escape_unrecognized_escape() {
    struct Parser {
        pos: usize,
        input: Vec<char>,
        octal: bool,
    }

    impl Parser {
        fn new(input: &str, octal: bool) -> Self {
            Parser {
                pos: 0,
                input: input.chars().collect(),
                octal,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &Self {
            self
        }

        fn error(&self, _: Span, _: ast::ErrorKind) -> Result<Primitive> {
            Err("error".into())
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }
    }

    let mut parser = Parser::new("\\ ", false); // Valid escape, but space not ignored
    parser.bump(); // Move past the escape character
    let result = parser.parse_escape();
    assert!(result.is_err());
}

