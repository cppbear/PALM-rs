// Answer 0

#[test]
fn test_parse_set_class_simple() {
    let pattern = "[a-z]";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern,
    };

    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_nested() {
    let pattern = "[a-z[0-9]]";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern,
    };

    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_intersection() {
    let pattern = "[a-z&&[A-Z]]";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern,
    };

    parser.parse_set_class();
}

#[test]
#[should_panic]
fn test_parse_set_class_unclosed() {
    let pattern = "[a-z";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern,
    };

    parser.parse_set_class();
}

#[test]
fn test_parse_set_class_difference() {
    let pattern = "[a-z--[0-9]]";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::new(0)),
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
        },
        pattern,
    };

    parser.parse_set_class();
}

