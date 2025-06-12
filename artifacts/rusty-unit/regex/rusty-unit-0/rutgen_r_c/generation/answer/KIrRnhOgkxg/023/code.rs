// Answer 0

#[test]
fn test_maybe_parse_ascii_class_success() {
    let pattern = "[[:alpha:]]"; // valid ASCII character class
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let span_start = Span::new(start_pos, start_pos);
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_some());
    if let Some(ascii_class) = result {
        assert_eq!(ascii_class.kind, ast::ClassAsciiKind::Alpha);
    }
}

#[test]
fn test_maybe_parse_ascii_class_invalid_syntax() {
    let pattern = "[[:loower:]]"; // invalid ASCII character class
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_empty_class() {
    let pattern = "[[:]]"; // empty ASCII character class
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_not_matching() {
    let pattern = "[[:upper]"; // missing closing bracket
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: pattern,
    };

    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

