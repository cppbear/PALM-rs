// Answer 0

#[test]
fn test_bump_and_bump_space_with_whitespace() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let mut parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "test  \n pattern");

    let result = parser_i.bump_and_bump_space();
}

#[test]
fn test_bump_and_bump_space_without_whitespace() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let mut parser = Parser {
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
    };
    let parser_i = ParserI::new(&parser, "testpattern");

    let result = parser_i.bump_and_bump_space();
}

#[test]
fn test_bump_and_bump_space_eof() {
    let position = Position { offset: 7, line: 1, column: 8 };
    let mut parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, " ");

    let result = parser_i.bump_and_bump_space();
}

#[test]
fn test_bump_and_bump_space_with_comments() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let mut parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "test # this is a comment\n pattern");

    let result = parser_i.bump_and_bump_space();
}

