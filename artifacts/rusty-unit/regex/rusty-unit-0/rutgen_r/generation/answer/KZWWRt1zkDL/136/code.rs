// Answer 0

#[test]
fn test_parse_escape_with_backreference() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                pos: 0,
                octal: false,
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

        fn error(&self, _: Span, _: ast::ErrorKind) -> String {
            String::from("error")
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn parse_hex(&self) -> Result<ast::Literal, String> {
            Ok(ast::Literal {
                span: self.span_char(),
                kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::HexHex),
                c: 'x',
            })
        }

        fn parse_unicode_class(&self) -> Result<ast::UnicodeClass, String> {
            Ok(ast::UnicodeClass { span: self.span_char() })
        }

        fn parse_perl_class(&self) -> ast::PerlClass {
            ast::PerlClass { span: self.span_char() }
        }
    }

    let mut parser = TestParser::new(r"\u");
    assert!(parser.bump()); // should be true
    let result = parser.parse_escape();
    assert!(result.is_ok());
}

#[test]
fn test_parse_escape_with_octal_invalid() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        octal: bool,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                pos: 0,
                octal: false,
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

        fn error(&self, _: Span, _: ast::ErrorKind) -> String {
            String::from("error")
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn parse_hex(&self) -> Result<ast::Literal, String> {
            Ok(ast::Literal {
                span: self.span_char(),
                kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::HexHex),
                c: 'x',
            })
        }

        fn parse_unicode_class(&self) -> Result<ast::UnicodeClass, String> {
            Err("unexpected eof".into())
        }

        fn parse_perl_class(&self) -> ast::PerlClass {
            ast::PerlClass { span: self.span_char() }
        }
    }

    let mut parser = TestParser::new(r"\8"); // Invalid octal
    assert!(parser.bump()); // should be true
    let result = parser.parse_escape();
    assert!(result.is_err());
}

#[test]
fn test_parse_escape_invalid_character() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                pos: 0,
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

        fn error(&self, _: Span, _: ast::ErrorKind) -> String {
            String::from("error")
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn parse_hex(&self) -> Result<ast::Literal, String> {
            Ok(ast::Literal {
                span: self.span_char(),
                kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::HexHex),
                c: 'x',
            })
        }

        fn parse_unicode_class(&self) -> Result<ast::UnicodeClass, String> {
            Ok(ast::UnicodeClass { span: self.span_char() })
        }

        fn parse_perl_class(&self) -> ast::PerlClass {
            ast::PerlClass { span: self.span_char() }
        }
    }

    let mut parser = TestParser::new(r"\$"); // special character
    assert!(parser.bump()); // should be true
    let result = parser.parse_escape();
    assert!(result.is_ok());
}

