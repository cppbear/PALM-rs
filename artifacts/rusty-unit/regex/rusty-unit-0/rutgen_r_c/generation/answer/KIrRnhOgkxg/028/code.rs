// Answer 0

#[test]
fn test_maybe_parse_ascii_class_no_initial_bump() {
    struct TestParser {
        pattern: String,
        pos: Position,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.pattern[self.pos.offset..].starts_with(chars) {
                self.pos.offset += chars.len();
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn maybe_parse_ascii_class(&mut self) -> Option<ClassAscii> {
            assert_eq!(self.char(), '[');
            let start = self.pos();
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
            let name_start = self.pos.offset;
            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                self.pos = start;
                return None;
            }
            let name = &self.pattern[name_start..self.pos.offset];
            if !self.bump_if(":]") {
                self.pos = start;
                return None;
            }
            let kind = ClassAsciiKind::from_name(name)?;
            Some(ClassAscii {
                span: Span::new(start, self.pos()),
                kind,
                negated,
            })
        }
    }

    let mut parser = TestParser::new("[::]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_no_closing_bracket() {
    struct TestParser {
        pattern: String,
        pos: Position,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.pattern[self.pos.offset..].starts_with(chars) {
                self.pos.offset += chars.len();
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn maybe_parse_ascii_class(&mut self) -> Option<ClassAscii> {
            assert_eq!(self.char(), '[');
            let start = self.pos();
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
            let name_start = self.pos.offset;
            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                self.pos = start;
                return None;
            }
            let name = &self.pattern[name_start..self.pos.offset];
            if !self.bump_if(":]") {
                self.pos = start;
                return None;
            }
            let kind = ClassAsciiKind::from_name(name)?;
            Some(ClassAscii {
                span: Span::new(start, self.pos()),
                kind,
                negated,
            })
        }
    }

    let mut parser = TestParser::new("[[:lower]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_empty_class_name() {
    struct TestParser {
        pattern: String,
        pos: Position,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            Self {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.offset).unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.pattern[self.pos.offset..].starts_with(chars) {
                self.pos.offset += chars.len();
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn maybe_parse_ascii_class(&mut self) -> Option<ClassAscii> {
            assert_eq!(self.char(), '[');
            let start = self.pos();
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
            let name_start = self.pos.offset;
            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                self.pos = start;
                return None;
            }
            let name = &self.pattern[name_start..self.pos.offset];
            if !self.bump_if(":]") {
                self.pos = start;
                return None;
            }
            let kind = ClassAsciiKind::from_name(name)?;
            Some(ClassAscii {
                span: Span::new(start, self.pos()),
                kind,
                negated,
            })
        }
    }

    let mut parser = TestParser::new("[[:]]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

