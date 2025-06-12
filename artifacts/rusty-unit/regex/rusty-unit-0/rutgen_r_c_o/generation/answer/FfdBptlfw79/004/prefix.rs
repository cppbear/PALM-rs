// Answer 0

#[test]
fn test_parse_primitive_dot() {
    let parser_input = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_instance = ParserI {
        parser: &parser_input,
        pattern: ".",
    };
    parser_instance.parse_primitive();
}

#[test]
fn test_parse_primitive_start_assertion() {
    let parser_input = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_instance = ParserI {
        parser: &parser_input,
        pattern: "^",
    };
    parser_instance.parse_primitive();
}

#[test]
fn test_parse_primitive_end_assertion() {
    let parser_input = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_instance = ParserI {
        parser: &parser_input,
        pattern: "$",
    };
    parser_instance.parse_primitive();
}

#[test]
fn test_parse_primitive_literal() {
    let parser_input = Parser {
        pos: Cell::new(0),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_instance = ParserI {
        parser: &parser_input,
        pattern: "a",
    };
    parser_instance.parse_primitive();
}

