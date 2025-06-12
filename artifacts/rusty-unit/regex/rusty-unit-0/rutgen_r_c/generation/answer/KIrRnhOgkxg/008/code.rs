// Answer 0

#[test]
fn test_maybe_parse_ascii_class_no_valid_class() {
    struct TestParser {
        position: Position,
        pattern: &'static str,
        current_char: usize,
    }
    
    impl TestParser {
        fn char(&self) -> char {
            self.pattern.chars().nth(self.current_char).unwrap_or('\0')
        }
        
        fn bump(&mut self) -> bool {
            if self.current_char + 1 < self.pattern.len() {
                self.current_char += 1;
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.position
        }

        fn is_eof(&self) -> bool {
            self.current_char >= self.pattern.len()
        }

        fn offset(&self) -> usize {
            self.current_char
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if self.pattern[self.current_char..].starts_with(s) {
                self.current_char += s.len();
                true
            } else {
                false
            }
        }
        
        fn maybe_parse_ascii_class(&mut self) -> Option<ClassAscii> {
            assert_eq!(self.char(), '[');
            let start = self.pos();
            let mut negated = false;

            if !self.bump() || self.char() != ':' {
                return None;
            }
            if !self.bump() {
                return None;
            }
            if self.char() == '^' {
                negated = true;
                if !self.bump() {
                    return None;
                }
            }
            let name_start = self.offset();
            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                return None;
            }
            let name = &self.pattern[name_start..self.offset()];
            if !self.bump_if(":]") {
                return None;
            }
            let kind = ClassAsciiKind::from_name(name);
            match kind {
                Some(_) => None,
                None => None,
            }
        }
    }

    let mut parser = TestParser {
        position: Position { offset: 0, line: 1, column: 1 },
        pattern: "[[:loower:]]",
        current_char: 0,
    };

    assert_eq!(parser.maybe_parse_ascii_class(), None);
}

