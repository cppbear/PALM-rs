// Answer 0

#[test]
fn test_peek_space_with_no_whitespace_enabled() {
    struct TestParser {
        pattern_data: String,
        current_offset: usize,
        ignore_whitespace: bool,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern_data: pattern.to_string(),
                current_offset: 0,
                ignore_whitespace: false,
            }
        }

        fn peek(&self) -> Option<char> {
            self.pattern_data.chars().nth(self.current_offset)
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn is_eof(&self) -> bool {
            self.current_offset >= self.pattern_data.len()
        }

        fn offset(&self) -> usize {
            self.current_offset
        }

        fn char(&self) -> char {
            self.peek().unwrap_or('\0')
        }

        fn pattern(&self) -> &str {
            &self.pattern_data
        }

        fn set_offset(&mut self, new_offset: usize) {
            self.current_offset = new_offset;
        }
    }

    let mut parser = TestParser::new("abc # comment\nxyz");
    
    // Before any whitespace, should return 'a'
    assert_eq!(parser.peek_space(), Some('a'));
    
    // Move past 'a' and check for 'b'
    parser.set_offset(1);
    assert_eq!(parser.peek_space(), Some('b'));
    
    // Move past 'b' and check for 'c'
    parser.set_offset(2);
    assert_eq!(parser.peek_space(), Some('c'));
    
    // Move past 'c', inside a comment, should skip to 'x'
    parser.set_offset(5);
    assert_eq!(parser.peek_space(), Some('x'));
    
    // Until end of input
    parser.set_offset(11); // after the 'z'
    assert_eq!(parser.peek_space(), None);
}

