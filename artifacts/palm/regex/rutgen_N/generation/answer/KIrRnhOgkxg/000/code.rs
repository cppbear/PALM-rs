// Answer 0

#[test]
fn test_maybe_parse_ascii_class_valid() {
    struct Parser {
        input: Vec<u8>,
        pos: usize,
    }
    
    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos] as char
        }

        fn bump(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &[u8] {
            &self.input
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump_if(&mut self, expected: &str) -> bool {
            if self.input[self.pos..].starts_with(expected.as_bytes()) {
                self.pos += expected.len();
                true
            } else {
                false
            }
        }

        fn maybe_parse_ascii_class(&mut self) -> Option<ast::ClassAscii> {
            // Original function implementation goes here...
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
            let name_start = self.pos;
            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                self.pos = start;
                return None;
            }
            let name = &self.pattern()[name_start..self.pos];
            if !self.bump_if(":]") {
                self.pos = start;
                return None;
            }
            let kind = ast::ClassAsciiKind::from_name(name);
            match kind {
                Some(kind) => Some(ast::ClassAscii {
                    span: Span::new(start, self.pos),
                    kind,
                    negated,
                }),
                None => {
                    self.pos = start;
                    None
                },
            }
        }
    }
    
    let mut parser = Parser::new("[[:alnum:]]");
    assert!(parser.maybe_parse_ascii_class().is_some());
}

#[test]
fn test_maybe_parse_ascii_class_invalid() {
    struct Parser {
        input: Vec<u8>,
        pos: usize,
    }
    
    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos] as char
        }

        fn bump(&mut self) -> bool {
            if self.pos + 1 < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &[u8] {
            &self.input
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump_if(&mut self, expected: &str) -> bool {
            if self.input[self.pos..].starts_with(expected.as_bytes()) {
                self.pos += expected.len();
                true
            } else {
                false
            }
        }

        fn maybe_parse_ascii_class(&mut self) -> Option<ast::ClassAscii> {
            // Original function implementation goes here...
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
            let name_start = self.pos;
            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                self.pos = start;
                return None;
            }
            let name = &self.pattern()[name_start..self.pos];
            if !self.bump_if(":]") {
                self.pos = start;
                return None;
            }
            let kind = ast::ClassAsciiKind::from_name(name);
            match kind {
                Some(kind) => Some(ast::ClassAscii {
                    span: Span::new(start, self.pos),
                    kind,
                    negated,
                }),
                None => {
                    self.pos = start;
                    None
                },
            }
        }
    }
    
    let mut parser = Parser::new("[[:loower:]]");
    assert!(parser.maybe_parse_ascii_class().is_none());
}

