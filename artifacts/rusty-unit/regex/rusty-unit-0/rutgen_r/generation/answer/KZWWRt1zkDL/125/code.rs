// Answer 0

#[test]
fn test_parse_escape_not_word_boundary() {
    struct FakeParser {
        input: Vec<char>,
        pos: usize,
    }

    impl FakeParser {
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
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Error").into())
        }

        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn parser(&self) -> &FakeParser {
            self
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
        
        fn parse_escape(&mut self) -> Result<Primitive> {
            assert_eq!(self.char(), '\\');
            let start = self.pos();
            if !self.bump() {
                return Err(self.error(Span::new(start, self.pos()), ast::ErrorKind::EscapeUnexpectedEof));
            }
            let c = self.char();
            self.bump(); // Simulate bumping ahead after recognizing the escape

            let span = Span::new(start, self.pos());
            match c {
                'B' => Ok(Primitive::Assertion(ast::Assertion {
                    span: span,
                    kind: ast::AssertionKind::NotWordBoundary,
                })),
                _ => Err(self.error(span, ast::ErrorKind::EscapeUnrecognized)),
            }
        }
    }

    let mut parser = FakeParser::new("\\B");
    let result = parser.parse_escape();
    assert!(result.is_ok());
    if let Ok(Primitive::Assertion(assertion)) = result {
        assert_eq!(assertion.kind, ast::AssertionKind::NotWordBoundary);
    } else {
        panic!("Expected Ok with a NotWordBoundary assertion");
    }
}

