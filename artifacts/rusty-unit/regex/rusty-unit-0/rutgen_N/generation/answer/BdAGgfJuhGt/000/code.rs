// Answer 0

#[test]
fn test_bump_space_with_whitespace() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        comments: Vec<ast::Comment>,
        ignore_whitespace: bool,
    }

    impl TestParser {
        fn new(input: &str, ignore_whitespace: bool) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
                comments: Vec::new(),
                ignore_whitespace,
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.pos += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.pos]
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&mut self) -> &mut Self {
            self
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }
        
        fn add_comment(&mut self, comment: ast::Comment) {
            self.comments.push(comment);
        }
    }

    impl TestParser {
        fn bump_space(&mut self) {
            if !self.ignore_whitespace() {
                return;
            }
            while !self.is_eof() {
                if self.char().is_whitespace() {
                    self.bump();
                } else if self.char() == '#' {
                    let start = self.pos();
                    let mut comment_text = String::new();
                    self.bump();
                    while !self.is_eof() {
                        let c = self.char();
                        self.bump();
                        if c == '\n' {
                            break;
                        }
                        comment_text.push(c);
                    }
                    let comment = ast::Comment {
                        span: Span::new(start, self.pos()),
                        comment: comment_text,
                    };
                    self.add_comment(comment);
                } else {
                    break;
                }
            }
        }
    }

    let mut parser = TestParser::new("   # This is a comment\n   5 , 6", true);
    parser.bump_space();

    assert_eq!(parser.pos, 13); // Position after ignoring whitespace and comment
    assert_eq!(parser.comments.len(), 1); // One comment should be captured
    assert_eq!(parser.comments[0].comment, "This is a comment"); // Check comment content
}

#[test]
fn test_bump_space_without_whitespace() {
    struct TestParser {
        input: Vec<char>,
        pos: usize,
        comments: Vec<ast::Comment>,
        ignore_whitespace: bool,
    }

    impl TestParser {
        fn new(input: &str, ignore_whitespace: bool) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
                comments: Vec::new(),
                ignore_whitespace,
            }
        }

        fn bump(&mut self) {
            if !self.is_eof() {
                self.pos += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.pos]
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn parser(&mut self) -> &mut Self {
            self
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }
        
        fn add_comment(&mut self, comment: ast::Comment) {
            self.comments.push(comment);
        }
    }

    impl TestParser {
        fn bump_space(&mut self) {
            if !self.ignore_whitespace() {
                return;
            }
            while !self.is_eof() {
                if self.char().is_whitespace() {
                    self.bump();
                } else if self.char() == '#' {
                    let start = self.pos();
                    let mut comment_text = String::new();
                    self.bump();
                    while !self.is_eof() {
                        let c = self.char();
                        self.bump();
                        if c == '\n' {
                            break;
                        }
                        comment_text.push(c);
                    }
                    let comment = ast::Comment {
                        span: Span::new(start, self.pos()),
                        comment: comment_text,
                    };
                    self.add_comment(comment);
                } else {
                    break;
                }
            }
        }
    }

    let mut parser = TestParser::new("5,6", false);
    parser.bump_space();

    assert_eq!(parser.pos, 0); // Position should remain unchanged
    assert_eq!(parser.comments.len(), 0); // No comments should be captured
}

