// Answer 0

#[test]
fn test_parse_flag_ignore_whitespace() {
    struct MockParser {
        char_to_return: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.char_to_return
        }

        fn error(&self, _span: ast::Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::from(""),
                span: ast::Span { start: 0, end: 1 },
            }
        }

        fn span_char(&self) -> ast::Span {
            ast::Span { start: 0, end: 1 }
        }
    }

    let parser = MockParser { char_to_return: 'x' };
    let result = parser.parse_flag();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ast::Flag::IgnoreWhitespace);
}

