// Answer 0

#[test]
fn test_parse_octal_valid_input() {
    struct MockParser {
        pub octal: bool,
        pub pattern: String,
        pub pos: Position,
    }

    impl MockParser {
        fn new(octal: bool, pattern: &str) -> Self {
            MockParser {
                octal,
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.offset < self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let mut parser = MockParser::new(true, "075");
    let result = parser.parse_octal();

    let expected_start = Position { offset: 0, line: 1, column: 1 };
    let expected_end = Position { offset: 3, line: 1, column: 4 };  // 3 octal digits
    let expected_char = '5'; // Unicode scalar value for octal 075
    let expected_literal = ast::Literal {
        span: Span::new(expected_start, expected_end),
        kind: ast::LiteralKind::Octal,
        c: expected_char,
    };

    assert_eq!(result, expected_literal);
}

#[test]
#[should_panic(expected = "valid octal number")]
fn test_parse_octal_invalid_input() {
    struct MockParser {
        pub octal: bool,
        pub pattern: String,
        pub pos: Position,
    }

    impl MockParser {
        fn new(octal: bool, pattern: &str) -> Self {
            MockParser {
                octal,
                pattern: pattern.to_string(),
                pos: Position { offset: 0, line: 1, column: 1 },
            }
        }

        fn bump(&mut self) -> bool {
            self.pos.offset += 1;
            self.pos.offset < self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern[self.pos.offset..].chars().next().unwrap()
        }

        fn parser(&self) -> &Self {
            self
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let mut parser = MockParser::new(true, "1000"); // Invalid octal representation
    parser.parse_octal();
}

