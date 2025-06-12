// Answer 0


#[test]
fn test_parse_flags_no_flags() {
    struct TestParser {
        input: &'static str,
        position: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or(':') // Simulating history up to ':'.
        }

        fn span_char(&self) -> ast::Span {
            ast::Span {
                start: ast::Position { offset: self.position, line: 1, column: 1 },
                end: ast::Position { offset: self.position, line: 1, column: 1 },
            }
        }

        fn bump(&mut self) -> bool {
            self.position += 1; // Move to next character.
            self.position < self.input.len()
        }

        fn error(&self, _span: ast::Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnexpectedEof, // Simulated error.
                pattern: self.input.to_string(),
                span: self.span_char(),
            }
        }
    }

    let mut parser = TestParser { input: "--::", position: 0 };
    let result = parser.parse_flags(); // Testing case with no valid flags.
    assert!(result.is_err());
}

#[test]
fn test_parse_flags_repeated_negation() {
    struct TestParser {
        input: &'static str,
        position: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or(':') // Ending in ':' to fulfill constraints.
        }

        fn span_char(&self) -> ast::Span {
            ast::Span {
                start: ast::Position { offset: self.position, line: 1, column: 1 },
                end: ast::Position { offset: self.position, line: 1, column: 1 },
            }
        }

        fn bump(&mut self) -> bool {
            self.position += 1; // Move to next character.
            self.position < self.input.len()
        }

        fn error(&self, _span: ast::Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagDanglingNegation, // Simulated error.
                pattern: self.input.to_string(),
                span: self.span_char(),
            }
        }
    }

    let mut parser = TestParser { input: "--::", position: 0 };
    let result = parser.parse_flags(); // Testing case with repeated negation.
    assert!(result.is_err());
}

#[test]
fn test_parse_flags_invalid_flag() {
    struct TestParser {
        input: &'static str,
        position: usize,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input[self.position..].chars().next().unwrap_or(':') // Ending in ':' to fulfill constraints.
        }

        fn span_char(&self) -> ast::Span {
            ast::Span {
                start: ast::Position { offset: self.position, line: 1, column: 1 },
                end: ast::Position { offset: self.position, line: 1, column: 1 },
            }
        }

        fn bump(&mut self) -> bool {
            self.position += 1; // Move to next character.
            self.position < self.input.len()
        }

        fn parse_flag(&self) -> Result<ast::Flag> {
            Err(self.error(self.span_char(), ast::ErrorKind::FlagUnrecognized)) // Always generating an error for invalid flags
        }

        fn error(&self, _span: ast::Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::FlagUnrecognized, // Simulated unrecognized flag error.
                pattern: self.input.to_string(),
                span: self.span_char(),
            }
        }
    }

    let mut parser = TestParser { input: "-z::", position: 0 }; 
    let result = parser.parse_flags(); // Testing case with unrecognized flags.
    assert!(result.is_err());
}


