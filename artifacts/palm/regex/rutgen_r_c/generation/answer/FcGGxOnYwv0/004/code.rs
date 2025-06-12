// Answer 0

#[test]
fn test_is_lookaround_prefix_not_at_lookaround() {
    struct MockParser;

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
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

    let parser = ParserI::new(MockParser, "some input text");
    assert_eq!(parser.is_lookaround_prefix(), false);
}

#[test]
fn test_is_lookaround_prefix_at_start() {
    struct MockParser {
        input: String,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(1),
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

    let mut parser = ParserI::new(MockParser { input: "?=<test>".to_string() }, "?=<test>");
    parser.bump(); // move past the '?'
    assert_eq!(parser.is_lookaround_prefix(), true);
} 

#[test]
fn test_is_lookaround_prefix_invalid() {
    struct MockParser {
        input: String,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(1),
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

    let parser = ParserI::new(MockParser { input: "?<another>".to_string() }, "?<another>");
    assert_eq!(parser.is_lookaround_prefix(), true);
} 

#[test]
fn test_is_lookaround_prefix_empty_string() {
    struct MockParser {
        input: String,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position::default()),
                capture_index: Cell::new(1),
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

    let parser = ParserI::new(MockParser { input: "".to_string() }, "");
    assert_eq!(parser.is_lookaround_prefix(), false);
}

