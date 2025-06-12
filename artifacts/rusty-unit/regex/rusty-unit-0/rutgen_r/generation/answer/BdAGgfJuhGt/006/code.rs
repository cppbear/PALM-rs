// Answer 0

#[test]
fn test_bump_space_with_ignore_whitespace_and_comment() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
                comments: Vec::new(),
            }
        }

        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&mut self) -> &mut Self {
            self
        }
    }

    let mut parser = TestParser::new("# This is a comment\n# Another comment");
    parser.bump_space();

    assert_eq!(parser.comments.len(), 2);
    assert_eq!(parser.comments[0].comment, " This is a comment");
    assert_eq!(parser.comments[1].comment, " Another comment");
}

#[test]
fn test_bump_space_no_whitespace() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
                comments: Vec::new(),
            }
        }

        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&mut self) -> &mut Self {
            self
        }
    }

    let mut parser = TestParser::new("# Comment without leading whitespace");
    parser.bump_space();

    assert_eq!(parser.comments.len(), 1);
    assert_eq!(parser.comments[0].comment, " Comment without leading whitespace");
}

