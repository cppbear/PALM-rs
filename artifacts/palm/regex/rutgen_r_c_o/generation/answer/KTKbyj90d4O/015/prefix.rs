// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    let pattern = "<name>";
    let start = Position { offset: 1, line: 1, column: 2 };
    let end = Position { offset: 5, line: 1, column: 6 };
    let capture_index = 1;

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start),
            capture_index: Cell::new(capture_index),
            nest_limit: 5,
            octal: true,
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

    parser.parse_capture_name(capture_index);
}

#[test]
fn test_parse_capture_name_empty_string() {
    let pattern = "<>";
    let start = Position { offset: 1, line: 1, column: 2 };
    let end = Position { offset: 1, line: 1, column: 2 };
    let capture_index = 0;

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start),
            capture_index: Cell::new(capture_index),
            nest_limit: 5,
            octal: true,
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

    parser.parse_capture_name(capture_index);
}

#[test]
fn test_parse_capture_name_invalid_character() {
    let pattern = "<nam3>";
    let start = Position { offset: 1, line: 1, column: 2 };
    let end = Position { offset: 5, line: 1, column: 6 };
    let capture_index = 2;

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start),
            capture_index: Cell::new(capture_index),
            nest_limit: 5,
            octal: true,
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

    parser.parse_capture_name(capture_index);
}

#[test]
#[should_panic]
fn test_parse_capture_name_eof_before_closing() {
    let pattern = "<name";
    let start = Position { offset: 1, line: 1, column: 2 };
    let capture_index = 3;

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start),
            capture_index: Cell::new(capture_index),
            nest_limit: 5,
            octal: true,
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

    parser.parse_capture_name(capture_index);
}

#[test]
fn test_parse_capture_name_valid_with_underscore() {
    let pattern = "<_validName>";
    let start = Position { offset: 1, line: 1, column: 2 };
    let end = Position { offset: 11, line: 1, column: 12 };
    let capture_index = 4;

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start),
            capture_index: Cell::new(capture_index),
            nest_limit: 5,
            octal: true,
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

    parser.parse_capture_name(capture_index);
}

