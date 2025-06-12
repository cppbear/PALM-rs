// Answer 0

#[test]
fn test_bump_space_with_whitespace_and_comment() {
    struct MockParser {
        pos: Position,
        ignore_whitespace: bool,
        comments: RefCell<Vec<ast::Comment>>,
    }

    impl MockParser {
        fn new(ignore_whitespace: bool) -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                ignore_whitespace,
                comments: RefCell::new(vec![]),
            }
        }

        fn bump(&mut self) {
            // Simulate bumping the position
            self.pos.offset += 1;
            self.pos.column += 1;
        }

        fn char(&self) -> char {
            match self.pos.offset {
                0 => ' ',
                1 => ' ',
                2 => '#',
                3 => 'C',
                4 => 'o',
                5 => 'm',
                6 => 'm',
                _ => '\n', // End of comment
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= 7 // Assume length of string is 7
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }
    }

    let mut parser = MockParser::new(true);
    parser.bump_space();

    let comments = parser.comments.take();
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].comment, "Comm");
    assert_eq!(comments[0].span.start, parser.pos());
}

#[test]
fn test_bump_space_no_whitespace() {
    struct MockParser {
        pos: Position,
        ignore_whitespace: bool,
    }

    impl MockParser {
        fn new(ignore_whitespace: bool) -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                ignore_whitespace,
            }
        }

        fn bump(&mut self) {
            self.pos.offset += 1;
            self.pos.column += 1;
        }

        fn char(&self) -> char {
            'a' // Random character
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= 1 // Assume length of string is 1
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }
    }

    let mut parser = MockParser::new(false);
    parser.bump_space();
    assert_eq!(parser.pos.offset, 0); // Position should not change
}

#[test]
fn test_bump_space_empty_comment() {
    struct MockParser {
        pos: Position,
        ignore_whitespace: bool,
        comments: RefCell<Vec<ast::Comment>>,
    }

    impl MockParser {
        fn new(ignore_whitespace: bool) -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                ignore_whitespace,
                comments: RefCell::new(vec![]),
            }
        }

        fn bump(&mut self) {
            self.pos.offset += 1;
            if self.pos.offset == 2 {
                self.pos.line += 1;
                self.pos.column = 1;
            } else {
                self.pos.column += 1;
            }
        }

        fn char(&self) -> char {
            match self.pos.offset {
                0 => ' ',
                1 => '#',
                2 => '\n', // End of comment
                _ => '\0', // No more characters
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= 3 // Assume length of string is 3
        }

        fn pos(&self) -> Position {
            self.pos.clone()
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn parser(&self) -> &MockParser {
            self
        }
    }

    let mut parser = MockParser::new(true);
    parser.bump_space();

    let comments = parser.comments.take();
    assert!(comments.is_empty()); // No comment content
}

