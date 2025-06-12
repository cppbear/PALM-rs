// Answer 0

#[test]
fn test_is_eof_at_start() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0 }),
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
    };
    let parser_i = ParserI::new(parser, "abc");
    let result = parser_i.is_eof();
}

#[test]
fn test_is_eof_at_end() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 3 }),
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
    };
    let parser_i = ParserI::new(parser, "abc");
    let result = parser_i.is_eof();
}

#[test]
fn test_is_eof_one_character_remaining() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 2 }),
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
    };
    let parser_i = ParserI::new(parser, "abc");
    let result = parser_i.is_eof();
}

#[test]
fn test_is_eof_empty_pattern() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0 }),
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
    };
    let parser_i = ParserI::new(parser, "");
    let result = parser_i.is_eof();
}

#[test]
fn test_is_eof_large_pattern() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 1000 }),
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
    };
    let pattern = "a".repeat(1000);
    let parser_i = ParserI::new(parser, &pattern);
    let result = parser_i.is_eof();
}

