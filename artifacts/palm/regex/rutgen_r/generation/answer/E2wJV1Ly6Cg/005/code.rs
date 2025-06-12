// Answer 0

#[test]
fn test_peek_space_ignore_whitespace_mode_no_eof() {
    struct TestParser {
        ignore_whitespace_flag: bool,
        eof_flag: bool,
        pattern_data: String,
        offset_data: usize,
        char_data: char,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace_flag
        }

        fn is_eof(&self) -> bool {
            self.eof_flag
        }

        fn pattern(&self) -> &str {
            &self.pattern_data
        }

        fn offset(&self) -> usize {
            self.offset_data
        }

        fn char(&self) -> char {
            self.char_data
        }

        fn peek(&self) -> Option<char> {
            Some(self.pattern()[self.offset()..].chars().next().unwrap_or('\0'))
        }
    }

    // Test case where there are whitespaces and comments to skip
    let parser = TestParser {
        ignore_whitespace_flag: true,
        eof_flag: false,
        pattern_data: String::from("    # This is a comment\n   a"),
        offset_data: 0,
        char_data: ' ',
    };

    assert_eq!(parser.peek_space(), Some('a'));
}

#[test]
fn test_peek_space_ignore_whitespace_mode_with_whitespace() {
    struct TestParser {
        ignore_whitespace_flag: bool,
        eof_flag: bool,
        pattern_data: String,
        offset_data: usize,
        char_data: char,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace_flag
        }

        fn is_eof(&self) -> bool {
            self.eof_flag
        }

        fn pattern(&self) -> &str {
            &self.pattern_data
        }

        fn offset(&self) -> usize {
            self.offset_data
        }

        fn char(&self) -> char {
            self.char_data
        }

        fn peek(&self) -> Option<char> {
            Some(self.pattern()[self.offset()..].chars().next().unwrap_or('\0'))
        }
    }

    // Test case where only whitespace exists and the parser is in comment
    let parser = TestParser {
        ignore_whitespace_flag: true,
        eof_flag: false,
        pattern_data: String::from("   # Comment line\n   "), 
        offset_data: 0,
        char_data: ' ',
    };

    assert_eq!(parser.peek_space(), None);
}

#[test]
fn test_peek_space_ignore_whitespace_mode_eof() {
    struct TestParser {
        ignore_whitespace_flag: bool,
        eof_flag: bool,
        pattern_data: String,
        offset_data: usize,
        char_data: char,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace_flag
        }

        fn is_eof(&self) -> bool {
            self.eof_flag
        }

        fn pattern(&self) -> &str {
            &self.pattern_data
        }

        fn offset(&self) -> usize {
            self.offset_data
        }

        fn char(&self) -> char {
            self.char_data
        }

        fn peek(&self) -> Option<char> {
            Some(self.pattern()[self.offset()..].chars().next().unwrap_or('\0'))
        }
    }

    // Test case with EOF condition
    let parser = TestParser {
        ignore_whitespace_flag: true,
        eof_flag: true,
        pattern_data: String::from(""), 
        offset_data: 0,
        char_data: ' ',
    };

    assert_eq!(parser.peek_space(), None);
}

#[test]
fn test_peek_space_inside_comment() {
    struct TestParser {
        ignore_whitespace_flag: bool,
        eof_flag: bool,
        pattern_data: String,
        offset_data: usize,
        char_data: char,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace_flag
        }

        fn is_eof(&self) -> bool {
            self.eof_flag
        }

        fn pattern(&self) -> &str {
            &self.pattern_data
        }

        fn offset(&self) -> usize {
            self.offset_data
        }

        fn char(&self) -> char {
            self.char_data
        }

        fn peek(&self) -> Option<char> {
            Some(self.pattern()[self.offset()..].chars().next().unwrap_or('\0'))
        }
    }

    // Test case where the parser points to a character inside a comment
    let parser = TestParser {
        ignore_whitespace_flag: true,
        eof_flag: false,
        pattern_data: String::from("   # Comment here\n# Another comment\nb"),
        offset_data: 0,
        char_data: ' ',
    };

    assert_eq!(parser.peek_space(), Some('b'));
}

