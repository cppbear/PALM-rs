// Answer 0

#[test]
fn test_peek_space_no_whitespace() {
    let parser = Parser {
        pos: Cell::new(Position::new(0, 0)),
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

    let parser_i = ParserI::new(&parser, "abc");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_in_comment() {
    let parser = Parser {
        pos: Cell::new(Position::new(0, 0)),
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

    let parser_i = ParserI::new(&parser, "a# comment\nb");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_whitespace() {
    let parser = Parser {
        pos: Cell::new(Position::new(0, 0)),
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

    let parser_i = ParserI::new(&parser, "a \n   b");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_multiple_comments() {
    let parser = Parser {
        pos: Cell::new(Position::new(0, 0)),
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

    let parser_i = ParserI::new(&parser, "a# comment\n# another comment\nb");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_edge_eof() {
    let parser = Parser {
        pos: Cell::new(Position::new(3, 0)),
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

    let parser_i = ParserI::new(&parser, "abc");
    let result = parser_i.peek_space();
}

