// Answer 0

#[derive(Debug)]
struct MockParser {
    pattern: String,
    offset: usize,
    ignore_whitespace: bool,
}

impl MockParser {
    fn new(pattern: &str, offset: usize, ignore_whitespace: bool) -> Self {
        Self {
            pattern: pattern.to_string(),
            offset,
            ignore_whitespace,
        }
    }

    fn peek_space(&self) -> Option<char> {
        // Implementation of the function you provided
        if !self.ignore_whitespace {
            return self.peek();
        }
        if self.is_eof() {
            return None;
        }
        let mut start = self.offset + self.char().len_utf8();
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
        self.pattern[start..].chars().next()
    }

    fn is_eof(&self) -> bool {
        self.offset >= self.pattern.len()
    }

    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn char(&self) -> char {
        self.pattern[self.offset..].chars().next().unwrap_or('\0')
    }

    fn peek(&self) -> Option<char> {
        self.pattern[self.offset..].chars().next()
    }
}

#[test]
fn test_peek_space_no_space() {
    let parser = MockParser::new("abc# comment\nxyz", 0, true);
    assert_eq!(parser.peek_space(), Some('x'));
}

#[test]
fn test_peek_space_with_comment() {
    let parser = MockParser::new("abc# comment\nxyz", 0, true);
    assert_eq!(parser.peek_space(), Some('x'));
}

#[test]
fn test_peek_space_end_of_file() {
    let parser = MockParser::new("abc# comment\n", 13, true);
    assert_eq!(parser.peek_space(), None);
}

#[test]
fn test_peek_space_leading_whitespace() {
    let parser = MockParser::new("   # comment\nabc", 0, true);
    assert_eq!(parser.peek_space(), Some('a'));
}

#[test]
fn test_peek_space_with_newline_in_comment() {
    let parser = MockParser::new("abc# comment\n# more\nxyz", 0, true);
    assert_eq!(parser.peek_space(), Some('x'));
}

