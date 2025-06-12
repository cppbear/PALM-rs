// Answer 0

#[test]
fn test_peek_space_no_whitespace() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 1 }),
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

    let parser_i = ParserI::new(&parser, "abc");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_with_non_whitespace() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 1 }),
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

    let parser_i = ParserI::new(&parser, "a# comment\nb");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_eof_condition() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 3 }),
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

    let parser_i = ParserI::new(&parser, "abc");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_at_start() {
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

    let parser_i = ParserI::new(&parser, "abc");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_ignore_whitespace_false() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 2 }),
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

    let parser_i = ParserI::new(&parser, "xyz");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_non_empty_pattern() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 1 }),
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

    let parser_i = ParserI::new(&parser, "x");
    let result = parser_i.peek_space();
}

