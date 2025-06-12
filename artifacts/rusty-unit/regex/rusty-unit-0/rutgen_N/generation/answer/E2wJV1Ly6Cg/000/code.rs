// Answer 0

#[derive(Clone)]
struct Parser {
    pattern: String,
    offset: usize,
    ignore_whitespace: bool,
}

impl Parser {
    fn new(pattern: String, offset: usize, ignore_whitespace: bool) -> Self {
        Self {
            pattern,
            offset,
            ignore_whitespace,
        }
    }
    
    fn peek_space(&self) -> Option<char> {
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

    fn peek(&self) -> Option<char> {
        self.pattern.get(self.offset..)?.chars().next()
    }

    fn is_eof(&self) -> bool {
        self.offset >= self.pattern.len()
    }

    fn char(&self) -> char {
        self.pattern[self.offset..].chars().next().unwrap_or('\0')
    }
}

#[test]
fn test_peek_space_with_whitespace_insensitive_mode() {
    let parser = Parser::new(" foo # comment\n bar", 0, true);
    assert_eq!(parser.peek_space(), Some('b'));
}

#[test]
fn test_peek_space_with_whitespace_insensitive_mode_eof() {
    let parser = Parser::new(" # comment", 0, true);
    assert_eq!(parser.peek_space(), None);
}

#[test]
fn test_peek_space_with_non_whitespace_insensitive_mode() {
    let parser = Parser::new(" foo # comment\n bar", 0, false);
    assert_eq!(parser.peek_space(), Some('f'));
}

#[test]
fn test_peek_space_with_non_whitespace_insensitive_mode_eof() {
    let parser = Parser::new(" # comment", 0, false);
    assert_eq!(parser.peek_space(), Some(' '));
}

