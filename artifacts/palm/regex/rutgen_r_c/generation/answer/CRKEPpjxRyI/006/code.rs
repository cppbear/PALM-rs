// Answer 0

#[test]
fn test_parse_set_class_range_valid_range() {
    struct MockParser;
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser_i = ParserI {
        parser: MockParser,
        pattern: "a-z",
    };
    
    let result = parser_i.parse_set_class_range();
    assert!(result.is_ok());
    if let Ok(ast::ClassSetItem::Range(range)) = result {
        assert_eq!(range.start.c, 'a');
        assert_eq!(range.end.c, 'z');
        assert!(range.is_valid());
    }
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    struct MockParserInvalid;
    impl Borrow<Parser> for MockParserInvalid {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser_i_invalid = ParserI {
        parser: MockParserInvalid,
        pattern: "z-a",
    };

    let result = parser_i_invalid.parse_set_class_range();
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_parse_set_class_range_unclosed() {
    struct MockParserUnclosed;
    impl Borrow<Parser> for MockParserUnclosed {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(vec![]),
                stack_group: RefCell::new(vec![]),
                stack_class: RefCell::new(vec![]),
                capture_names: RefCell::new(vec![]),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser_i_unclosed = ParserI {
        parser: MockParserUnclosed,
        pattern: "a-",
    };

    let _ = parser_i_unclosed.parse_set_class_range(); // Should panic due to unclosed class
}

