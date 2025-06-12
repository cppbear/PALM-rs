// Answer 0

#[test]
fn test_char_valid_position() {
    struct TestParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> TestParser {
            TestParser {
                pos: Cell::new(Position { offset: 0 }),
                pattern: pattern.to_string(),
            }
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn char_at(&self, i: usize) -> char {
            self.pattern.as_str().chars().nth(i).unwrap()
        }
    }

    let parser = TestParser::new("abc");
    assert_eq!(parser.char_at(parser.offset()), 'a');

    // Manually setting the position to 1
    parser.pos.set(Position { offset: 1 });
    assert_eq!(parser.char_at(parser.offset()), 'b');

    // Manually setting the position to 2
    parser.pos.set(Position { offset: 2 });
    assert_eq!(parser.char_at(parser.offset()), 'c');
}

#[should_panic]
#[test]
fn test_char_out_of_bounds() {
    struct TestParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> TestParser {
            TestParser {
                pos: Cell::new(Position { offset: 0 }),
                pattern: pattern.to_string(),
            }
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn char_at(&self, i: usize) -> char {
            self.pattern.as_str().chars().nth(i).unwrap()
        }
    }

    let parser = TestParser::new("abc");
    
    // Setting position to 3, which is out of bounds
    parser.pos.set(Position { offset: 3 });
    parser.char_at(parser.offset());
}

#[should_panic]
#[test]
fn test_char_empty_pattern() {
    struct TestParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl TestParser {
        fn new(pattern: &str) -> TestParser {
            TestParser {
                pos: Cell::new(Position { offset: 0 }),
                pattern: pattern.to_string(),
            }
        }

        fn offset(&self) -> usize {
            self.pos.get().offset
        }

        fn char_at(&self, i: usize) -> char {
            self.pattern.as_str().chars().nth(i).unwrap()
        }
    }

    let parser = TestParser::new("");
    
    // Attempting to access character at position 0 in an empty pattern
    parser.pos.set(Position { offset: 0 });
    parser.char_at(parser.offset());
}

