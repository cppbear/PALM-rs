// Answer 0

#[test]
fn test_maybe_parse_ascii_class_with_invalid_class() {
    struct Parser {
        pattern: &'static str,
        pos: usize,
    }

    impl Parser {
        fn new(pattern: &'static str) -> Self {
            Self { pattern, pos: 0 }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.pattern.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.pattern.len()
        }

        fn pattern(&self) -> &'static str {
            self.pattern
        }

        fn offset(&self) -> usize {
            self.pos
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if self.pattern[self.pos..].starts_with(s) {
                self.pos += s.len();
                true
            } else {
                false
            }
        }
    }

    impl Parser {
        fn maybe_parse_ascii_class(&mut self) -> Option<ast::ClassAscii> {
            assert_eq!(self.char(), '[');
            let start = self.pos;
            let mut negated = false;
            if !self.bump() || self.char() != ':' {
                self.pos = start;
                return None;
            }
            if !self.bump() {
                self.pos = start;
                return None;
            }
            if self.char() == '^' {
                negated = true;
                if !self.bump() {
                    self.pos = start;
                    return None;
                }
            }
            let name_start = self.offset();
            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                self.pos = start;
                return None;
            }
            let name = &self.pattern()[name_start..self.offset()];
            if !self.bump_if(":]") {
                self.pos = start;
                return None;
            }
            match ast::ClassAsciiKind::from_name(name) {
                Some(_) => Some(ast::ClassAscii {
                    span: Span::new(start, self.pos),
                    kind: ast::ClassAsciiKind::Alnum, // Example; adjust as necessary
                    negated,
                }),
                None => {
                    self.pos = start;
                    return None;
                }
            }
        }
    }

    let mut parser = Parser::new("[[:loower:]]");
    let result = parser.maybe_parse_ascii_class();
    assert_eq!(result, None);
}

