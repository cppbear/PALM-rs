// Answer 0

#[test]
fn test_peek_eof_empty_pattern() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0 }),
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
    };
    let parser_i = ParserI::new(&parser, "");
    let _ = parser_i.peek();
}

#[test]
fn test_peek_eof_non_empty_pattern() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 5 }),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::from("abcd")),
    };
    let parser_i = ParserI::new(&parser, "abcd");
    let _ = parser_i.peek();
}

#[test]
fn test_peek_eof_at_pattern_boundary() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 4 }),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::from("abc")),
    };
    let parser_i = ParserI::new(&parser, "abc");
    let _ = parser_i.peek();
}

#[test]
fn test_peek_eof_at_max_length() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 1000 }),
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
    };
    let parser_i = ParserI::new(&parser, "x".repeat(1000).as_str());
    let _ = parser_i.peek();
}

