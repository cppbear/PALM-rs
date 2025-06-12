// Answer 0

#[test]
fn test_maybe_parse_ascii_class_invalid_name() {
    let pattern = "[[:loower:]]";
    let start = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start),
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
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_missing_closing_bracket() {
    let pattern = "[[:lower:";
    let start = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start),
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
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_no_initial_colon() {
    let pattern = "[lower:]]";
    let start = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start),
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
    parser.maybe_parse_ascii_class();
}

#[test]
fn test_maybe_parse_ascii_class_invalid_outer_structure() {
    let pattern = "[[:lower:]A]";
    let start = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start),
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
    parser.maybe_parse_ascii_class();
}

