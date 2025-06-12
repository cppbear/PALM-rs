// Answer 0

#[test]
fn test_parse_flag_case_insensitive() {
    struct DummyParser {
        input: Vec<char>,
        index: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input[self.index]
        }

        fn span_char(&self) -> char {
            self.char()
        }

        fn error(&self, _span_char: char, _error_kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }
    }

    let parser = DummyParser {
        input: vec!['i'],
        index: 0,
    };
    
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::CaseInsensitive));
}

#[test]
fn test_parse_flag_multi_line() {
    struct DummyParser {
        input: Vec<char>,
        index: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input[self.index]
        }

        fn span_char(&self) -> char {
            self.char()
        }

        fn error(&self, _span_char: char, _error_kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }
    }

    let parser = DummyParser {
        input: vec!['m'],
        index: 0,
    };
    
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::MultiLine));
}

#[test]
fn test_parse_flag_dot_matches_new_line() {
    struct DummyParser {
        input: Vec<char>,
        index: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input[self.index]
        }

        fn span_char(&self) -> char {
            self.char()
        }

        fn error(&self, _span_char: char, _error_kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }
    }

    let parser = DummyParser {
        input: vec!['s'],
        index: 0,
    };
    
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::DotMatchesNewLine));
}

#[test]
fn test_parse_flag_unrecognized() {
    struct DummyParser {
        input: Vec<char>,
        index: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input[self.index]
        }

        fn span_char(&self) -> char {
            self.char()
        }

        fn error(&self, _span_char: char, _error_kind: ast::ErrorKind) -> ast::Error {
            ast::Error::new("Unrecognized flag")
        }
    }

    let parser = DummyParser {
        input: vec!['z'],
        index: 0,
    };
    
    let result = parser.parse_flag();
    assert!(result.is_err());
}

