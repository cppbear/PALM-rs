// Answer 0

#[test]
fn test_maybe_parse_ascii_class_valid_case() {
    struct TestParser {
        pattern: String,
        pos: Position,
        current_index: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                current_index: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.current_index < self.pattern.len() {
                self.current_index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.current_index..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn offset(&self) -> usize {
            self.current_index
        }

        fn is_eof(&self) -> bool {
            self.current_index >= self.pattern.len()
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if self.pattern[self.current_index..].starts_with(s) {
                self.current_index += s.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = TestParser::new("[[:alpha:]]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());
}

#[test]
fn test_maybe_parse_ascii_class_no_colon() {
    struct TestParser {
        pattern: String,
        pos: Position,
        current_index: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                current_index: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.current_index < self.pattern.len() {
                self.current_index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.current_index..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn offset(&self) -> usize {
            self.current_index
        }

        fn is_eof(&self) -> bool {
            self.current_index >= self.pattern.len()
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if self.pattern[self.current_index..].starts_with(s) {
                self.current_index += s.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = TestParser::new("[alpha:]"); // Missing starting ':'
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_negated() {
    struct TestParser {
        pattern: String,
        pos: Position,
        current_index: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                current_index: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.current_index < self.pattern.len() {
                self.current_index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.current_index..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn offset(&self) -> usize {
            self.current_index
        }

        fn is_eof(&self) -> bool {
            self.current_index >= self.pattern.len()
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if self.pattern[self.current_index..].starts_with(s) {
                self.current_index += s.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = TestParser::new("[[:^digit:]]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());
}

#[test]
fn test_maybe_parse_ascii_class_invalid_name() {
    struct TestParser {
        pattern: String,
        pos: Position,
        current_index: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
                current_index: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.current_index < self.pattern.len() {
                self.current_index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.current_index..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn offset(&self) -> usize {
            self.current_index
        }

        fn is_eof(&self) -> bool {
            self.current_index >= self.pattern.len()
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if self.pattern[self.current_index..].starts_with(s) {
                self.current_index += s.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = TestParser::new("[[:loower:]]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

