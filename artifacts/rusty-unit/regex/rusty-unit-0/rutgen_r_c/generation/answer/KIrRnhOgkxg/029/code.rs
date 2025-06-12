// Answer 0

#[test]
fn test_maybe_parse_ascii_class_valid() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
        pattern: "[[:alnum:]]",
    };
    
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());
    let class = result.unwrap();
    assert_eq!(class.kind, ClassAsciiKind::Alnum);
}

#[test]
fn test_maybe_parse_ascii_class_negated() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
        pattern: "[[:^digit:]]",
    };
    
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());
    let class = result.unwrap();
    assert_eq!(class.kind, ClassAsciiKind::Digit);
    assert!(class.negated);
}

#[test]
fn test_maybe_parse_ascii_class_invalid() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
        pattern: "[[:loower:]]",
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());
    let class = result.unwrap();
    // Check that the result is valid, but not a recognized ASCII class type
    assert!(class.kind != ClassAsciiKind::Alpha);
}

#[test]
fn test_maybe_parse_ascii_class_missing_close() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
        pattern: "[[:alnum:]]a[[:digit:]",
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_empty() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
        pattern: "[[:]]",
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

