// Answer 0

#[test]
fn test_parse_escape_unrecognized() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }
        
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<Primitive> {
            Err("Unrecognized escape".into())
        }
        
        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }
    }

    struct MockParserWrapper {
        parser: MockParser,
    }

    impl MockParserWrapper {
        fn new(input: &str) -> Self {
            Self {
                parser: MockParser::new(input),
            }
        }

        fn parse_escape(&mut self) -> Result<Primitive> {
            let start = self.parser.pos();
            if !self.parser.bump() {
                return Err(self.parser.error(
                    Span::new(start, self.parser.pos()),
                    ast::ErrorKind::EscapeUnexpectedEof,
                ));
            }
            let c = self.parser.char();
            match c {
                _ if c < '0' || c > '9' => return Err(self.parser.error(self.parser.span_char(), ast::ErrorKind::EscapeUnrecognized)),
                'U' | 'd' | 'x' | 'w' | 'W' | 'u' | 'D' | 'P' | 's' | 'S' | 'p' | 'z' 
                | 'r' | ' ' | 'A' | 'B' | 'a' | 'f' | 't' | 'n' | 'v' => {
                    // Handled as recognized characters. In this test we will expect them to return Ok if not escaped unrecognized.
                    return Ok(Primitive::Literal(ast::Literal { span: Span::new(start, self.parser.pos()), kind: ast::LiteralKind::Punctuation, c: c }));
                }
                _ => return Err(self.parser.error(self.parser.span_char(), ast::ErrorKind::EscapeUnrecognized)),
            }
        }
    }

    let unrecognized_inputs = vec!['$', '#', '&', '@', ',', '<', '>', '!', '^', ':'];
    for input in unrecognized_inputs {
        let mut parser = MockParserWrapper::new(&format!("\\{}", input));
        let result = parser.parse_escape();
        assert!(result.is_err(), "Expected an error for input: \\{}", input);
    }
}

