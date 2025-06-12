// Answer 0

#[test]
fn test_maybe_parse_ascii_class_valid_alnum() {
    let pattern = "[[:alnum:]]";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 16,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());
}

#[test]
fn test_maybe_parse_ascii_class_invalid() {
    let pattern = "[[:loower:]]";
    let mut parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 16,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_no_colon() {
    let pattern = "[:alnum:]";
    let mut parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 16,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_empty() {
    let pattern = "[]";
    let mut parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 16,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_not_ending() {
    let pattern = "[[:alnum:]";
    let mut parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 16,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

