// Answer 0

#[test]
fn test_parse_set_class_range_valid() {
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

    let parser_instance = ParserI {
        parser: MockParser,
        pattern: "a-b",
    };

    let result = parser_instance.parse_set_class_range();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_set_class_range_invalid() {
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

    let parser_instance = ParserI {
        parser: MockParser,
        pattern: "b-a", // Invalid range
    };

    let _ = parser_instance.parse_set_class_range(); // This should panic due to ClassRangeInvalid
}

