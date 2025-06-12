// Answer 0

#[test]
fn test_maybe_parse_ascii_class_valid_alnum() {
    struct Parser {
        input: &'static str,
        pos: usize,
    }

    impl Parser {
        fn new(input: &'static str) -> Self {
            Parser { input, pos: 0 }
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += self.char().len_utf8();
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &'static str {
            self.input
        }

        fn offset(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.input[self.pos..].starts_with(chars) {
                self.pos += chars.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = Parser::new("[[:alnum:]]");
    assert!(parser.maybe_parse_ascii_class().is_some());
}

#[test]
fn test_maybe_parse_ascii_class_invalid_name() {
    struct Parser {
        input: &'static str,
        pos: usize,
    }

    impl Parser {
        fn new(input: &'static str) -> Self {
            Parser { input, pos: 0 }
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += self.char().len_utf8();
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &'static str {
            self.input
        }

        fn offset(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.input[self.pos..].starts_with(chars) {
                self.pos += chars.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = Parser::new("[[:loower:]]");
    assert!(parser.maybe_parse_ascii_class().is_none());
}

#[test]
fn test_maybe_parse_ascii_class_missing_closing_bracket() {
    struct Parser {
        input: &'static str,
        pos: usize,
    }

    impl Parser {
        fn new(input: &'static str) -> Self {
            Parser { input, pos: 0 }
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += self.char().len_utf8();
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &'static str {
            self.input
        }

        fn offset(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.input[self.pos..].starts_with(chars) {
                self.pos += chars.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = Parser::new("[[:alnum:");
    assert!(parser.maybe_parse_ascii_class().is_none());
}

#[test]
fn test_maybe_parse_ascii_class_not_enclosed() {
    struct Parser {
        input: &'static str,
        pos: usize,
    }

    impl Parser {
        fn new(input: &'static str) -> Self {
            Parser { input, pos: 0 }
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += self.char().len_utf8();
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &'static str {
            self.input
        }

        fn offset(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.input[self.pos..].starts_with(chars) {
                self.pos += chars.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = Parser::new("[a-z]");
    assert!(parser.maybe_parse_ascii_class().is_none());
}

#[test]
fn test_maybe_parse_ascii_class_negated() {
    struct Parser {
        input: &'static str,
        pos: usize,
    }

    impl Parser {
        fn new(input: &'static str) -> Self {
            Parser { input, pos: 0 }
        }

        fn char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap_or('\0')
        }

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += self.char().len_utf8();
                true
            } else {
                false
            }
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn pattern(&self) -> &'static str {
            self.input
        }

        fn offset(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if self.input[self.pos..].starts_with(chars) {
                self.pos += chars.len();
                true
            } else {
                false
            }
        }
    }

    let mut parser = Parser::new("[[:^alnum:]]");
    assert!(parser.maybe_parse_ascii_class().is_some());
}

