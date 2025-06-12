// Answer 0

#[test]
fn test_parse_escape_p_unrecognized_escape() {
    struct Parser {
        chars: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl Parser {
        fn new(chars: Vec<char>, octal: bool) -> Self {
            Self { chars, pos: 0, octal }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos < self.chars.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<Primitive> {
            Err(kind.into())
        }

        fn parser(&self) -> &Parser {
            self
        }

        fn parse_unicode_class(&mut self) -> Result<Class> {
            Err(ast::ErrorKind::EscapeInvalidClass.into())
        }
    }

    let mut parser = Parser::new(vec!['\\', 'p'], false);
    let result = parser.parse_escape();
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_parse_escape_P_unrecognized_escape() {
    struct Parser {
        chars: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl Parser {
        fn new(chars: Vec<char>, octal: bool) -> Self {
            Self { chars, pos: 0, octal }
        }

        fn char(&self) -> char {
            *self.chars.get(self.pos).unwrap()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos < self.chars.len()
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> Result<Primitive> {
            Err(kind.into())
        }

        fn parser(&self) -> &Parser {
            self
        }

        fn parse_unicode_class(&mut self) -> Result<Class> {
            Err(ast::ErrorKind::EscapeInvalidClass.into())
        }
    }

    let mut parser = Parser::new(vec!['\\', 'P'], false);
    let result = parser.parse_escape();
    assert_eq!(result.is_err(), true);
}

