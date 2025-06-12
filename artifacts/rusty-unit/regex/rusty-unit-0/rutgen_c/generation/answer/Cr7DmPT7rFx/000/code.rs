// Answer 0

#[test]
fn test_bump_next_unicode_scalar() {
    struct MockParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
            }
        }
    }

    let mock_pattern = "abc\n123";
    let parser = MockParser::new(mock_pattern);
    let parser_instance = ParserI::new(&parser, &parser.pattern);

    assert_eq!(parser_instance.bump(), true);
    assert_eq!(parser_instance.offset(), 1);
    assert_eq!(parser_instance.line(), 1);
    assert_eq!(parser_instance.column(), 2);

    assert_eq!(parser_instance.bump(), true);
    assert_eq!(parser_instance.offset(), 2);
    assert_eq!(parser_instance.line(), 1);
    assert_eq!(parser_instance.column(), 3);

    assert_eq!(parser_instance.bump(), true);
    assert_eq!(parser_instance.offset(), 3);
    assert_eq!(parser_instance.line(), 1);
    assert_eq!(parser_instance.column(), 4);

    assert_eq!(parser_instance.bump(), true);
    assert_eq!(parser_instance.offset(), 4);
    assert_eq!(parser_instance.line(), 2);
    assert_eq!(parser_instance.column(), 1);

    assert_eq!(parser_instance.bump(), true);
    assert_eq!(parser_instance.offset(), 5);
    assert_eq!(parser_instance.line(), 2);
    assert_eq!(parser_instance.column(), 2);

    assert_eq!(parser_instance.bump(), true);
    assert_eq!(parser_instance.offset(), 6);
    assert_eq!(parser_instance.line(), 2);
    assert_eq!(parser_instance.column(), 3);

    assert_eq!(parser_instance.bump(), false);
}

#[test]
fn test_bump_eof() {
    struct MockParser {
        pos: Cell<Position>,
        pattern: String,
    }

    impl MockParser {
        fn new(pattern: &str) -> Self {
            Self {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                pattern: pattern.to_string(),
            }
        }
    }

    let mock_pattern = "a";
    let parser = MockParser::new(mock_pattern);
    let parser_instance = ParserI::new(&parser, &parser.pattern);

    assert_eq!(parser_instance.bump(), true); // First bump
    assert_eq!(parser_instance.bump(), false); // At EOF
}

