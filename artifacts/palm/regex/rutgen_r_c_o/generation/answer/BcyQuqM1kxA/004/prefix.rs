// Answer 0

#[test]
fn test_parse_flag_case_swap_greed() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern: "U",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_case_insensitive() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern: "i",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_case_multiline() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern: "m",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_case_dot_matches_new_line() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern: "s",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_case_unicode() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern: "u",
    };
    let _ = parser.parse_flag();
}

#[test]
fn test_parse_flag_case_ignore_whitespace() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(Vec::new()),
            stack_group: RefCell::new(Vec::new()),
            stack_class: RefCell::new(Vec::new()),
            capture_names: RefCell::new(Vec::new()),
            scratch: RefCell::new(String::new()),
        },
        pattern: "x",
    };
    let _ = parser.parse_flag();
}

