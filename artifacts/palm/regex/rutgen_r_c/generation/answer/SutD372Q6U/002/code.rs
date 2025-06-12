// Answer 0

#[test]
fn test_parse_group_capture_name_error() {
    struct MockParser {
        current_char: char,
        capture_index: u32,
        eof: bool,
        bump_called: bool,
        pattern: String,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                current_char: '(',
                capture_index: 0,
                eof: false,
                bump_called: false,
                pattern: String::from("(?P<name>)"),
            }
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump(&mut self) {
            self.bump_called = true;
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 })
        }

        fn bump_if(&mut self, pattern: &str) -> bool {
            self.current_char == '(' && pattern == "?P<"
        }

        fn next_capture_index(&self, _open_span: Span) -> Result<u32> {
            Err(ast::Error {
                kind: ast::ErrorKind::GroupNameEmpty,
                pattern: self.pattern.clone(),
                span: self.span(),
            })
        }

        fn is_lookaround_prefix(&self) -> bool {
            false
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span: self.span() }
        }
    }

    let parser = MockParser::new();
    let result = parser.parse_group();

    match result {
        Err(err) => {
            assert_eq!(err.kind, ast::ErrorKind::GroupNameEmpty);
        },
        _ => panic!("Expected an error but got a valid result"),
    }
}

#[test]
fn test_parse_group_non_capturing_empty_flags() {
    struct MockParser {
        current_char: char,
        capture_index: u32,
        eof: bool,
        pattern: String,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                current_char: '(',
                capture_index: 0,
                eof: false,
                pattern: String::from("(?)"),
            }
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump(&mut self) {}

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn span(&self) -> Span {
            Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 0, line: 1, column: 1 })
        }

        fn bump_if(&mut self, pattern: &str) -> bool {
            self.current_char == '(' && pattern == "?"
        }

        fn next_capture_index(&self, _open_span: Span) -> Result<u32> {
            Ok(self.capture_index)
        }

        fn is_lookaround_prefix(&self) -> bool {
            false
        }

        fn error(&self, _span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error { kind, pattern: self.pattern.clone(), span: self.span() }
        }

        fn parse_flags(&self) -> Result<ast::Flags> {
            let flags = ast::Flags {
                span: self.span(),
                items: Vec::new(),
            };
            Ok(flags)
        }
    }

    let parser = MockParser::new();
    let result = parser.parse_group();

    match result {
        Err(err) => {
            assert_eq!(err.kind, ast::ErrorKind::RepetitionMissing);
        },
        _ => panic!("Expected an error but got a valid result"),
    }
}

