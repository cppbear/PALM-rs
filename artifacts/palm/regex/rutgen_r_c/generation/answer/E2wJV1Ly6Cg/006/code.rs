// Answer 0

#[test]
fn test_peek_space_with_ignore_whitespace() {
    struct MockAstError;
    struct MockPosition {
        offset: usize,
    }
    
    struct MockParser {
        pos: Cell<MockPosition>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }
    
    impl MockParser {
        fn new(offset: usize, ignore_whitespace: bool, pattern: &str) -> Self {
            Self {
                pos: Cell::new(MockPosition { offset }),
                ignore_whitespace: Cell::new(ignore_whitespace),
                pattern: pattern.to_string(),
            }
        }
        
        fn is_eof(&self) -> bool {
            self.pos.get().offset >= self.pattern.len()
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern[self.pos.get().offset..].chars().next()
        }

        fn char(&self) -> char {
            self.peek().unwrap_or(' ')
        }
        
        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
        
        fn bump_space(&mut self) {
            self.pos.set(MockPosition { offset: self.pos.get().offset + 1 });
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.bump_space();
                true
            } else {
                false
            }
        }
    }

    impl ParserI<'static, &MockParser> {
        fn new(parser: &MockParser, pattern: &'static str) -> ParserI<'static, &MockParser> {
            ParserI { parser, pattern }
        }
    }

    // Test Initialization
    let mock_parser = MockParser::new(0, true, "    # comment\n  a b c");
    let parser_i = ParserI::new(&mock_parser, "    # comment\n  a b c");

    // Test when `peek_space` is called
    let result = parser_i.peek_space();
    assert_eq!(result, Some('a'));
}

#[test]
fn test_peek_space_no_whitespace_to_ignore() {
    struct MockAstError;
    struct MockPosition {
        offset: usize,
    }
    
    struct MockParser {
        pos: Cell<MockPosition>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }
    
    impl MockParser {
        fn new(offset: usize, ignore_whitespace: bool, pattern: &str) -> Self {
            Self {
                pos: Cell::new(MockPosition { offset }),
                ignore_whitespace: Cell::new(ignore_whitespace),
                pattern: pattern.to_string(),
            }
        }
        
        fn is_eof(&self) -> bool {
            self.pos.get().offset >= self.pattern.len()
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                return None;
            }
            self.pattern[self.pos.get().offset..].chars().next()
        }

        fn char(&self) -> char {
            self.peek().unwrap_or(' ')
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }

        fn bump_space(&mut self) {
            self.pos.set(MockPosition { offset: self.pos.get().offset + 1 });
        }

        fn bump(&mut self) -> bool {
            if !self.is_eof() {
                self.bump_space();
                true
            } else {
                false
            }
        }
    }

    impl ParserI<'static, &MockParser> {
        fn new(parser: &MockParser, pattern: &'static str) -> ParserI<'static, &MockParser> {
            ParserI { parser, pattern }
        }
    }

    // Test Initialization
    let mock_parser = MockParser::new(0, true, "abc");
    let parser_i = ParserI::new(&mock_parser, "abc");

    // Test when `peek_space` is called
    let result = parser_i.peek_space();
    assert_eq!(result, Some('a'));
}

