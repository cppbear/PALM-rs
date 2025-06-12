// Answer 0

#[test]
fn test_parse_set_class_single_and_operator() {
    let parser = Parser {
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
    let parser_i = ParserI {
        parser: &parser,
        pattern: "[]&]",
    };
    let _ = parser_i.parse_set_class();
}

#[test]
fn test_parse_set_class_double_and_operator() {
    let parser = Parser {
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
    let parser_i = ParserI {
        parser: &parser,
        pattern: "[a&&b]",
    };
    let _ = parser_i.parse_set_class();
}

#[test]
#[should_panic]
fn test_parse_set_class_missing_closing_bracket() {
    let parser = Parser {
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
    let parser_i = ParserI {
        parser: &parser,
        pattern: "[a&&b",
    };
    let _ = parser_i.parse_set_class();
}

#[test]
fn test_parse_set_class_both_and_operators() {
    let parser = Parser {
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
    let parser_i = ParserI {
        parser: &parser,
        pattern: "[a&&b--c]",
    };
    let _ = parser_i.parse_set_class();
}

#[test]
fn test_parse_set_class_with_empty_class() {
    let parser = Parser {
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
    let parser_i = ParserI {
        parser: &parser,
        pattern: "[&&]",
    };
    let _ = parser_i.parse_set_class();
}

#[test]
#[should_panic]
fn test_parse_set_class_invalid_range() {
    let parser = Parser {
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
    let parser_i = ParserI {
        parser: &parser,
        pattern: "[a-b]",
    };
    let _ = parser_i.parse_set_class();
}

