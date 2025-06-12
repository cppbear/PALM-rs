// Answer 0

#[test]
fn test_peek_space_no_whitespace() {
    struct TestParser {
        pattern: String,
        current_offset: usize,
        should_ignore_whitespace: bool,
        end_of_file: bool,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            self.should_ignore_whitespace
        }

        fn is_eof(&self) -> bool {
            self.end_of_file
        }

        fn offset(&self) -> usize {
            self.current_offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn char(&self) -> char {
            // Return the character at the current offset; if out of bounds return some default
            self.pattern.chars().nth(self.current_offset).unwrap_or('\0')
        }

        fn peek(&self) -> Option<char> {
            self.pattern.chars().nth(self.current_offset)
        }
    }

    // Test input where there is no whitespace
    let parser = TestParser {
        pattern: "# comment\nabc".to_string(),
        current_offset: 0,
        should_ignore_whitespace: true,
        end_of_file: false,
    };

    assert_eq!(parser.peek_space(), Some('a'));
}

#[test]
fn test_peek_space_with_whitespace() {
    struct TestParser {
        pattern: String,
        current_offset: usize,
        should_ignore_whitespace: bool,
        end_of_file: bool,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            self.should_ignore_whitespace
        }

        fn is_eof(&self) -> bool {
            self.end_of_file
        }

        fn offset(&self) -> usize {
            self.current_offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.current_offset).unwrap_or('\0')
        }

        fn peek(&self) -> Option<char> {
            self.pattern.chars().nth(self.current_offset)
        }
    }

    // Test input including whitespace
    let parser = TestParser {
        pattern: "# comment\n   abc".to_string(),
        current_offset: 0,
        should_ignore_whitespace: true,
        end_of_file: false,
    };

    assert_eq!(parser.peek_space(), Some('a'));
}

#[test]
fn test_peek_space_only_comments() {
    struct TestParser {
        pattern: String,
        current_offset: usize,
        should_ignore_whitespace: bool,
        end_of_file: bool,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            self.should_ignore_whitespace
        }

        fn is_eof(&self) -> bool {
            self.end_of_file
        }

        fn offset(&self) -> usize {
            self.current_offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.current_offset).unwrap_or('\0')
        }

        fn peek(&self) -> Option<char> {
            self.pattern.chars().nth(self.current_offset)
        }
    }

    // Test input where the only content is a comment
    let parser = TestParser {
        pattern: "# comment\n".to_string(),
        current_offset: 0,
        should_ignore_whitespace: true,
        end_of_file: false,
    };

    assert_eq!(parser.peek_space(), None);
}

#[test]
fn test_peek_space_eof_reached() {
    struct TestParser {
        pattern: String,
        current_offset: usize,
        should_ignore_whitespace: bool,
        end_of_file: bool,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            self.should_ignore_whitespace
        }

        fn is_eof(&self) -> bool {
            self.end_of_file
        }

        fn offset(&self) -> usize {
            self.current_offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.current_offset).unwrap_or('\0')
        }

        fn peek(&self) -> Option<char> {
            self.pattern.chars().nth(self.current_offset)
        }
    }

    // Test input that reaches EOF without characters after ignoring whitespace
    let parser = TestParser {
        pattern: "# comment\n   ".to_string(),
        current_offset: 0,
        should_ignore_whitespace: true,
        end_of_file: true,
    };

    assert_eq!(parser.peek_space(), None);
}

