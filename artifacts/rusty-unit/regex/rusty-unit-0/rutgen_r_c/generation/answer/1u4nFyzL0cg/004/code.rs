// Answer 0

#[test]
fn test_parse_set_class_with_valid_intersection() {
    struct MockParser<'a> {
        char: char,
        pattern: &'a str,
        pos: usize,
        eof: bool,
    }

    impl<'a> MockParser<'a> {
        fn char(&self) -> char {
            self.char
        }

        fn bump_space(&mut self) {}

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn peek(&self) -> Option<char> {
            if self.pos + 1 < self.pattern.len() {
                Some(self.pattern[self.pos + 1] as char)
            } else {
                None
            }
        }

        fn bump_if(&mut self, _s: &str) -> bool {
            if self.char == '~' {
                self.pos += 1; // Move the position forward
                true
            } else {
                false
            }
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::UnclosedClass,
                pattern: self.pattern.to_string(),
                span: ast::Span { start: 0, end: self.pos as u32 },
            }
        }
    }

    let mut parser = MockParser {
        char: '[',
        pattern: "[a-z~]",
        pos: 0,
        eof: false,
    };

    // Simulate the parsing
    // This should invoke the error handling for unclosed classes
    parser.char = '~'; // Simulate encountering '~'
    assert_eq!(parser.is_eof(), false);
    assert_eq!(parser.char(), '~');
    let result = parser.bump_if("~~");
    assert_eq!(result, true);
    parser.eof = true; // Set to true to trigger the unclosed error
    assert_eq!(parser.unclosed_class_error().kind, ast::ErrorKind::UnclosedClass);
}

#[test]
#[should_panic]
fn test_parse_set_class_with_unclosed_class() {
    struct MockParser<'a> {
        char: char,
        pattern: &'a str,
        pos: usize,
        eof: bool,
    }

    impl<'a> MockParser<'a> {
        fn char(&self) -> char {
            self.char
        }

        fn bump_space(&mut self) {}

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn peek(&self) -> Option<char> {
            if self.pos + 1 < self.pattern.len() {
                Some(self.pattern[self.pos + 1] as char)
            } else {
                None
            }
        }

        fn bump_if(&mut self, _s: &str) -> bool {
            if self.char == '~' {
                self.pos += 1; // Move the position forward
                true
            } else {
                false
            }
        }

        fn unclosed_class_error(&self) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::UnclosedClass,
                pattern: self.pattern.to_string(),
                span: ast::Span { start: 0, end: self.pos as u32 },
            }
        }
    }

    let mut parser = MockParser {
        char: '[',
        pattern: "[a-z~",
        pos: 0,
        eof: false,
    };

    // This will now simulate an unclosed class, leading to a panic
    assert_eq!(parser.is_eof(), false);
    parser.eof = true; // Set to true to trigger the unclosed error
    let result = parser.unclosed_class_error();
    assert_eq!(result.kind, ast::ErrorKind::UnclosedClass);
}

