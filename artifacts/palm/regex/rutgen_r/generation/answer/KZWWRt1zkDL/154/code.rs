// Answer 0

#[test]
fn test_parse_escape_invalid_octal() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }
    
    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<Primitive> {
            Err(Error::new())
        }

        fn parser(&self) -> &Parser {
            &Parser { octal: false }
        }
    }

    let mut parser = TestParser { input: "\\8", pos: 0 };
    let result = parser.parse_escape();
    assert!(result.is_err());
}

#[test]
fn test_parse_escape_valid_hex() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }
    
    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<Primitive> {
            Err(Error::new())
        }

        fn parser(&self) -> &Parser {
            &Parser { octal: true }
        }

        fn parse_hex(&self) -> Result<Literal> {
            Ok(Literal { span: Span::new(0, 2), kind: LiteralKind::Hex, c: 'x' })
        }
    }

    let mut parser = TestParser { input: "\\x", pos: 0 };
    let result = parser.parse_escape();
    assert!(result.is_ok());
}

#[test]
fn test_parse_escape_invalid_unicode() {
    struct TestParser {
        input: &'static str,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<Primitive> {
            Err(Error::new())
        }

        fn parser(&self) -> &Parser {
            &Parser { octal: false }
        }

        fn parse_unicode_class(&self) -> Result<Class> {
            Err(Error::new())
        }
    }

    let mut parser = TestParser { input: "\\p", pos: 0 };
    let result = parser.parse_escape();
    assert!(result.is_err());
}

