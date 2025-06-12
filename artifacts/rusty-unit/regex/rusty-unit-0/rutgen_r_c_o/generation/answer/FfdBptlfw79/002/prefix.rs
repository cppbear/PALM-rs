// Answer 0

#[test]
fn test_parse_primitive_dollar() {
    let parser = ParserI {
        parser: Parser {
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
        },
        pattern: "$abc",
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_caret() {
    let parser = ParserI {
        parser: Parser {
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
        },
        pattern: "^abc",
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape() {
    let parser = ParserI {
        parser: Parser {
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
        },
        pattern: "\\abc",
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_dot() {
    let parser = ParserI {
        parser: Parser {
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
        },
        pattern: ".abc",
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_literal_a() {
    let parser = ParserI {
        parser: Parser {
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
        },
        pattern: "aabc",
    };
    let _ = parser.parse_primitive();
}

