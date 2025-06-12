// Answer 0

#[test]
fn test_bump_space_with_comments() {
    struct TestParser {
        input: Vec<char>,
        position: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.get(self.position).copied().unwrap_or('\0')
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += 1;
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&mut self) -> &mut Self {
            self
        }
    }

    let mut parser = TestParser {
        input: "# This is a comment\n  ".chars().collect(),
        position: 0,
        comments: vec![],
    };

    parser.bump_space();

    assert_eq!(parser.comments.len(), 1);
    assert_eq!(parser.comments[0].comment, " This is a comment");
    assert_eq!(parser.comments[0].span, Span::new(0, parser.pos()));
}

#[test]
fn test_bump_space_with_only_whitespace() {
    struct TestParser {
        input: Vec<char>,
        position: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.get(self.position).copied().unwrap_or('\0')
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += 1;
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&mut self) -> &mut Self {
            self
        }
    }

    let mut parser = TestParser {
        input: "   \n  ".chars().collect(),
        position: 0,
        comments: vec![],
    };

    parser.bump_space();

    assert_eq!(parser.comments.len(), 0);
    assert_eq!(parser.pos(), 4); // After bumping over whitespace and newline
}

#[test]
fn test_bump_space_edge_case() {
    struct TestParser {
        input: Vec<char>,
        position: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.get(self.position).copied().unwrap_or('\0')
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += 1;
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&mut self) -> &mut Self {
            self
        }
    }

    let mut parser = TestParser {
        input: "# Comment with no newline".chars().collect(),
        position: 0,
        comments: vec![],
    };

    parser.bump_space();

    assert_eq!(parser.comments.len(), 1);
    assert_eq!(parser.comments[0].comment, " Comment with no newline");
    assert_eq!(parser.comments[0].span, Span::new(0, parser.pos()));
}

#[test]
#[should_panic]
fn test_bump_space_no_whitespace_or_comments() {
    struct TestParser {
        input: Vec<char>,
        position: usize,
        comments: Vec<ast::Comment>,
    }

    impl TestParser {
        fn ignore_whitespace(&self) -> bool {
            true
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            self.input.get(self.position).copied().unwrap_or('\0')
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.position += 1;
            }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn parser(&mut self) -> &mut Self {
            self
        }
    }

    let mut parser = TestParser {
        input: "abc".chars().collect(),
        position: 0,
        comments: vec![],
    };

    parser.bump_space();
}

