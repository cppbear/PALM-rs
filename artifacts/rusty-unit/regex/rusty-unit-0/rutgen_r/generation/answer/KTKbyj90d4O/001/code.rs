// Answer 0

#[test]
fn test_parse_capture_name_eof() {
    struct TestParser {
        position: usize,
        input: &'static str,
    }

    impl TestParser {
        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            if self.position < self.input.len() {
                self.input.chars().nth(self.position).unwrap()
            } else {
                '\0' // EOF character
            }
        }

        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn span(&self) -> usize {
            self.position // Just a placeholder for the span
        }

        fn error(&self, _span: usize, error_kind: ast::ErrorKind) -> Result<ast::CaptureName, ast::ErrorKind> {
            Err(error_kind)
        }

        fn pattern(&self) -> &'static str {
            self.input
        }

        fn add_capture_name(&self, _caps: &ast::CaptureName) -> Result<(), ast::ErrorKind> {
            Ok(())
        }
    }

    let mut parser = TestParser {
        position: 0,
        input: "<",
    };

    let result = parser.parse_capture_name(0);
    assert_eq!(result, Err(ast::ErrorKind::GroupNameUnexpectedEof));
}

