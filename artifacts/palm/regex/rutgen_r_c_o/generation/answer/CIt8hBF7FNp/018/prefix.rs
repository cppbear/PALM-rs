// Answer 0

#[test]
fn test_parse_set_class_open_negated_with_multiple_hyphens() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 5, line: 1, column: 6 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: "[^^-----]".as_ref(),
    };
    let _ = parser.parse_set_class_open();
}

#[test]
fn test_parse_set_class_open_negated_with_single_hyphen() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 3, line: 1, column: 4 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: "[^-]".as_ref(),
    };
    let _ = parser.parse_set_class_open();
}

#[test]
fn test_parse_set_class_open_non_negated_with_empty_class() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 3, line: 1, column: 4 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: "[^]]".as_ref(),
    };
    let _ = parser.parse_set_class_open();
}

#[test]
fn test_parse_set_class_open_with_hyphen_as_first_char() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 2, line: 1, column: 3 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: "[--]".as_ref(),
    };
    let _ = parser.parse_set_class_open();
}

