// Answer 0

#[test]
fn test_parse_flag_case_insensitive() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }
        
        fn error(&self, _span: Position, _kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::from("Unrecognized flag"),
                span: Span { start: 0, end: 1 },
            })
        }
        
        fn span_char(&self) -> Position {
            0 // mock position
        }
    }

    let parser = MockParser { current_char: 'i' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::CaseInsensitive));
}

#[test]
fn test_parse_flag_multi_line() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: Position, _kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::from("Unrecognized flag"),
                span: Span { start: 0, end: 1 },
            })
        }

        fn span_char(&self) -> Position {
            0 
        }
    }

    let parser = MockParser { current_char: 'm' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::MultiLine));
}

#[test]
fn test_parse_flag_dot_matches_new_line() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: Position, _kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::from("Unrecognized flag"),
                span: Span { start: 0, end: 1 },
            })
        }

        fn span_char(&self) -> Position {
            0 
        }
    }

    let parser = MockParser { current_char: 's' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::DotMatchesNewLine));
}

#[test]
fn test_parse_flag_swap_greed() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: Position, _kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::from("Unrecognized flag"),
                span: Span { start: 0, end: 1 },
            })
        }

        fn span_char(&self) -> Position {
            0 
        }
    }

    let parser = MockParser { current_char: 'U' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::SwapGreed));
}

#[test]
fn test_parse_flag_unicode() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: Position, _kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::from("Unrecognized flag"),
                span: Span { start: 0, end: 1 },
            })
        }

        fn span_char(&self) -> Position {
            0 
        }
    }

    let parser = MockParser { current_char: 'u' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::Unicode));
}

#[test]
fn test_parse_flag_ignore_whitespace() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: Position, _kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::from("Unrecognized flag"),
                span: Span { start: 0, end: 1 },
            })
        }

        fn span_char(&self) -> Position {
            0 
        }
    }

    let parser = MockParser { current_char: 'x' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::IgnoreWhitespace));
}

#[test]
fn test_parse_flag_unrecognized() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: Position, _kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized,
                pattern: String::from("Unrecognized flag"),
                span: Span { start: 0, end: 1 },
            })
        }

        fn span_char(&self) -> Position {
            0 
        }
    }

    let parser = MockParser { current_char: 'z' };
    let result = parser.parse_flag();
    assert!(result.is_err());
}

