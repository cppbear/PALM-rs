// Answer 0

#[test]
fn test_parse_escape_with_octal_not_supported() {
    struct Parser {
        input: Vec<char>,
        position: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn parser(&self) -> ParserConfig {
            ParserConfig { octal: false }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<Primitive, ast::Error> {
            Err(ast::Error { span, kind })
        }

        fn span_char(&self) -> Span {
            Span::new(self.position, self.position + 1)
        }
    }

    struct ParserConfig {
        octal: bool,
    }

    let mut parser = Parser::new(r"\89");
    assert_eq!(parser.char(), '\\'); // starting character
    let result = parse_escape(&mut parser);
    assert!(result.is_err());
}

#[test]
fn test_parse_escape_with_unrecognized_escape() {
    struct Parser {
        input: Vec<char>,
        position: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn parser(&self) -> ParserConfig {
            ParserConfig { octal: false }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<Primitive, ast::Error> {
            Err(ast::Error { span, kind })
        }

        fn span_char(&self) -> Span {
            Span::new(self.position, self.position + 1)
        }
    }

    struct ParserConfig {
        octal: bool,
    }

    let mut parser = Parser::new(r"\8");
    parser.bump(); // move to '8'
    let result = parse_escape(&mut parser);
    assert!(result.is_err());
}

#[test]
fn test_parse_escape_with_numerical_range() {
    struct Parser {
        input: Vec<char>,
        position: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() - 1 {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn parser(&self) -> ParserConfig {
            ParserConfig { octal: false }
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<Primitive, ast::Error> {
            Err(ast::Error { span, kind })
        }

        fn span_char(&self) -> Span {
            Span::new(self.position, self.position + 1)
        }
    }

    struct ParserConfig {
        octal: bool,
    }

    let mut parser = Parser::new(r"\99");
    parser.bump(); // move to '9'
    let result = parse_escape(&mut parser);
    assert!(result.is_err());
}

