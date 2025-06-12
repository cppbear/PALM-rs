// Answer 0

#[derive(Debug)]
struct Parser {
    pattern: String,
    offset: usize,
    ignore_whitespace: bool,
    eof: bool,
}

impl Parser {
    fn new(pattern: String, offset: usize, ignore_whitespace: bool, eof: bool) -> Self {
        Self {
            pattern,
            offset,
            ignore_whitespace,
            eof,
        }
    }

    fn ignore_whitespace(&self) -> bool {
        self.ignore_whitespace
    }

    fn is_eof(&self) -> bool {
        self.eof
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn char(&self) -> char {
        self.pattern.chars().nth(self.offset).unwrap_or('\0')
    }

    fn pattern(&self) -> &str {
        &self.pattern
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
        self.pattern.get(self.offset()).and_then(|c| c.chars().next())
    }
}

#[test]
fn test_peek_space_no_whitespace() {
    let parser = Parser::new("abc#comment\ndef".to_string(), 0, true, false);
    assert_eq!(parser.peek_space(), Some('a')); // 'a' is the first non-whitespace char
}

#[test]
fn test_peek_space_with_whitespace() {
    let parser = Parser::new("   #comment\nb".to_string(), 0, true, false);
    assert_eq!(parser.peek_space(), Some('b')); // 'b' is the first non-whitespace char after whitespace
}

#[test]
fn test_peek_space_end_of_file() {
    let parser = Parser::new("abc#comment\n".to_string(), 8, true, true);
    assert_eq!(parser.peek_space(), None); // EOF condition, should return None
}

#[test]
fn test_peek_space_inside_comment() {
    let parser = Parser::new("abc#comment\n #   d".to_string(), 0, true, false);
    assert_eq!(parser.peek_space(), Some('d')); // 'd' is the first non-whitespace char after comment
}

