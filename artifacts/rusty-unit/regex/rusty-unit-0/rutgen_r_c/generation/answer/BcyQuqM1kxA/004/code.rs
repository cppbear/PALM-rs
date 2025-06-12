// Answer 0

#[test]
fn test_parse_flag_swap_greed() {
    struct TestParser {
        current_char: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: Position, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::new(),
                span: Span { start: 0, end: 0 },
            }
        }

        fn span_char(&self) -> Position {
            0
        }
    }

    let parser = TestParser { current_char: 'U' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::SwapGreed));
}

