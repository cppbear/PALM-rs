// Answer 0

#[test]
fn test_parse_flag_case_insensitive() {
    struct TestParser {
        current_char: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }

        fn span_char(&self) -> usize {
            0
        }
    }
    
    let parser = TestParser { current_char: 'i' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::CaseInsensitive));
}

#[test]
fn test_parse_flag_multi_line() {
    struct TestParser {
        current_char: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }

        fn span_char(&self) -> usize {
            0
        }
    }
    
    let parser = TestParser { current_char: 'm' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::MultiLine));
}

#[test]
fn test_parse_flag_dot_matches_new_line() {
    struct TestParser {
        current_char: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }

        fn span_char(&self) -> usize {
            0
        }
    }
    
    let parser = TestParser { current_char: 's' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::DotMatchesNewLine));
}

#[test]
fn test_parse_flag_swap_greed() {
    struct TestParser {
        current_char: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }

        fn span_char(&self) -> usize {
            0
        }
    }
    
    let parser = TestParser { current_char: 'U' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::SwapGreed));
}

#[test]
fn test_parse_flag_unicode() {
    struct TestParser {
        current_char: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }

        fn span_char(&self) -> usize {
            0
        }
    }
    
    let parser = TestParser { current_char: 'u' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::Unicode));
}

#[test]
fn test_parse_flag_ignore_whitespace() {
    struct TestParser {
        current_char: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }

        fn span_char(&self) -> usize {
            0
        }
    }
    
    let parser = TestParser { current_char: 'x' };
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::IgnoreWhitespace));
}

#[test]
#[should_panic(expected = "Unrecognized flag")]
fn test_parse_flag_unrecognized() {
    struct TestParser {
        current_char: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }

        fn span_char(&self) -> usize {
            0
        }
    }
    
    let parser = TestParser { current_char: 'z' };
    let _ = parser.parse_flag();
}

