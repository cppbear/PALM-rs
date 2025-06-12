// Answer 0

#[test]
fn test_parse_escape_case_p() {
    let pattern = r"\p";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(true),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: pattern,
    };
    let _ = parser.parse_escape();
}

#[test]
fn test_parse_escape_case_P() {
    let pattern = r"\P";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(true),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: pattern,
    };
    let _ = parser.parse_escape();
}

#[test]
#[should_panic]
fn test_parse_escape_case_invalid_unicode_class() {
    let pattern = r"\p{invalid}";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(true),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: pattern,
    };
    let _ = parser.parse_escape();
}

