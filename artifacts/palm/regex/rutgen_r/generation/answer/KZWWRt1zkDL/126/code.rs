// Answer 0

#[test]
fn test_parse_escape_word_boundary() {
    struct Parser {
        input: Vec<char>,
        pos: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Parser {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos < self.input.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<Primitive> {
            Err("error".into())
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
            self.bump(); // Simulate advancing after processing the char.
            if c == 'b' {
                return Ok(Primitive::Assertion(ast::Assertion {
                    span: Span::new(start, self.pos()),
                    kind: ast::AssertionKind::WordBoundary,
                }));
            }
            // Handle other cases...
            Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::EscapeUnrecognized))
        }
    }

    let mut parser = Parser::new(r"\b");
    let result = parser.parse_escape();
    assert!(result.is_ok());
    if let Ok(Primitive::Assertion(assertion)) = result {
        assert_eq!(assertion.kind, ast::AssertionKind::WordBoundary);
    }
}

