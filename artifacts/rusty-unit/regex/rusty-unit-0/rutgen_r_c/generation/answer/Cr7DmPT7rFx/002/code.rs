// Answer 0

#[test]
fn test_bump_with_no_eof_and_no_newline() {
    struct MockParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> MockParser {
            MockParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
            }
        }
    }

    let mock_parser = MockParser::new("abc");
    let parser_i = ParserI {
        parser: &mock_parser,
        pattern: &mock_parser.pattern,
    };

    let result = parser_i.bump();
    assert_eq!(result, true);
    let new_pos = parser_i.pos();
    assert_eq!(new_pos.offset, 1);
    assert_eq!(new_pos.line, 1);
    assert_eq!(new_pos.column, 2);
}

#[test]
#[should_panic]
fn test_bump_should_panic_on_overflow_line() {
    struct MockParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> MockParser {
            MockParser {
                pos: Cell::new(Position { offset: 0, line: usize::MAX, column: 1 }),
                pattern: pattern.to_string(),
            }
        }
    }

    let mock_parser = MockParser::new("a\nb");
    let parser_i = ParserI {
        parser: &mock_parser,
        pattern: &mock_parser.pattern,
    };

    let _ = parser_i.bump();
}

#[test]
#[should_panic]
fn test_bump_should_panic_on_pattern_index_out_of_bounds() {
    struct MockParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> MockParser {
            MockParser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
            }
        }
    }

    let mock_parser = MockParser::new("");
    let parser_i = ParserI {
        parser: &mock_parser,
        pattern: &mock_parser.pattern,
    };

    let _ = parser_i.bump();
}

