// Answer 0

#[test]
fn test_maybe_parse_ascii_class_invalid_start_no_colon() {
    struct TestParser {
        pos: usize,
        chars: Vec<char>,
    }
    
    impl TestParser {
        fn new(chars: Vec<char>) -> Self {
            Self { pos: 0, chars }
        }
        
        fn char(&self) -> char {
            self.chars[self.pos]
        }
        
        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos < self.chars.len()
        }
        
        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }
        
        fn pos(&self) -> usize {
            self.pos
        }
        
        fn offset(&self) -> usize {
            self.pos
        }
        
        fn pattern(&self) -> &[char] {
            &self.chars
        }
        
        fn bump_if(&mut self, _: &str) -> bool {
            false
        }
        
        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser::new(vec!['[', 'a', 'b', 'c']);
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_invalid_no_closing() {
    struct TestParser {
        pos: usize,
        chars: Vec<char>,
    }

    impl TestParser {
        fn new(chars: Vec<char>) -> Self {
            Self { pos: 0, chars }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos < self.chars.len()
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn offset(&self) -> usize {
            self.pos
        }
        
        fn pattern(&self) -> &[char] {
            &self.chars
        }
        
        fn bump_if(&mut self, _: &str) -> bool {
            false
        }
        
        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser::new(vec!['[', ':', 'a', 'b', 'c', 'd', 'e']);
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_invalid_double_colon() {
    struct TestParser {
        pos: usize,
        chars: Vec<char>,
    }

    impl TestParser {
        fn new(chars: Vec<char>) -> Self {
            Self { pos: 0, chars }
        }

        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos < self.chars.len()
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn offset(&self) -> usize {
            self.pos
        }
        
        fn pattern(&self) -> &[char] {
            &self.chars
        }
        
        fn bump_if(&mut self, _: &str) -> bool {
            true
        }
        
        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser::new(vec!['[', ':', ':', 'a', 'b', 'c', ']', ':']);
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

