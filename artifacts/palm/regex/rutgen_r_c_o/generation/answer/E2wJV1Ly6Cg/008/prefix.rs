// Answer 0

#[test]
fn test_peek_space_no_whitespace() {
    let pattern = "abc";
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_instance = ParserI::new(&parser, pattern);
    parser_instance.peek_space();
}

#[test]
fn test_peek_space_with_whitespace_no_comments() {
    let pattern = "a b c";
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, pattern);
    parser_instance.peek_space();
}

#[test]
fn test_peek_space_with_comment() {
    let pattern = "abc # this is a comment";
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, pattern);
    parser_instance.peek_space();
}

#[test]
fn test_peek_space_with_new_line_comment() {
    let pattern = "abc # comment\nxyz";
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, pattern);
    parser_instance.peek_space();
}

#[test]
fn test_peek_space_multiple_whitespace_and_comment() {
    let pattern = "   # leading spaces\nabc   ";
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, pattern);
    parser_instance.peek_space();
}

#[test]
fn test_peek_space_comment_followed_by_newline() {
    let pattern = "abc # comment\n def";
    let parser = Parser {
        pos: Cell::new(Position::new(0)),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, pattern);
    parser_instance.peek_space();
}

