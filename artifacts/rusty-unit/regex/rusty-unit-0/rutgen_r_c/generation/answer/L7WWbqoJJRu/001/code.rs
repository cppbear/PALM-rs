// Answer 0

#[test]
fn test_bump_and_bump_space_not_eof() {
    struct DummyParser {
        pos: Position,
        pattern: String,
        offset: usize,
    }
    
    impl DummyParser {
        fn new(pattern: &str) -> Self {
            DummyParser {
                pos: Position { offset: 0, line: 0, column: 0 },
                pattern: pattern.to_string(),
                offset: 0,
            }
        }
        
        fn bump(&mut self) -> bool {
            if self.offset < self.pattern.len() {
                self.offset += self.pattern[self.offset..].chars().next().unwrap().len_utf8();
                self.pos.offset = self.offset;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.offset..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.pattern.len()
        }

        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn bump_space(&mut self) {
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
        }
    }
    
    let mut parser = DummyParser::new("abc  \n   def");
    
    let result = parser.bump_and_bump_space();
    
    assert!(result);
    assert!(!parser.is_eof());
}

#[test]
fn test_bump_and_bump_space_reaching_eof() {
    struct DummyParser {
        pos: Position,
        pattern: String,
        offset: usize,
    }
    
    impl DummyParser {
        fn new(pattern: &str) -> Self {
            DummyParser {
                pos: Position { offset: 0, line: 0, column: 0 },
                pattern: pattern.to_string(),
                offset: 0,
            }
        }
        
        fn bump(&mut self) -> bool {
            if self.offset < self.pattern.len() {
                self.offset += self.pattern[self.offset..].chars().next().unwrap().len_utf8();
                self.pos.offset = self.offset;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.offset..].chars().next().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.offset >= self.pattern.len()
        }

        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn bump_space(&mut self) {
            while !self.is_eof() && self.char().is_whitespace() {
                self.bump();
            }
        }
    }
    
    let mut parser = DummyParser::new("   \n  ");
    
    // Bump through spaces until the end
    while parser.bump() { }

    let result = parser.bump_and_bump_space();

    assert!(!result);
    assert!(parser.is_eof());
}

