// Answer 0

#[test]
fn test_parse_octal_valid() {
    struct MockParser {
        octal: bool,
        pos: Position,
        pattern: &'static str,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                octal: true,
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: "075",
            }
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += self.pattern[self.pos.offset..].chars().next().unwrap().len_utf8();
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn pattern(&self) -> &str {
            self.pattern
        }
    }

    let parser = MockParser::new();
    let result = parser.parse_octal();
    assert_eq!(result.c, 'o');
    assert_eq!(result.kind, LiteralKind::Octal);
    assert_eq!(result.span.start.offset, 0);
    assert_eq!(result.span.end.offset, 3);
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_start() {
    struct MockParser {
        octal: bool,
        pos: Position,
        pattern: &'static str,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                octal: true,
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: "g75",
            }
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += self.pattern[self.pos.offset..].chars().next().unwrap().len_utf8();
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn pattern(&self) -> &str {
            self.pattern
        }
    }

    let parser = MockParser::new();
    parser.parse_octal();
}

#[test]
fn test_parse_octal_max_value() {
    struct MockParser {
        octal: bool,
        pos: Position,
        pattern: &'static str,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                octal: true,
                pos: Position { offset: 0, line: 1, column: 1 },
                pattern: "777",
            }
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap()
        }

        fn bump(&mut self) {
            if self.pos.offset < self.pattern.len() {
                self.pos.offset += self.pattern[self.pos.offset..].chars().next().unwrap().len_utf8();
            }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn pattern(&self) -> &str {
            self.pattern
        }
    }

    let parser = MockParser::new();
    let result = parser.parse_octal();
    assert_eq!(result.c, 'ɿ'); // Expecting the corresponding unicode character for octal 511
    assert_eq!(result.kind, LiteralKind::Octal);
    assert_eq!(result.span.start.offset, 0);
    assert_eq!(result.span.end.offset, 3);
}

