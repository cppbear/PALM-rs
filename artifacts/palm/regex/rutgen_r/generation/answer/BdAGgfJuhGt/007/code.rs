// Answer 0

#[test]
fn test_bump_space_with_whitespace_and_comments() {
    struct TestParser {
        input: Vec<char>,
        position: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
                comments: vec![],
            }
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position]
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += 1;
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&mut self) -> &mut TestParser {
            self
        }
    }

    let mut parser = TestParser::new("   # This is a comment\n   a");

    // Invoking the method under test
    parser.bump_space();

    assert_eq!(parser.position, 5); // Position should skip leading whitespace and comment
    assert_eq!(parser.comments.len(), 1); // One comment should be captured
    assert_eq!(parser.comments[0].comment, " This is a comment"); // The content of the comment
}

#[test]
fn test_bump_space_with_only_whitespace() {
    struct TestParser {
        input: Vec<char>,
        position: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
                comments: vec![],
            }
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position]
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += 1;
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&mut self) -> &mut TestParser {
            self
        }
    }

    let mut parser = TestParser::new("   ");

    // Invoking the method under test
    parser.bump_space();

    assert_eq!(parser.position, 3); // Position should skip all whitespace
    assert_eq!(parser.comments.len(), 0); // No comments should be captured
}

#[test]
fn test_bump_space_with_comments_only() {
    struct TestParser {
        input: Vec<char>,
        position: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
                comments: vec![],
            }
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position]
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += 1;
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&mut self) -> &mut TestParser {
            self
        }
    }

    let mut parser = TestParser::new("# This is a comment\n");

    // Invoking the method under test
    parser.bump_space();

    assert_eq!(parser.position, 18); // Position should skip the comment
    assert_eq!(parser.comments.len(), 1); // One comment should be captured
    assert_eq!(parser.comments[0].comment, " This is a comment"); // The content of the comment
}

#[test]
fn test_bump_space_with_mixed_content() {
    struct TestParser {
        input: Vec<char>,
        position: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
                comments: vec![],
            }
        }

        fn ignore_whitespace(&self) -> bool {
            false
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position]
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += 1;
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&mut self) -> &mut TestParser {
            self
        }
    }

    let mut parser = TestParser::new("   # Comment 1\n   b   # Comment 2\n");

    // Invoking the method under test
    parser.bump_space();

    assert_eq!(parser.position, 21); // Position should skip all whitespace and comments
    assert_eq!(parser.comments.len(), 2); // Two comments should be captured
    assert_eq!(parser.comments[0].comment, " Comment 1"); // The content of the first comment
    assert_eq!(parser.comments[1].comment, " Comment 2"); // The content of the second comment
}

