// Answer 0

#[derive(Debug)]
struct Parser {
    pattern_str: String,
    offset_value: usize,
    ignore_whitespace_flag: bool,
    eof_flag: bool,
}

impl Parser {
    fn new(pattern_str: &str, offset_value: usize, ignore_whitespace_flag: bool, eof_flag: bool) -> Self {
        Self {
            pattern_str: pattern_str.to_string(),
            offset_value,
            ignore_whitespace_flag,
            eof_flag,
        }
    }

    fn ignore_whitespace(&self) -> bool {
        self.ignore_whitespace_flag
    }

    fn is_eof(&self) -> bool {
        self.eof_flag
    }

    fn offset(&self) -> usize {
        self.offset_value
    }

    fn char(&self) -> char {
        self.pattern_str.chars().nth(self.offset()).unwrap_or('\0')
    }

    fn pattern(&self) -> &str {
        &self.pattern_str
    }

    fn peek_space(&self) -> Option<char> {
        if !self.ignore_whitespace() {
            return self.peek();
        }
        if self.is_eof() {
            return None;
        }
        let mut start = self.offset() + self.char().len_utf8();
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

    fn peek(&self) -> Option<char> {
        self.pattern().chars().nth(self.offset())
    }
}

#[test]
fn test_peek_space_eof_with_ignore_whitespace() {
    let parser = Parser::new("   # Comment \n   next", 0, true, true);
    assert_eq!(parser.peek_space(), None);
}

#[test]
fn test_peek_space_eof_with_ignore_whitespace_no_visible_chars() {
    let parser = Parser::new("      # Comment", 0, true, true);
    assert_eq!(parser.peek_space(), None);
}

