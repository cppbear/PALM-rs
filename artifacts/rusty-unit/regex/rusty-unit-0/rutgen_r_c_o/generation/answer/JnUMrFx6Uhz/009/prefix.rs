// Answer 0

#[test]
fn test_parse_flags_duplicate_case_insensitive() {
    let parser = ParserI {
        parser: RefCell::new(Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
        }),
        pattern: "---im",
    };
    
    let _ = parser.parse_flags();
}

#[test]
fn test_parse_flags_duplicate_multi_line() {
    let parser = ParserI {
        parser: RefCell::new(Parser {
            pos: Cell::new(Position { offset: 5, line: 1, column: 6 }),
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
        }),
        pattern: "---m",
    };
    
    let _ = parser.parse_flags();
}

#[test]
fn test_parse_flags_duplicate_negation() {
    let parser = ParserI {
        parser: RefCell::new(Parser {
            pos: Cell::new(Position { offset: 10, line: 1, column: 11 }),
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
        }),
        pattern: "---i-i",
    };
    
    let _ = parser.parse_flags();
}

#[test]
fn test_parse_flags_invalid_case() {
    let parser = ParserI {
        parser: RefCell::new(Parser {
            pos: Cell::new(Position { offset: 15, line: 1, column: 16 }),
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
        }),
        pattern: "---x--",
    };
    
    let _ = parser.parse_flags();
}

