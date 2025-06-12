// Answer 0

#[test]
fn test_parse_capture_name_success_case() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    let pattern = "<name>";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(position),
            capture_index: Cell::new(0),
            nest_limit: 5,
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
    let _ = parser.parse_capture_name(0);
}

#[test]
#[should_panic(expected = "GroupNameUnexpectedEof")]
fn test_parse_capture_name_unexpected_eof() {
    let position = Position { offset: 1, line: 1, column: 2 };
    let pattern = "<";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(position),
            capture_index: Cell::new(0),
            nest_limit: 5,
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
    let _ = parser.parse_capture_name(0);
}

#[test]
#[should_panic(expected = "GroupNameInvalid")]
fn test_parse_capture_name_invalid_char() {
    let position = Position { offset: 1, line: 1, column: 2 };
    let pattern = "<123>";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(position),
            capture_index: Cell::new(1),
            nest_limit: 5,
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
    let _ = parser.parse_capture_name(1);
}

#[test]
#[should_panic(expected = "GroupNameEmpty")]
fn test_parse_capture_name_empty_name() {
    let position = Position { offset: 1, line: 1, column: 2 };
    let pattern = "<>";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(position),
            capture_index: Cell::new(0),
            nest_limit: 5,
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
    let _ = parser.parse_capture_name(0);
}

#[test]
fn test_parse_capture_name_multiple_valid_names() {
    let pattern = "<name1><name2><name3>";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 5,
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
    let _ = parser.parse_capture_name(0);  // name1
    parser.parser.pos.set(Position { offset: 6, line: 1, column: 7 });
    let _ = parser.parse_capture_name(1);  // name2
    parser.parser.pos.set(Position { offset: 12, line: 1, column: 13 });
    let _ = parser.parse_capture_name(2);  // name3
}

