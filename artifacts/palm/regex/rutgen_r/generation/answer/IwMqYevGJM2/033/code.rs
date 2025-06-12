// Answer 0

#[test]
fn test_parse_unicode_class_unexpected_eof() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.bump();
                return true;
            }
            false
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn span(&self) -> ast::Span {
            ast::Span::new(self.pos, self.pos)
        }

        fn error(&self, span: ast::Span, kind: ast::ErrorKind) -> Result<ast::ClassUnicode, ast::Error> {
            Err(ast::Error { span, kind })
        }

        fn pos(&self) -> usize {
            self.pos
        }
    }

    // Input setup
    let parser = TestParser {
        input: vec!['p'],
        pos: 0,
    };

    let result = parser.parse_unicode_class();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::EscapeUnexpectedEof);
    }
}

