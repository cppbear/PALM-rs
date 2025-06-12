// Answer 0

#[test]
fn test_parse_set_class_with_intersection() {
    let pattern = "[a-z&&[b-f]]";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0, 0)),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    let result = parser.parse_set_class();
    assert!(result.is_ok());

    let class = result.unwrap();
    match class {
        ast::Class::Bracketed(_) => assert!(true),
        _ => assert!(false, "Expected Bracketed class type"),
    }
}

#[test]
fn test_parse_set_class_with_empty_intersection() {
    let pattern = "[a-z&&]";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0, 0)),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    let result = parser.parse_set_class();
    assert!(result.is_err());
}

#[test]
fn test_parse_set_class_with_range() {
    let pattern = "[a-z-]";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0, 0)),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    let result = parser.parse_set_class();
    assert!(result.is_err());
}

// Additional test that confirms the failure condition for the constraints
#[test]
#[should_panic(expected = "unclosed class error")]
fn test_parse_set_class_should_panic_unclosed_class() {
    let pattern = "[a-z";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0, 0)),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    parser.parse_set_class().unwrap(); // This should panic due to unclosed class
}

