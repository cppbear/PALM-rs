// Answer 0

#[test]
fn test_maybe_parse_ascii_class_no_closing_bracket() {
    struct DummyParser {
        input: Vec<char>,
        pos: usize,
    }

    impl DummyParser {
        fn new(input: &str) -> Self {
            Self { input: input.chars().collect(), pos: 0 }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn set_pos(&mut self, pos: usize) {
            self.pos = pos;
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pattern(&self) -> &Vec<char> {
            &self.input
        }
    }

    impl DummyParser {
        fn maybe_parse_ascii_class(&mut self) -> Option<ast::ClassAscii> {
            assert_eq!(self.char(), '[');
            let start = self.pos();
            if !self.bump() || self.char() != ':' {
                self.set_pos(start);
                return None;
            }
            if !self.bump() {
                self.set_pos(start);
                return None;
            }
            if self.char() == '^' {
                if !self.bump() {
                    self.set_pos(start);
                    return None;
                }
            }
            let name_start = self.pos();
            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                self.set_pos(start);
                return None;
            }
            if self.bump() {
                self.set_pos(start);
                return None;
            }
            Some(ast::ClassAscii { span: Span::new(start, self.pos()), kind: ast::ClassAsciiKind::Alnum, negated: false })
        }
    }

    let mut parser = DummyParser::new("[[:alnum:");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_incomplete_class_name() {
    struct DummyParser {
        input: Vec<char>,
        pos: usize,
    }

    impl DummyParser {
        fn new(input: &str) -> Self {
            Self { input: input.chars().collect(), pos: 0 }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn set_pos(&mut self, pos: usize) {
            self.pos = pos;
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pattern(&self) -> &Vec<char> {
            &self.input
        }
    }

    impl DummyParser {
        fn maybe_parse_ascii_class(&mut self) -> Option<ast::ClassAscii> {
            assert_eq!(self.char(), '[');
            let start = self.pos();
            if !self.bump() || self.char() != ':' {
                self.set_pos(start);
                return None;
            }
            if !self.bump() {
                self.set_pos(start);
                return None;
            }
            let mut negated = false;
            if self.char() == '^' {
                negated = true;
                if !self.bump() {
                    self.set_pos(start);
                    return None;
                }
            }
            let name_start = self.pos();
            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                self.set_pos(start);
                return None;
            }
            let name = &self.pattern()[name_start..self.pos()];
            if self.bump() {
                self.set_pos(start);
                return None;
            }
            None
        }
    }

    let mut parser = DummyParser::new("[[:lower");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

