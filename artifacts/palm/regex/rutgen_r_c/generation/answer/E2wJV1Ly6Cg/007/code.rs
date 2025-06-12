// Answer 0

#[test]
fn test_peek_space_no_whitespace() {
    struct MockParser {
        pos: Position,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position::new(),
                ignore_whitespace: Cell::new(true),
                pattern: pattern.to_string(),
            }
        }
        
        fn offset(&self) -> usize {
            self.pos.offset
        }
        
        fn is_eof(&self) -> bool {
            self.offset() >= self.pattern.len()
        }
        
        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.offset()).unwrap_or('\0')
        }
        
        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern.chars().nth(self.offset())
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
            for (i, c) in self.pattern[start..].char_indices() {
                if c.is_whitespace() {
                    continue;
                } else if !in_comment && c == '#' {
                    in_comment = true;
                } else if in_comment && c == '\n' {
                    in_comment = false;
                } else {
                    start += i;
                    return self.pattern.chars().nth(start);
                }
            }
            None
        }
    }

    let parser = MockParser::new("a b # comment\nc");
    assert_eq!(parser.peek_space(), Some('c'));
}

#[test]
fn test_peek_space_with_comment() {
    struct MockParser {
        pos: Position,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position::new(),
                ignore_whitespace: Cell::new(true),
                pattern: pattern.to_string(),
            }
        }
        
        fn offset(&self) -> usize {
            self.pos.offset
        }
        
        fn is_eof(&self) -> bool {
            self.offset() >= self.pattern.len()
        }
        
        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.offset()).unwrap_or('\0')
        }
        
        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern.chars().nth(self.offset())
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
            for (i, c) in self.pattern[start..].char_indices() {
                if c.is_whitespace() {
                    continue;
                } else if !in_comment && c == '#' {
                    in_comment = true;
                } else if in_comment && c == '\n' {
                    in_comment = false;
                } else {
                    start += i;
                    return self.pattern.chars().nth(start);
                }
            }
            None
        }
    }

    let parser = MockParser::new("a b # comment\n d");
    assert_eq!(parser.peek_space(), Some('d'));
}

#[test]
fn test_peek_space_end_of_pattern() {
    struct MockParser {
        pos: Position,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position::new(),
                ignore_whitespace: Cell::new(true),
                pattern: pattern.to_string(),
            }
        }
        
        fn offset(&self) -> usize {
            self.pos.offset
        }
        
        fn is_eof(&self) -> bool {
            self.offset() >= self.pattern.len()
        }
        
        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.offset()).unwrap_or('\0')
        }
        
        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern.chars().nth(self.offset())
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
            for (i, c) in self.pattern[start..].char_indices() {
                if c.is_whitespace() {
                    continue;
                } else if !in_comment && c == '#' {
                    in_comment = true;
                } else if in_comment && c == '\n' {
                    in_comment = false;
                } else {
                    start += i;
                    return self.pattern.chars().nth(start);
                }
            }
            None
        }
    }

    let parser = MockParser::new("a b # comment\n");
    assert_eq!(parser.peek_space(), None);
}

