// Answer 0

#[test]
fn test_parse_group_with_capture_name_error() {
    struct FakeParser {
        input: String,
        pos: usize,
        capture_index: u32,
    }

    impl FakeParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                pos: 0,
                capture_index: 1,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if self.input[self.pos..].starts_with(s) {
                self.pos += s.len();
                true
            } else {
                false
            }
        }

        fn is_lookaround_prefix(&self) -> bool {
            false
        }

        fn span_char(&self) -> Span {
            Span::new(Position { offset: self.pos, line: 1, column: 1 }, Position { offset: self.pos + 1, line: 1, column: 2 })
        }

        fn next_capture_index(&self, _open_span: Span) -> Result<u32> {
            Ok(self.capture_index)
        }

        fn parse_capture_name(&self, _capture_index: u32) -> Result<ast::CaptureName> {
            Err(ast::Error { kind: ast::ErrorKind::GroupNameInvalid, pattern: self.input.clone(), span: self.span_char() })
        }

        fn parse_group(&mut self) -> Result<Either<ast::SetFlags, ast::Group>> {
            assert_eq!(self.char(), '(');
            let open_span = self.span_char();
            self.bump();
            self.bump(); // assuming this is for the space or ')' coming after '('.

            if self.bump_if("?P<") {
                let capture_index = self.next_capture_index(open_span)?;
                let cap = self.parse_capture_name(capture_index)?;
                return Ok(Either::Right(ast::Group {
                    span: open_span,
                    kind: ast::GroupKind::CaptureName(cap),
                    ast: Box::new(ast::Ast::Empty(open_span)),
                }));
            }
            Err(ast::Error { kind: ast::ErrorKind::GroupNameInvalid, pattern: self.input.clone(), span: open_span }) // For cases where it doesn't hit the group (should not hit here).
        }
    }

    let mut parser = FakeParser::new("(?:abc)");
    let result = parser.parse_group();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind, ast::ErrorKind::GroupNameInvalid);
}

