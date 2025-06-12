// Answer 0

fn test_peek_space_ignore_whitespace() {
    struct MockParser {
        pattern: String,
        offset: usize,
        ignore_whitespace: bool,
    }

    impl MockParser {
        fn new(pattern: String, offset: usize, ignore_whitespace: bool) -> Self {
            MockParser {
                pattern,
                offset,
                ignore_whitespace,
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn is_eof(&self) -> bool {
            false // Constraint: is_eof is false
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn char(&self) -> char {
            // Return the first character of the pattern for testing
            self.pattern.chars().next().unwrap_or('\0')
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn peek_space(&self) -> Option<char> {
            if !self.ignore_whitespace() {
                return None; // Return None if whitespace is not ignored
            }
            if self.is_eof() {
                return None;
            }
            let mut start = self.offset + self.char().len_utf8();
            let mut in_comment = false;
            for (i, c) in self.pattern()[start..].char_indices() {
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
            self.pattern()[start..].chars().next()
        }
    }

    let parser = MockParser::new("foo # this is a comment\nbar".to_string(), 0, true);
    assert_eq!(parser.peek_space(), Some('b'));

    let parser2 = MockParser::new("foo # comment\nbaz".to_string(), 0, true);
    assert_eq!(parser2.peek_space(), Some('b'));

    let parser3 = MockParser::new("   # comment\n   foo".to_string(), 0, true);
    assert_eq!(parser3.peek_space(), Some('f'));

    let parser4 = MockParser::new("   \n# comment\n   bar".to_string(), 0, true);
    assert_eq!(parser4.peek_space(), Some('b'));
}

fn test_peek_space_no_whitespace() {
    struct MockParser {
        pattern: String,
        offset: usize,
        ignore_whitespace: bool,
    }

    impl MockParser {
        fn new(pattern: String, offset: usize, ignore_whitespace: bool) -> Self {
            MockParser {
                pattern,
                offset,
                ignore_whitespace,
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn is_eof(&self) -> bool {
            false
        }

        fn offset(&self) -> usize {
            self.offset
        }
        
        fn char(&self) -> char {
            self.pattern.chars().next().unwrap_or('\0')
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn peek_space(&self) -> Option<char> {
            if !self.ignore_whitespace() {
                return None;
            }
            if self.is_eof() {
                return None;
            }
            let mut start = self.offset + self.char().len_utf8();
            let mut in_comment = false;
            for (i, c) in self.pattern()[start..].char_indices() {
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
            self.pattern()[start..].chars().next()
        }
    }

    let parser = MockParser::new("foo#bar".to_string(), 0, true);
    assert_eq!(parser.peek_space(), Some('b'));
}

