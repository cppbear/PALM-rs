// Answer 0

#[test]
fn test_is_lookaround_prefix_positive() {
    struct DummyParser {}

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position(0)),
                capture_index: Cell::new(0),
                nest_limit: 0,
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

    let parser_string = "?=";
    let parser = ParserI::new(DummyParser {}, parser_string);

    assert!(parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_negative() {
    struct DummyParser {}

    impl Borrow<Parser> for DummyParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position(0)),
                capture_index: Cell::new(0),
                nest_limit: 0,
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

    let parser_string = "abc";
    let parser = ParserI::new(DummyParser {}, parser_string);

    assert!(!parser.is_lookaround_prefix());
}

