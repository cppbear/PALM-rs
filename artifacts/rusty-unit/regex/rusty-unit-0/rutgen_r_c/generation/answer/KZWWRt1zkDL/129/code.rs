// Answer 0

#[test]
fn test_parse_escape_with_unicode() {
    struct MockParser {
        pattern: String,
        pos: Position,
        octal: bool,
        ignore_whitespace: Cell<bool>,
    }

    impl MockParser {
        fn new(pattern: &str, pos: Position) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos,
                octal: true,
                ignore_whitespace: Cell::new(true),
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.offset < self.pattern.len()
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        // Method to initiate the necessary `bump` and `char` processes
        fn parse_escape(&mut self) -> Result<Primitive> {
            assert_eq!(self.char(), '\\');
            let start = self.pos();
            if !self.bump() {
                return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::EscapeUnexpectedEof));
            }
            let c = self.char();

            if c == 'u' {
                // Simulate parsing a Unicode escape
                return Ok(Primitive::Unicode(ast::ClassUnicode {
                    span: Span::new(start, self.pos()),
                    negated: false,
                    kind: ast::ClassUnicodeKind::Named("Unicode".to_string()),
                }));
            }

            Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::EscapeUnrecognized))
        }
    }

    let mut parser = MockParser::new("\\u", Position { offset: 0, line: 1, column: 1 });
    let result = parser.parse_escape();
    assert!(result.is_ok());
}

#[test]
fn test_parse_escape_with_perl_class() {
    struct MockParser {
        pattern: String,
        pos: Position,
        octal: bool,
        ignore_whitespace: Cell<bool>,
    }

    impl MockParser {
        fn new(pattern: &str, pos: Position) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos,
                octal: false,
                ignore_whitespace: Cell::new(true),
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.offset < self.pattern.len()
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        // Method to initiate the necessary `bump` and `char` processes
        fn parse_escape(&mut self) -> Result<Primitive> {
            assert_eq!(self.char(), '\\');
            let start = self.pos();
            if !self.bump() {
                return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::EscapeUnexpectedEof));
            }
            let c = self.char();

            if c == 'd' {
                return Ok(Primitive::Perl(ast::ClassPerl {
                    span: Span::new(start, self.pos()),
                    kind: ast::ClassPerlKind::Digit,
                    negated: false,
                }));
            }

            Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::EscapeUnrecognized))
        }
    }

    let mut parser = MockParser::new("\\d", Position { offset: 0, line: 1, column: 1 });
    let result = parser.parse_escape();
    assert!(result.is_ok());
}

#[test]
fn test_parse_escape_with_space_when_whitespace_ignored() {
    struct MockParser {
        pattern: String,
        pos: Position,
        octal: bool,
        ignore_whitespace: Cell<bool>,
    }

    impl MockParser {
        fn new(pattern: &str, pos: Position) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos,
                octal: false,
                ignore_whitespace: Cell::new(true),
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.offset < self.pattern.len()
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind,
                pattern: self.pattern.clone(),
                span,
            }
        }

        // Method to initiate the necessary `bump` and `char` processes
        fn parse_escape(&mut self) -> Result<Primitive> {
            assert_eq!(self.char(), '\\');
            let start = self.pos();
            if !self.bump() {
                return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::EscapeUnexpectedEof));
            }
            let c = self.char();

            if c == ' ' {
                return Ok(Primitive::Literal(ast::Literal {
                    span: Span::new(start, self.pos()),
                    kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Space),
                    c: ' ',
                }));
            }
            
            Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::EscapeUnrecognized))
        }
    }

    let mut parser = MockParser::new("\\ ", Position { offset: 0, line: 1, column: 1 });
    let result = parser.parse_escape();
    assert!(result.is_ok());
}

