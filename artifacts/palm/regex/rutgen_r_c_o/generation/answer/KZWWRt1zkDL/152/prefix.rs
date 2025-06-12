// Answer 0

#[test]
fn test_parse_escape_hex_u() {
    let pattern = "\\u1234";
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 6, line: 1, column: 7 };
    let span = Span::new(start, end);
    let parser = Parser {
        pos: Cell::new(start),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_hex_x() {
    let pattern = "\\xAB";
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start, end);
    let parser = Parser {
        pos: Cell::new(start),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_hex_u_brace() {
    let pattern = "\\u{1F600}";
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 9, line: 1, column: 10 };
    let span = Span::new(start, end);
    let parser = Parser {
        pos: Cell::new(start),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };
    let _ = parser_i.parse_escape();
}

#[test]
fn test_parse_escape_hex_U() {
    let pattern = "\\U0001F600";
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 10, line: 1, column: 11 };
    let span = Span::new(start, end);
    let parser = Parser {
        pos: Cell::new(start),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };
    let _ = parser_i.parse_escape();
}

