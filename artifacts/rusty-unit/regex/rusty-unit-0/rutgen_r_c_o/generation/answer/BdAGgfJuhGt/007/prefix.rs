// Answer 0

#[test]
fn test_bump_space_no_whitespace_when_ignore_whitespace_false() {
    let starting_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(starting_position),
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
    let pattern = " 5,6";
    let parser_i = ParserI::new(&parser, pattern);

    parser_i.bump_space();
}

#[test]
fn test_bump_space_with_whitespace_when_ignore_whitespace_true() {
    let starting_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(starting_position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "  # a comment\n 5,6";
    let parser_i = ParserI::new(&parser, pattern);

    parser_i.bump_space();
}

#[test]
fn test_bump_space_when_eof_reached() {
    let starting_position = Position { offset: 6, line: 1, column: 7 };
    let parser = Parser {
        pos: Cell::new(starting_position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "   ";
    let parser_i = ParserI::new(&parser, pattern);

    parser_i.bump_space();
}

#[test]
fn test_bump_space_single_comment() {
    let starting_position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(starting_position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "# This is a comment\n5,6";
    let parser_i = ParserI::new(&parser, pattern);

    parser_i.bump_space();
}

