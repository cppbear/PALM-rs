// Answer 0

#[test]
fn test_parse_set_class_open_valid_basic() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.bump();
                return true;
            }
            false
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<(), ()> {
            Err(())
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser::new("[abc]");
    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_open_valid_negated() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.bump();
                return true;
            }
            false
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<(), ()> {
            Err(())
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser::new("[^abc]");
    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_open_empty_class() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.bump();
                return true;
            }
            false
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<(), ()> {
            Err(())
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser::new("[--]");
    let result = parser.parse_set_class_open();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_set_class_open_unclosed() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            false
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<(), ()> {
            Err(())
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos + 1)
        }
    }

    let mut parser = MockParser::new("[abc");
    parser.parse_set_class_open().unwrap(); // This should panic
}

