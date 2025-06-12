// Answer 0

#[test]
fn test_parse_set_class_open_negated() {
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

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<!, ()> {
            Err(())
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1;
                return true;
            }
            false
        }
    }

    let parser = MockParser::new("[^a-z]");

    let result = parse_set_class_open(&parser);
    assert!(result.is_ok());
}

#[test]
fn test_parse_set_class_open_empty_union() {
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

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.input.len() {
                self.pos += 1;
                return true;
            }
            false
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> Result<!, ()> {
            Err(())
        }
    }

    let parser = MockParser::new("[]");

    let result = parse_set_class_open(&parser);
    assert!(result.is_ok());
}

