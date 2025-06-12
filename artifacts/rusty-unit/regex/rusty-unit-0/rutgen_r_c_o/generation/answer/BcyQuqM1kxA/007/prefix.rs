// Answer 0

#[test]
fn test_parse_flag_case_insensitive() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
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
        pattern: "i",
    };

    parser.parse_flag();
}

#[test]
fn test_parse_flag_multi_line() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
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
        pattern: "m",
    };

    parser.parse_flag();
}

#[test]
fn test_parse_flag_dot_matches_new_line() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
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
        pattern: "s",
    };

    parser.parse_flag();
}

#[test]
fn test_parse_flag_swap_greed() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
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
        pattern: "U",
    };

    parser.parse_flag();
}

#[test]
fn test_parse_flag_unicode() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
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
        pattern: "u",
    };

    parser.parse_flag();
}

#[test]
fn test_parse_flag_ignore_whitespace() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
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
        pattern: "x",
    };

    parser.parse_flag();
}

#[test]
#[should_panic]
fn test_parse_flag_unrecognized() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
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
        pattern: "a",
    };

    parser.parse_flag();
}

