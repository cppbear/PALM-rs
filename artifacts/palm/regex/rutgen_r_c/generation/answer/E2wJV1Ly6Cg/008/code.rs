// Answer 0

#[test]
fn test_peek_space_whitespace_insensitive_mode_non_eof() {
    struct MockParser {
        pos: Position,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }
    
    impl MockParser {
        fn new() -> Self {
            Self {
                pos: Position { offset: 0 }, // Initialize to a valid position
                ignore_whitespace: Cell::new(true), // Constraint: whitespace ignored
                pattern: String::from("  #comment\n charA"), // Setup pattern
            }
        }
        
        fn offset(&self) -> usize {
            self.pos.offset
        }
        
        fn is_eof(&self) -> bool {
            self.offset() >= self.pattern.len() // Not EOF if offset less than pattern length
        }
        
        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern[self.offset()..].chars().next()
            }
        }
        
        fn chars(&self) -> impl Iterator<Item = (usize, char)> {
            self.pattern[self.offset()..].char_indices()
        }
    }
    
    let parser = MockParser::new();
    
    // Test the peek_space behavior
    let result = parser.peek_space();
    
    // Check that the first non-whitespace character is returned
    assert_eq!(result, Some('A')); // Expect 'A' right after the comment
}

#[test]
fn test_peek_space_ignore_comments() {
    struct MockParser {
        pos: Position,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                pos: Position { offset: 0 },
                ignore_whitespace: Cell::new(true), // Constraint: whitespace ignored
                pattern: String::from("# Start comment\n  charB"), // Setup with leading comment
            }
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn is_eof(&self) -> bool {
            self.offset() >= self.pattern.len() // Checks if we've reached the end
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern[self.offset()..].chars().next()
            }
        }
        
        fn chars(&self) -> impl Iterator<Item = (usize, char)> {
            self.pattern[self.offset()..].char_indices()
        }
    }

    let parser = MockParser::new();

    // Invoke peek_space
    let result = parser.peek_space();

    // Verify it returns 'B', the character after the comment
    assert_eq!(result, Some('B'));
} 

#[test]
fn test_peek_space_with_newline_in_comment() {
    struct MockParser {
        pos: Position,
        ignore_whitespace: Cell<bool>,
        pattern: String,
    }

    impl MockParser {
        fn new() -> Self {
            Self {
                pos: Position { offset: 0 },
                ignore_whitespace: Cell::new(true), // Constraint: whitespace ignored
                pattern: String::from("# comment part1\n# comment part2\n  charC"), // Newline in comments
            }
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn is_eof(&self) -> bool {
            self.offset() >= self.pattern.len() // Not at EOF
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern[self.offset()..].chars().next()
            }
        }
        
        fn chars(&self) -> impl Iterator<Item = (usize, char)> {
            self.pattern[self.offset()..].char_indices()
        }
    }

    let parser = MockParser::new();

    // Calling peek_space
    let result = parser.peek_space();

    // Expect 'C' as the character after the comments
    assert_eq!(result, Some('C'));
}

