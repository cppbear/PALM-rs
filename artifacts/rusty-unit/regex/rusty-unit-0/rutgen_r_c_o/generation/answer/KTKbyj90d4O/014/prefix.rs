// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    let pattern = "abc>";
    let capture_index = 1;
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(pos),
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
        pattern,
    };
    let _ = parser.parse_capture_name(capture_index);
}

#[test]
#[should_panic(expected = "GroupNameEmpty")]
fn test_parse_capture_name_empty_name() {
    let pattern = "<>";
    let capture_index = 1;
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(pos),
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
        pattern,
    };
    let _ = parser.parse_capture_name(capture_index);
}

#[test]
#[should_panic(expected = "GroupNameUnexpectedEof")]
fn test_parse_capture_name_eof_before_closing() {
    let pattern = "<abc"; // No closing '>'
    let capture_index = 1;
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(pos),
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
        pattern,
    };
    let _ = parser.parse_capture_name(capture_index);
}

#[test]
#[should_panic(expected = "GroupNameInvalid")]
fn test_parse_capture_name_invalid_character() {
    let pattern = "<abc@>"; // '@' is not allowed in capture names
    let capture_index = 1;
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(pos),
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
        pattern,
    };
    let _ = parser.parse_capture_name(capture_index);
}


