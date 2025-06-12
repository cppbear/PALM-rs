// Answer 0

#[test]
fn test_parse_octal_valid_input() {
    struct TestParser {
        octal: bool,
        pos: Position,
        pattern: String,
        current_char: char,
        offset: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump(&mut self) -> bool {
            self.offset += 1;
            if self.offset < self.pattern.len() {
                self.current_char = self.pattern.chars().nth(self.offset).unwrap();
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let mut test_parser = TestParser {
        octal: true,
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from("077"),
        current_char: '0',
        offset: 0,
    };

    let literal = test_parser.parse_octal();

    assert_eq!(literal.kind, ast::LiteralKind::Octal);
    assert_eq!(literal.c, '');  // Since octal 077 corresponds to '`'
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_input_underflow() {
    struct TestParser {
        octal: bool,
        pos: Position,
        pattern: String,
        current_char: char,
        offset: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump(&mut self) -> bool {
            self.offset += 1;
            if self.offset < self.pattern.len() {
                self.current_char = self.pattern.chars().nth(self.offset).unwrap();
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let mut test_parser = TestParser {
        octal: true,
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from("8"),  // Invalid starting character for octal
        current_char: '8',
        offset: 0,
    };

    let _ = test_parser.parse_octal();  // This should panic
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_unicode() {
    struct TestParser {
        octal: bool,
        pos: Position,
        pattern: String,
        current_char: char,
        offset: usize,
    }

    impl TestParser {
        fn parser(&self) -> &Self {
            self
        }

        fn char(&self) -> char {
            self.current_char
        }

        fn bump(&mut self) -> bool {
            self.offset += 1;
            if self.offset < self.pattern.len() {
                self.current_char = self.pattern.chars().nth(self.offset).unwrap();
                true
            } else {
                false
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let mut test_parser = TestParser {
        octal: true,
        pos: Position { offset: 0, line: 1, column: 1 },
        pattern: String::from("512"),  // Valid octal, but invalid Unicode
        current_char: '5',
        offset: 0,
    };

    let _ = test_parser.parse_octal();  // This should panic on char::from_u32
}

