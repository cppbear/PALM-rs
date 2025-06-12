// Answer 0

#[test]
fn test_peek_space_ignore_whitespace_true_and_not_eof() {
    struct TestParser {
        pattern: String,
        offset: usize,
        eof: bool,
        ignore_whitespace: bool,
    }

    impl TestParser {
        fn peek_space(&self) -> Option<char> {
            if !self.ignore_whitespace {
                return None; // Simplified for testing
            }
            if self.eof {
                return None;
            }
            let mut start = self.offset;
            let mut in_comment = false;
            for (i, c) in self.pattern[start..].char_indices() {
                if c.is_whitespace() {
                    continue;
                } else if !in_comment && c == '#' {
                    in_comment = true;
                } else if in_comment && c == '\n' {
                    in_comment = false;
                } else {
                    start += i;
                    break;
                }
            }
            self.pattern.get(start..).and_then(|s| s.chars().next())
        }
    }

    let parser = TestParser {
        pattern: String::from("   # comment\n   a"),
        offset: 0,
        eof: false,
        ignore_whitespace: true,
    };
    assert_eq!(parser.peek_space(), Some('a'));

    let parser2 = TestParser {
        pattern: String::from("   # comment\n   "),
        offset: 0,
        eof: false,
        ignore_whitespace: true,
    };
    assert_eq!(parser2.peek_space(), None);
}

#[test]
#[should_panic]
fn test_peek_space_pattern_out_of_bounds() {
    struct TestParser {
        pattern: String,
        offset: usize,
        eof: bool,
        ignore_whitespace: bool,
    }

    impl TestParser {
        fn peek_space(&self) -> Option<char> {
            if !self.ignore_whitespace {
                return None; // Simplified for testing
            }
            if self.eof {
                return None;
            }
            let mut start = self.offset;
            let mut in_comment = false;
            for (i, c) in self.pattern[start..].char_indices() {
                if c.is_whitespace() {
                    continue;
                } else if !in_comment && c == '#' {
                    in_comment = true;
                } else if in_comment && c == '\n' {
                    in_comment = false;
                } else {
                    start += i;
                    break;
                }
            }
            self.pattern.get(start..).unwrap().chars().next()
        }
    }

    let parser = TestParser {
        pattern: String::from(""),
        offset: 1,
        eof: false,
        ignore_whitespace: true,
    };
    // This should panic due to out-of-bounds access
    let _ = parser.peek_space();
}

