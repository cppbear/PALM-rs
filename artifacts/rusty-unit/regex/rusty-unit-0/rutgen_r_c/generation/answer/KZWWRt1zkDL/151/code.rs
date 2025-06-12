// Answer 0

#[test]
fn test_parse_escape_hex() {
    struct TestParser<'s> {
        pos: Position,
        pattern: &'s str,
        octal: bool,
    }

    impl<'s> TestParser<'s> {
        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap()
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset + 1 <= self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from(self.pattern), span }
        }

        fn parse_hex(&self) -> Result<ast::Literal> {
            Err(self.error(Span::new(self.pos, self.pos), ast::ErrorKind::EscapeUnexpectedEof))
        }
        
        fn parser(&self) -> &TestParser<'s> {
            self
        }
        
        fn parse_escape(&mut self) -> Result<Primitive> {
            assert_eq!(self.char(), '\\');
            let start = self.pos();
            if !self.bump() {
                return Err(self.error(
                    Span::new(start, self.pos()),
                    ast::ErrorKind::EscapeUnexpectedEof,
                ));
            }
            let c = self.char();
            match c {
                'x' => {
                    let mut lit = self.parse_hex()?;
                    lit.span.start = start;
                    return Ok(Primitive::Literal(lit));
                }
                _ => {}
            }
            // Additional parsing logic...
            Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::EscapeUnrecognized))
        }
    }

    let mut parser = TestParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: r"\x",
        octal: true,
    };

    assert_eq!(parser.parse_escape().is_err(), true);
}

#[test]
fn test_parse_escape_invalid_octal() {
    struct TestParser<'s> {
        pos: Position,
        pattern: &'s str,
        octal: bool,
    }

    impl<'s> TestParser<'s> {
        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap()
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset + 1 <= self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from(self.pattern), span }
        }

        fn parser(&self) -> &TestParser<'s> {
            self
        }
        
        fn parse_escape(&mut self) -> Result<Primitive> {
            let start = self.pos();
            if !self.bump() {
                return Err(self.error(
                    Span::new(start, self.pos()),
                    ast::ErrorKind::EscapeUnexpectedEof,
                ));
            }
            let c = self.char();
            if c.is_digit(10) {
                return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::UnsupportedBackreference));
            }
            Ok(Primitive::Literal(ast::Literal { span: Span::new(start, start), kind: ast::LiteralKind::Verbatim, c }))
        }
    }

    let mut parser = TestParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: r"\8",
        octal: false,
    };

    assert_eq!(parser.parse_escape().is_err(), true);
}

#[test]
fn test_parse_escape_octal_not_supported() {
    struct TestParser<'s> {
        pos: Position,
        pattern: &'s str,
        octal: bool,
    }

    impl<'s> TestParser<'s> {
        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap()
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset + 1 <= self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: String::from(self.pattern), span }
        }

        fn parser(&self) -> &TestParser<'s> {
            self
        }
        
        fn parse_escape(&mut self) -> Result<Primitive> {
            let start = self.pos();
            if !self.bump() {
                return Err(self.error(
                    Span::new(start, self.pos()),
                    ast::ErrorKind::EscapeUnexpectedEof,
                ));
            }
            let c = self.char();
            
            if c >= '0' && c <= '7' && !self.octal {
                  return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::UnsupportedBackreference));
            }
            Ok(Primitive::Literal(ast::Literal { span: Span::new(start, start), kind: ast::LiteralKind::Verbatim, c }))
        }
    }

    let mut parser = TestParser {
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: r"\1",
        octal: false,
    };

    let result = parser.parse_escape();
    assert!(result.is_err());
}

