// Answer 0

#[test]
fn test_bump_space_no_op_when_eof() {
    struct TestParser {
        eof: bool,
        ignore_whitespace: bool,
        input: Vec<char>,
        pos: usize,
    }

    impl TestParser {
        fn new(input: &str, ignore_whitespace: bool) -> Self {
            Self {
                eof: false,
                ignore_whitespace,
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn char(&self) -> char {
            if self.pos < self.input.len() {
                self.input[self.pos]
            } else {
                '\0'
            }
        }

        fn bump(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&self) -> &Self {
            self
        }
    }

    let mut parser = TestParser::new("   ", true);
    parser.eof = true; // Setting EOF

    parser.bump_space(); // Calling the function

    assert_eq!(parser.pos, 0); // Position should not change since we are at EOF
}

#[test]
fn test_bump_space_whitespace_and_comment() {
    struct TestParser {
        eof: bool,
        ignore_whitespace: bool,
        input: Vec<char>,
        pos: usize,
        comments: Vec<String>,
    }

    impl TestParser {
        fn new(input: &str, ignore_whitespace: bool) -> Self {
            Self {
                eof: false,
                ignore_whitespace,
                input: input.chars().collect(),
                pos: 0,
                comments: Vec::new(),
            }
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn char(&self) -> char {
            if self.pos < self.input.len() {
                self.input[self.pos]
            } else {
                '\0'
            }
        }

        fn bump(&mut self) {
            if self.pos < self.input.len() {
                self.pos += 1;
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&mut self) -> &mut Self {
            self
        }

        fn push_comment(&mut self, comment: String) {
            self.comments.push(comment);
        }
    }

    let mut parser = TestParser::new("   # This is a comment\n   ", true);
    parser.eof = false; // Not setting EOF

    parser.bump_space(); // Calling the function

    assert_eq!(parser.pos, 11); // Position should advance past whitespace and comment
    assert_eq!(parser.comments.len(), 1); // One comment should have been collected
    assert_eq!(parser.comments[0], " This is a comment"); // Check the collected comment
}

