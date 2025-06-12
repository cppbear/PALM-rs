// Answer 0

#[test]
fn test_bump_space_with_whitespace_and_comment_handling() {
    struct TestParser {
        pos: Cell<Position>,
        ignore_whitespace: Cell<bool>,
        pattern: String,
        comments: RefCell<Vec<ast::Comment>>,
    }

    impl TestParser {
        fn new(pattern: &str, ignore_whitespace: bool) -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                ignore_whitespace: Cell::new(ignore_whitespace),
                pattern: pattern.to_string(),
                comments: RefCell::new(Vec::new()),
            }
        }

        fn bump_space(&self) {
            if !self.ignore_whitespace.get() {
                return;
            }
            while !self.is_eof() {
                if self.char().is_whitespace() {
                    self.bump();
                } else if self.char() == '#' {
                    let start = self.pos.get();
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
                        span: Span::new(start, self.pos.get()),
                        comment: comment_text,
                    };
                    self.comments.borrow_mut().push(comment);
                } else {
                    break;
                }
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.get().offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            if self.is_eof() {
                return '\0';
            }
            self.pattern[self.pos.get().offset..].chars().next().unwrap()
        }

        fn bump(&self) {
            if self.is_eof() {
                return;
            }
            let Position { mut offset, mut line, mut column } = self.pos.get();

            if self.char() == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            offset += self.char().len_utf8();

            self.pos.set(Position { offset, line, column });
        }
    }

    // Initialize the parser with ignore_whitespace set to true and pattern including whitespace and comments
    let pattern = "   # comment line\n   x y";
    let parser = TestParser::new(pattern, true);
    
    // Before calling bump_space
    parser.bump_space();
    
    // Verify final parser position and comments collected
    assert!(parser.is_eof() == false);
    assert_eq!(parser.pos.get().offset, 10); // Position after whitespace and comment (length 10)
    assert_eq!(parser.comments.borrow().len(), 1); // One comment should be captured
    assert_eq!(parser.comments.borrow()[0].comment, " comment line"); // Check captured comment text
}

