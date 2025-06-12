// Answer 0

#[test]
fn test_maybe_parse_ascii_class_valid_class() {
    struct TestParser<'a> {
        pattern: &'a str,
        pos: Position,
    }

    impl<'a> TestParser<'a> {
        fn new(pattern: &'a str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap_or('\0')
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

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn maybe_parse_ascii_class(&mut self) -> Option<ClassAscii> {
            // The method implementation directly as provided...
            // ...
            // Return None if the parsing fails as per the original logic
        }
    }

    let mut parser = TestParser::new("[[:digit:]]");
    assert!(parser.maybe_parse_ascii_class().is_some());
}

#[test]
fn test_maybe_parse_ascii_class_invalid_class() {
    struct TestParser<'a> {
        pattern: &'a str,
        pos: Position,
    }

    impl<'a> TestParser<'a> {
        fn new(pattern: &'a str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap_or('\0')
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

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn maybe_parse_ascii_class(&mut self) -> Option<ClassAscii> {
            // The method implementation directly as provided...
            // ...
            // Return None if the parsing fails as per the original logic
        }
    }

    let mut parser = TestParser::new("[[:loower:]]");
    assert!(parser.maybe_parse_ascii_class().is_none());
}

#[test]
fn test_maybe_parse_ascii_class_correct_syntax() {
    struct TestParser<'a> {
        pattern: &'a str,
        pos: Position,
    }

    impl<'a> TestParser<'a> {
        fn new(pattern: &'a str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap_or('\0')
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

        fn is_eof(&self) -> bool {
            self.pos.offset >= self.pattern.len()
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn maybe_parse_ascii_class(&mut self) -> Option<ClassAscii> {
            // The method implementation directly as provided...
            // ...
            // Return None if the parsing fails as per the original logic
        }
    }

    let mut parser = TestParser::new("[[:^upper:]]");
    assert!(parser.maybe_parse_ascii_class().is_some());
}

