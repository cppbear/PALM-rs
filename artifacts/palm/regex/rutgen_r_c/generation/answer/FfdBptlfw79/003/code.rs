// Answer 0

#[test]
fn test_parse_primitive_start_line() {
    struct MockParser;

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(0),
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

    let parser = ParserI {
        parser: MockParser,
        pattern: "^abc",
    };

    let result = parser.parse_primitive();
    match result {
        Ok(Primitive::Assertion(assertion)) => {
            assert_eq!(assertion.kind, AssertionKind::StartLine);
        }
        _ => panic!("Expected an assertion for start of line"),
    }
}

#[test]
fn test_parse_primitive_end_line() {
    struct MockParser;

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(0),
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

    let parser = ParserI {
        parser: MockParser,
        pattern: "$abc",
    };

    let result = parser.parse_primitive();
    match result {
        Ok(Primitive::Assertion(assertion)) => {
            assert_eq!(assertion.kind, AssertionKind::EndLine);
        }
        _ => panic!("Expected an assertion for end of line"),
    }
}

