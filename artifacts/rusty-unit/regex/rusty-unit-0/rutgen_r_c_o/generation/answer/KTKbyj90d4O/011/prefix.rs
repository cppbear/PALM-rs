// Answer 0

#[test]
fn test_parse_capture_name_valid() {
    let pattern = "<validName>";
    let capture_index = 0;
    let start_pos = Position { offset: 1, line: 1, column: 2 };
    let end_pos = Position { offset: 12, line: 1, column: 13 };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
            capture_index: Cell::new(capture_index),
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
        pattern,
    };
    parser.parse_capture_name(capture_index);
}

#[test]
fn test_parse_capture_name_empty() {
    let pattern = "<>";
    let capture_index = 1;
    let start_pos = Position { offset: 1, line: 1, column: 2 };
    
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
            capture_index: Cell::new(capture_index),
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
        pattern,
    };
    let _ = parser.parse_capture_name(capture_index);
}

#[test]
#[should_panic]
fn test_parse_capture_name_invalid_char() {
    let pattern = "<inv@lidName>";
    let capture_index = 2;
    let start_pos = Position { offset: 1, line: 1, column: 2 };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
            capture_index: Cell::new(capture_index),
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
        pattern,
    };
    parser.parse_capture_name(capture_index);
}

#[test]
#[should_panic]
fn test_parse_capture_name_duplicate() {
    let pattern = "<duplicateName>";
    let capture_index = 2;
    let start_pos = Position { offset: 1, line: 1, column: 2 };
    
    let mut parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
            capture_index: Cell::new(capture_index),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![ast::CaptureName {
                span: Span::new(Position { offset: 1, line: 1, column: 2 }, Position { offset: 15, line: 1, column: 16 }),
                name: "duplicateName".to_string(),
                index: 1,
            }]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    parser.parse_capture_name(capture_index);
}

#[test]
fn test_parse_capture_name_edge_case() {
    let pattern = "<AnotherValidName>";
    let capture_index = 9;
    let start_pos = Position { offset: 1, line: 1, column: 2 };
    
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
            capture_index: Cell::new(capture_index),
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
        pattern,
    };
    parser.parse_capture_name(capture_index);
}

