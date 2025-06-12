// Answer 0

#[test]
fn test_peek_space_no_whitespace_mode() {
    struct DummyParser {
        pos: Position,
        ignore_whitespace: bool,
        pattern: String,
    }

    impl DummyParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position::default(),
                ignore_whitespace: false,
                pattern: pattern.to_string(),
            }
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern[self.offset()..].chars().next()
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn char(&self) -> char {
            self.peek().unwrap_or('\0') // Fallback character for the purpose of this test
        }
    }

    let parser = ParserI::new(DummyParser::new("abc # comment"), "abc # comment");
    assert_eq!(parser.peek_space(), Some('a'));
}

#[test]
fn test_peek_space_with_whitespace_mode() {
    struct DummyParser {
        pos: Position,
        ignore_whitespace: bool,
        pattern: String,
    }

    impl DummyParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position::default(),
                ignore_whitespace: true,
                pattern: pattern.to_string(),
            }
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern[self.offset()..].chars().next()
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn char(&self) -> char {
            self.peek().unwrap_or('\0') // Fallback character for the purpose of this test
        }
    }

    let parser = ParserI::new(DummyParser::new("abc # comment"), "abc # comment");
    assert_eq!(parser.peek_space(), Some('a'));
}

#[test]
fn test_peek_space_ignore_comment() {
    struct DummyParser {
        pos: Position,
        ignore_whitespace: bool,
        pattern: String,
    }

    impl DummyParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position::default(),
                ignore_whitespace: true,
                pattern: pattern.to_string(),
            }
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern[self.offset()..].chars().next()
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn char(&self) -> char {
            self.peek().unwrap_or('\0') // Fallback character for the purpose of this test
        }
    }

    let parser = ParserI::new(DummyParser::new("abc # comment\ndef"), "abc # comment\ndef");
    assert_eq!(parser.peek_space(), Some('d'));
}

#[test]
fn test_peek_space_end_of_file() {
    struct DummyParser {
        pos: Position,
        ignore_whitespace: bool,
        pattern: String,
    }

    impl DummyParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Position::default(),
                ignore_whitespace: true,
                pattern: pattern.to_string(),
            }
        }

        fn offset(&self) -> usize {
            self.pos.offset
        }

        fn is_eof(&self) -> bool {
            self.offset() == self.pattern.len()
        }

        fn peek(&self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                self.pattern[self.offset()..].chars().next()
            }
        }

        fn ignore_whitespace(&self) -> bool {
            self.ignore_whitespace
        }

        fn char(&self) -> char {
            self.peek().unwrap_or('\0') // Fallback character for the purpose of this test
        }
    }

    let parser = ParserI::new(DummyParser::new(""), "");
    assert_eq!(parser.peek_space(), None);
}

