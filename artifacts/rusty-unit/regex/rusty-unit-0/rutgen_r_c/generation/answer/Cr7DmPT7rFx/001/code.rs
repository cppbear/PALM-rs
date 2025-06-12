// Answer 0

#[test]
fn test_bump_eof() {
    struct DummyParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl DummyParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Cell::new(Position { offset: pattern.len(), line: 1, column: 1 }),
                pattern: pattern.to_string(),
            }
        }

        fn pos(&self) -> Position {
            self.pos.get()
        }

        fn is_eof(&self) -> bool {
            self.pos().offset >= self.pattern.len()
        }

        fn char(&self) -> char {
            '\0' // No character to return as we are at EOF
        }

        fn bump(&self) -> bool {
            if self.is_eof() {
                return false;
            }
            // Note: The actual implementation details are irrelevant for this case
            true
        }
    }

    let parser = DummyParser::new("abc");
    assert_eq!(parser.bump(), false);
}

