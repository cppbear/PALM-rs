// Answer 0

#[test]
fn test_parse_escape_end_text_assertion() {
    struct MockParser {
        input: Vec<char>,
        position: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                position: 0,
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn error(&self, _: Span, _: ast::ErrorKind) -> ast::Error {
            ast::Error::new("error")
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn parser(&self) -> &MockParser {
            self
        }
    }

    let mut parser = MockParser::new("\\z");
    assert_eq!(
        parser.parse_escape(),
        Ok(Primitive::Assertion(ast::Assertion {
            span: Span::new(0, 2),
            kind: ast::AssertionKind::EndText,
        }))
    );
}

