// Answer 0

fn test_parse_flag_case_insensitive() {
    struct TestParser {
        char_val: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char_val
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(kind)
        }

        fn span_char(&self) -> ast::Span {
            ast::Span { start: 0, end: 1 }
        }
    }

    let parser = TestParser { char_val: 'i' };
    let result = parser.parse_flag();
    assert!(result.is_ok() && result.unwrap() == ast::Flag::CaseInsensitive);
}

fn test_parse_flag_multi_line() {
    struct TestParser {
        char_val: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char_val
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(kind)
        }

        fn span_char(&self) -> ast::Span {
            ast::Span { start: 0, end: 1 }
        }
    }

    let parser = TestParser { char_val: 'm' };
    let result = parser.parse_flag();
    assert!(result.is_ok() && result.unwrap() == ast::Flag::MultiLine);
}

fn test_parse_flag_dot_matches_new_line() {
    struct TestParser {
        char_val: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char_val
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(kind)
        }

        fn span_char(&self) -> ast::Span {
            ast::Span { start: 0, end: 1 }
        }
    }

    let parser = TestParser { char_val: 's' };
    let result = parser.parse_flag();
    assert!(result.is_ok() && result.unwrap() == ast::Flag::DotMatchesNewLine);
}

fn test_parse_flag_swap_greed() {
    struct TestParser {
        char_val: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char_val
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(kind)
        }

        fn span_char(&self) -> ast::Span {
            ast::Span { start: 0, end: 1 }
        }
    }

    let parser = TestParser { char_val: 'U' };
    let result = parser.parse_flag();
    assert!(result.is_ok() && result.unwrap() == ast::Flag::SwapGreed);
}

fn test_parse_flag_unicode() {
    struct TestParser {
        char_val: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char_val
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(kind)
        }

        fn span_char(&self) -> ast::Span {
            ast::Span { start: 0, end: 1 }
        }
    }

    let parser = TestParser { char_val: 'u' };
    let result = parser.parse_flag();
    assert!(result.is_ok() && result.unwrap() == ast::Flag::Unicode);
}

fn test_parse_flag_ignore_whitespace() {
    struct TestParser {
        char_val: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char_val
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(kind)
        }

        fn span_char(&self) -> ast::Span {
            ast::Span { start: 0, end: 1 }
        }
    }

    let parser = TestParser { char_val: 'x' };
    let result = parser.parse_flag();
    assert!(result.is_ok() && result.unwrap() == ast::Flag::IgnoreWhitespace);
}

fn test_parse_flag_unrecognized() {
    struct TestParser {
        char_val: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.char_val
        }

        fn error(&self, _span: ast::Span, kind: ast::ErrorKind) -> Result<ast::Flag> {
            Err(kind)
        }

        fn span_char(&self) -> ast::Span {
            ast::Span { start: 0, end: 1 }
        }
    }

    let parser = TestParser { char_val: '#' };
    let result = parser.parse_flag();
    assert!(result.is_err() && result.unwrap_err() == ast::ErrorKind::FlagUnrecognized);
}

