// Answer 0

#[test]
fn test_parse_octal_valid() {
    struct MockParser {
        pos: Cell<Position>,
        pattern: String,
        octal: bool,
    }

    impl MockParser {
        fn new(octal: bool, pattern: &str) -> Self {
            MockParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
                octal,
            }
        }

        fn bump(&self) {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: self.pos.get().column + 1 });
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    impl ParserI<'_, MockParser> {
        fn new(parser: MockParser, pattern: &str) -> Self {
            ParserI {
                parser,
                pattern,
            }
        }
    }

    let mock_parser = MockParser::new(true, "077");
    let parser_i = ParserI::new(mock_parser, "077");
    let literal = parser_i.parse_octal();

    assert_eq!(literal.kind, LiteralKind::Octal);
    assert_eq!(literal.c, 'w'); // 'w' is the character for octal 077
}

#[test]
#[should_panic]
fn test_parse_octal_invalid_character() {
    struct MockParser {
        pos: Cell<Position>,
        pattern: String,
        octal: bool,
    }

    impl MockParser {
        fn new(octal: bool, pattern: &str) -> Self {
            MockParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
                octal,
            }
        }

        fn bump(&self) {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: self.pos.get().column + 1 });
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    impl ParserI<'_, MockParser> {
        fn new(parser: MockParser, pattern: &str) -> Self {
            ParserI {
                parser,
                pattern,
            }
        }
    }

    let mock_parser = MockParser::new(true, "8");
    let parser_i = ParserI::new(mock_parser, "8");
    parser_i.parse_octal(); // This should panic due to invalid octal digit.
}

#[test]
fn test_parse_octal_three_digits() {
    struct MockParser {
        pos: Cell<Position>,
        pattern: String,
        octal: bool,
    }

    impl MockParser {
        fn new(octal: bool, pattern: &str) -> Self {
            MockParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
                octal,
            }
        }

        fn bump(&self) {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: self.pos.get().column + 1 });
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    impl ParserI<'_, MockParser> {
        fn new(parser: MockParser, pattern: &str) -> Self {
            ParserI {
                parser,
                pattern,
            }
        }
    }

    let mock_parser = MockParser::new(true, "512");
    let parser_i = ParserI::new(mock_parser, "512");
    let literal = parser_i.parse_octal();

    assert_eq!(literal.kind, LiteralKind::Octal);
    assert_eq!(literal.c, 'ƈ'); // Character for octal 512
}

#[test]
#[should_panic]
fn test_parse_octal_over_three_digits() {
    struct MockParser {
        pos: Cell<Position>,
        pattern: String,
        octal: bool,
    }

    impl MockParser {
        fn new(octal: bool, pattern: &str) -> Self {
            MockParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
                octal,
            }
        }

        fn bump(&self) {
            self.pos.set(Position { offset: self.pos.get().offset + 1, line: 1, column: self.pos.get().column + 1 });
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos.get().offset).unwrap()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    impl ParserI<'_, MockParser> {
        fn new(parser: MockParser, pattern: &str) -> Self {
            ParserI {
                parser,
                pattern,
            }
        }
    }

    let mock_parser = MockParser::new(true, "5123");
    let parser_i = ParserI::new(mock_parser, "5123");
    parser_i.parse_octal(); // This should panic due to exceeding three digits.
}

