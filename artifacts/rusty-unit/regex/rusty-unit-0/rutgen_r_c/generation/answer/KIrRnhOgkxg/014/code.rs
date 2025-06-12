// Answer 0

#[test]
fn test_maybe_parse_ascii_class_valid() {
    #[derive(Clone)]
    struct MockParser<'s> {
        pattern: &'s str,
        pos: Position,
        index: usize,
    }

    impl<'s> MockParser<'s> {
        fn new(pattern: &'s str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
                index: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.index < self.pattern.len() {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.index..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn offset(&self) -> usize {
            self.index
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pattern(&self) -> &'s str {
            self.pattern
        }

        fn is_eof(&self) -> bool {
            self.index >= self.pattern.len()
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.pattern[self.index..].starts_with(chars) {
                self.index += chars.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = MockParser::new("[[:alnum:]]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());
}

#[test]
fn test_maybe_parse_ascii_class_invalid() {
    #[derive(Clone)]
    struct MockParser<'s> {
        pattern: &'s str,
        pos: Position,
        index: usize,
    }

    impl<'s> MockParser<'s> {
        fn new(pattern: &'s str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
                index: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.index < self.pattern.len() {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.index..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn offset(&self) -> usize {
            self.index
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pattern(&self) -> &'s str {
            self.pattern
        }

        fn is_eof(&self) -> bool {
            self.index >= self.pattern.len()
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.pattern[self.index..].starts_with(chars) {
                self.index += chars.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = MockParser::new("[[:invalid:]]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_without_closing() {
    #[derive(Clone)]
    struct MockParser<'s> {
        pattern: &'s str,
        pos: Position,
        index: usize,
    }

    impl<'s> MockParser<'s> {
        fn new(pattern: &'s str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
                index: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.index < self.pattern.len() {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.index..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn offset(&self) -> usize {
            self.index
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pattern(&self) -> &'s str {
            self.pattern
        }

        fn is_eof(&self) -> bool {
            self.index >= self.pattern.len()
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.pattern[self.index..].starts_with(chars) {
                self.index += chars.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = MockParser::new("[[:digit:");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_negated() {
    #[derive(Clone)]
    struct MockParser<'s> {
        pattern: &'s str,
        pos: Position,
        index: usize,
    }

    impl<'s> MockParser<'s> {
        fn new(pattern: &'s str) -> Self {
            Self {
                pattern,
                pos: Position { offset: 0, line: 1, column: 1 },
                index: 0,
            }
        }

        fn bump(&mut self) -> bool {
            if self.index < self.pattern.len() {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.pattern[self.index..].chars().next().unwrap_or('\0')
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn offset(&self) -> usize {
            self.index
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pattern(&self) -> &'s str {
            self.pattern
        }

        fn is_eof(&self) -> bool {
            self.index >= self.pattern.len()
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.pattern[self.index..].starts_with(chars) {
                self.index += chars.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = MockParser::new("[[:^upper:]]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());
    if let Some(class) = result {
        assert!(class.negated);
    }
}

