// Answer 0

#[test]
fn test_bump_if_matches_prefix() {
    struct FakeParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl FakeParser {
        fn new(pattern: &str) -> Self {
            FakeParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_owned(),
            }
        }

        fn bump(&self) -> bool {
            if self.is_eof() {
                return false;
            }
            let mut offset = self.pos.get().offset;
            offset += self.pattern.chars().next().unwrap().len_utf8();
            self.pos.set(Position { offset, line: 1, column: 1 });
            true
        }

        fn is_eof(&self) -> bool {
            self.pos.get().offset >= self.pattern.len()
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    impl Borrow<FakeParser> for FakeParser {
        fn borrow(&self) -> &FakeParser {
            self
        }
    }

    let parser = FakeParser::new("hello world");
    let parser_ref = &parser;
    let parser_instance = ParserI::new(parser_ref, parser.pattern());

    assert!(parser_instance.bump_if("hello"));
    assert_eq!(parser_instance.offset(), 5); // "hello" has 5 characters
}

#[test]
fn test_bump_if_does_not_match() {
    struct FakeParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl FakeParser {
        fn new(pattern: &str) -> Self {
            FakeParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_owned(),
            }
        }

        fn bump(&self) -> bool {
            if self.is_eof() {
                return false;
            }
            let mut offset = self.pos.get().offset;
            offset += self.pattern.chars().next().unwrap().len_utf8();
            self.pos.set(Position { offset, line: 1, column: 1 });
            true
        }

        fn is_eof(&self) -> bool {
            self.pos.get().offset >= self.pattern.len()
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    impl Borrow<FakeParser> for FakeParser {
        fn borrow(&self) -> &FakeParser {
            self
        }
    }

    let parser = FakeParser::new("hello world");
    let parser_ref = &parser;
    let parser_instance = ParserI::new(parser_ref, parser.pattern());

    assert!(!parser_instance.bump_if("world")); // Should return false.
    assert_eq!(parser_instance.offset(), 0); // Offset should remain the same.
}

