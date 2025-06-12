// Answer 0

#[test]
fn test_bump_space_with_whitespace() {
    struct MockParser {
        ignore_whitespace: bool,
        input: String,
        position: Position,
        comments: Vec<ast::Comment>,
    }
    
    impl MockParser {
        fn new(ignore_whitespace: bool, input: &str) -> Self {
            Self {
                ignore_whitespace,
                input: input.to_string(),
                position: Position { offset: 0, line: 1, column: 1 },
                comments: Vec::new(),
            }
        }

        fn is_eof(&self) -> bool {
            self.position.offset >= self.input.len()
        }
        
        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position.offset..].chars().next().unwrap()
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position.offset += self.char().len_utf8();
            }
        }

        fn pos(&self) -> Position {
            self.position
        }
        
        fn push_comment(&mut self, comment: ast::Comment) {
            self.comments.push(comment);
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }
    }

    let mut parser = MockParser::new(true, "   # This is a comment\n   5, 6");

    parser.bump_space(); // Invoking the bump_space function

    assert_eq!(parser.position.offset, 12); // Should skip all whitespaces and the comment
}

#[test]
fn test_bump_space_without_whitespace() {
    struct MockParser {
        ignore_whitespace: bool,
        input: String,
        position: Position,
        comments: Vec<ast::Comment>,
    }
    
    impl MockParser {
        fn new(ignore_whitespace: bool, input: &str) -> Self {
            Self {
                ignore_whitespace,
                input: input.to_string(),
                position: Position { offset: 0, line: 1, column: 1 },
                comments: Vec::new(),
            }
        }

        fn is_eof(&self) -> bool {
            self.position.offset >= self.input.len()
        }
        
        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position.offset..].chars().next().unwrap()
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position.offset += self.char().len_utf8();
            }
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }
    }

    let mut parser = MockParser::new(false, "   # This is a comment\n   5, 6");

    parser.bump_space(); // Invoking the bump_space function

    assert_eq!(parser.position.offset, 0); // Should remain at the start since ignore_whitespace is false
}

#[test]
#[should_panic]
fn test_bump_space_at_eof() {
    struct MockParser {
        ignore_whitespace: bool,
        input: String,
        position: Position,
    }
    
    impl MockParser {
        fn new(ignore_whitespace: bool, input: &str) -> Self {
            Self {
                ignore_whitespace,
                input: input.to_string(),
                position: Position { offset: input.len(), line: 1, column: 1 },
            }
        }

        fn is_eof(&self) -> bool {
            self.position.offset >= self.input.len()
        }
        
        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position.offset..].chars().next().unwrap()
            }
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }
    }

    let mut parser = MockParser::new(true, "");
    
    parser.bump_space(); // Invoking the bump_space function, should panic due to being at EOF
}

