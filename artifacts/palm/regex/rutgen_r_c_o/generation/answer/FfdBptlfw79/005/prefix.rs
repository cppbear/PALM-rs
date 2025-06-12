// Answer 0

#[test]
fn test_parse_primitive_escape_octal_valid() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 0,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "\\0",
    };
    parser.parse_primitive();
}

#[test]
#[should_panic]
fn test_parse_primitive_escape_octal_invalid() {
    let parser = ParserI {
        parser: Parser {
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
        },
        pattern: "\\8",
    };
    parser.parse_primitive();
}

#[test]
#[should_panic]
fn test_parse_primitive_escape_invalid_x() {
    let parser = ParserI {
        parser: Parser {
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
        },
        pattern: "\\x",
    };
    parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape_hex_valid() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 0,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "\\x61",
    };
    parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape_unicode_valid() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 0,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "\\u0041",
    };
    parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape_perl_class() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 0,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "\\d",
    };
    parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape_assertion_valid() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 0,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "\\b",
    };
    parser.parse_primitive();
}

