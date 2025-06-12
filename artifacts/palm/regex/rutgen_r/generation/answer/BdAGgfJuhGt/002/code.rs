// Answer 0

#[test]
fn test_bump_space_with_whitespace() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
        ignore_whitespace: bool,
    }
    
    impl TestParser {
        fn new(input: &str, ignore_whitespace: bool) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
                ignore_whitespace,
            }
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position] as char
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

        fn parser(&self) -> &Self {
            self
        }

        fn comments(&self) -> &Vec<ast::Comment> {
            // The comments would need to be collected for verification
            unimplemented!()
        }
        
        fn bump_space(&mut self) {
            if !self.ignore_whitespace {
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
                    self.parser().comments.borrow_mut().push(comment);
                } else {
                    break;
                }
            }
        }
    }

    let mut parser = TestParser::new("   # this is a comment\n   ", true);
    parser.bump_space();
    assert_eq!(parser.pos(), 16); // Whitespace and comment should be skipped
}

#[test]
fn test_bump_space_without_whitespace() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
        ignore_whitespace: bool,
    }

    impl TestParser {
        fn new(input: &str, ignore_whitespace: bool) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
                ignore_whitespace,
            }
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                '\0'
            } else {
                self.input[self.position] as char
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

        fn parser(&self) -> &Self {
            self
        }

        fn comments(&self) -> &Vec<ast::Comment> {
            // The comments would need to be collected for verification
            unimplemented!()
        }

        fn bump_space(&mut self) {
            if !self.ignore_whitespace {
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
                    self.parser().comments.borrow_mut().push(comment);
                } else {
                    break;
                }
            }
        }
    }

    let mut parser = TestParser::new("abc", true);
    parser.bump_space();
    assert_eq!(parser.pos(), 0); // No whitespace or comments should lead to no change
}

#[test]
#[should_panic]
fn test_bump_space_should_trigger_panic_on_eof() {
    struct TestParser {
        input: Vec<u8>,
        position: usize,
        ignore_whitespace: bool,
    }

    impl TestParser {
        fn new(input: &str, ignore_whitespace: bool) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
                ignore_whitespace,
            }
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                panic!("char called on EOF");
            } else {
                self.input[self.position] as char
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

        fn parser(&self) -> &Self {
            self
        }

        fn bump_space(&mut self) {
            if !self.ignore_whitespace {
                return;
            }
            while !self.is_eof() {
                if self.char().is_whitespace() {
                    self.bump();
                } else if self.char() == '#' {
                    self.bump(); // Should not panic even if we are at EOF
                } else {
                    break;
                }
            }
        }
    }

    let mut parser = TestParser::new("", true);
    parser.bump_space(); // This should panic as we are at EOF
}

