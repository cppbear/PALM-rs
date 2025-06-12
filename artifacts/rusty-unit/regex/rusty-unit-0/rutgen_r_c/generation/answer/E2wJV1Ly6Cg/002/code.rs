// Answer 0

#[test]
fn test_peek_space_when_ignore_whitespace_and_not_eof() {
    struct TestParser {
        pos: Position,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }
    
    impl TestParser {
        fn new(pos: Position, ignore_whitespace: bool, pattern: String) -> Self {
            Self {
                pos,
                ignore_whitespace: Cell::new(ignore_whitespace),
                pattern,
            }
        }
        
        fn offset(&self) -> usize {
            self.pos.offset
        }
        
        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }
        
        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.offset()).unwrap_or('\0')
        }
        
        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern.chars().nth(self.offset())
            }
        }
        
        fn char_indices(&self, start: usize) -> impl Iterator<Item=(usize, char)> {
            self.pattern[start..].char_indices()
        }
    }

    let initial_position = Position { offset: 0 };
    let pattern = "# this is a comment\n  a  b  c";
    let parser = TestParser::new(initial_position, true, pattern.to_string());
    
    let result = {
        let mut start = parser.offset() + parser.char().len_utf8();
        let mut in_comment = false;
        for (i, c) in parser.char_indices(start) {
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
        parser.pattern.chars().nth(start)
    };

    assert_eq!(result, Some('a'));
}

#[test]
fn test_peek_space_ignore_whitespace_with_no_whitespace() {
    struct TestParser {
        pos: Position,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl TestParser {
        fn new(pos: Position, ignore_whitespace: bool, pattern: String) -> Self {
            Self {
                pos,
                ignore_whitespace: Cell::new(ignore_whitespace),
                pattern,
            }
        }
        
        fn offset(&self) -> usize {
            self.pos.offset
        }
        
        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }
        
        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace.get()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.offset()).unwrap_or('\0')
        }
        
        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern.chars().nth(self.offset())
            }
        }
        
        fn char_indices(&self, start: usize) -> impl Iterator<Item=(usize, char)> {
            self.pattern[start..].char_indices()
        }
    }

    let initial_position = Position { offset: 0 };
    let pattern = "ab";
    let parser = TestParser::new(initial_position, true, pattern.to_string());
    
    let result = {
        let mut start = parser.offset() + parser.char().len_utf8();
        let mut in_comment = false;
        for (i, c) in parser.char_indices(start) {
            if c.is_whitespace() {
                continue;
            } else {
                start += i;
                break;
            }
        }
        parser.pattern.chars().nth(start)
    };

    assert_eq!(result, Some('a'));
}

