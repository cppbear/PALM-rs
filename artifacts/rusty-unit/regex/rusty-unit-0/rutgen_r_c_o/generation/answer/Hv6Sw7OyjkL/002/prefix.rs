// Answer 0

#[test]
fn test_peek_non_empty_pattern() {
    let pattern = "abc";
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
    let parser_i = ParserI::new(parser.borrow(), pattern);
    parser_i.peek();
}

#[test]
fn test_peek_pattern_with_whitespace() {
    let pattern = "  abc  ";
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
    let parser_i = ParserI::new(parser.borrow(), pattern);
    parser_i.peek();
}

#[test]
fn test_peek_at_end_of_pattern() {
    let pattern = "a";
    let parser = Parser {
        pos: Cell::new(Position { offset: 1 }),
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
    let parser_i = ParserI::new(parser.borrow(), pattern);
    parser_i.peek();
}

#[test]
fn test_peek_large_pattern() {
    let pattern = "x".repeat(255);
    let parser = Parser {
        pos: Cell::new(Position { offset: 254 }),
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
    let parser_i = ParserI::new(parser.borrow(), pattern);
    parser_i.peek();
}

#[test]
fn test_peek_pattern_with_special_chars() {
    let pattern = "a.b*c?";
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
    let parser_i = ParserI::new(parser.borrow(), pattern);
    parser_i.peek();
}

