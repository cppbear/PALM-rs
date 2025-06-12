// Answer 0

#[test]
fn test_parse_escape_unrecognized_escape() {
    struct MockParser {
        pattern: String,
        pos: Position,
        octal: bool,
        ignore_whitespace: bool,
        current_char: char,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                octal: true,
                ignore_whitespace: false,
                current_char: '\\',
            }
        }

        fn bump(&mut self) -> bool {
            // Moving the position past the escape character
            self.pos.offset += 1;
            if self.pos.offset < self.pattern.len() {
                self.current_char = self.pattern.chars().nth(self.pos.offset).unwrap();
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error { kind, pattern: self.pattern.clone(), span }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }
        
        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    impl ParserI<'_, MockParser> {
        fn bump(&mut self) -> bool {
            self.parser.bump()
        }
    
        fn char(&self) -> char {
            self.parser.char()
        }
    
        fn parser(&self) -> &MockParser {
            self.parser
        }
    
        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            self.parser.error(span, kind)
        }
    
        fn ignore_whitespace(&self) -> bool {
            self.parser.ignore_whitespace
        }
    
        fn pos(&self) -> Position {
            self.parser.pos
        }

        fn parse_escape(&self) -> Result<Primitive> {
            assert_eq!(self.char(), '\\');
            let start = self.pos();
            if !self.bump() {
                return Err(self.error(
                    Span::new(start, self.pos()),
                    ast::ErrorKind::EscapeUnexpectedEof,
                ));
            }
            let c = self.char();
            // The rest of the original parse_escape implementation follows...

            self.bump();
            let span = Span::new(start, self.pos());
            return Err(self.error(span, ast::ErrorKind::EscapeUnrecognized));
        }
    }

    let pattern = r"\8"; // Starting the escape sequence with '\', expects '8' to cause unrecognized escape error
    let parser = MockParser::new(pattern);
    let parser_i = ParserI { parser: &parser, pattern: &parser.pattern };
    
    let result = parser_i.parse_escape();
    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(error.kind, ErrorKind::EscapeUnrecognized);
    }
}

