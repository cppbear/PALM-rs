// Answer 0

#[test]
fn test_peek_space_no_whitespace() {
    struct DummyParser {
        pos: Cell<Position>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl DummyParser {
        fn new(pos: Position, ignore_whitespace: bool, pattern: String) -> Self {
            DummyParser {
                pos: Cell::new(pos),
                ignore_whitespace: Cell::new(ignore_whitespace),
                pattern,
            }
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
        
        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.offset()..].chars().next().unwrap_or('\0')
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern[self.offset()..].chars().next()
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
                    break;
                }
            }
            self.pattern[start..].chars().next()
        }
    }

    let pos = Position { offset: 0 }; // Assuming Position has an offset field.
    let parser = DummyParser::new(pos, true, String::from("   # This is a comment\nbcd"));
    
    assert_eq!(parser.peek_space(), Some('b'));
}

#[test]
fn test_peek_space_with_whitespace() {
    struct DummyParser {
        pos: Cell<Position>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl DummyParser {
        fn new(pos: Position, ignore_whitespace: bool, pattern: String) -> Self {
            DummyParser {
                pos: Cell::new(pos),
                ignore_whitespace: Cell::new(ignore_whitespace),
                pattern,
            }
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
        
        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.offset()..].chars().next().unwrap_or('\0')
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern[self.offset()..].chars().next()
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
                    break;
                }
            }
            self.pattern[start..].chars().next()
        }
    }

    let pos = Position { offset: 8 }; // When offset is after whitespace and comment
    let parser = DummyParser::new(pos, true, String::from("   # This is a comment\nabc"));
    
    assert_eq!(parser.peek_space(), Some('a'));
}

#[test]
#[should_panic]
fn test_peek_space_eof() {
    struct DummyParser {
        pos: Cell<Position>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl DummyParser {
        fn new(pos: Position, ignore_whitespace: bool, pattern: String) -> Self {
            DummyParser {
                pos: Cell::new(pos),
                ignore_whitespace: Cell::new(ignore_whitespace),
                pattern,
            }
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
        
        fn is_eof(&self) -> bool {
            self.offset() >= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.offset()..].chars().next().unwrap_or('\0')
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
                    break;
                }
            }
            self.pattern[start..].chars().next()
        }
    }

    let pos = Position { offset: 11 }; // Offset beyond the pattern length.
    let parser = DummyParser::new(pos, true, String::from("   # This is a comment\n"));
    
    parser.peek_space(); // This should panic due to the invalid range access.
}

