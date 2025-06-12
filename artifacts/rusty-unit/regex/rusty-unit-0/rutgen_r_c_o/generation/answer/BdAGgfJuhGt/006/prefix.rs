// Answer 0

#[test]
fn test_bump_space_with_ignore_whitespace_enabled_and_comment_at_start() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "# This is a comment\nNext character";
    let parser_i = ParserI::new(&parser, pattern);

    parser_i.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_enabled_and_multiline_comment() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "# Comment line 1\n# Comment line 2\nNext character";
    let parser_i = ParserI::new(&parser, pattern);

    parser_i.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_enabled_and_non_comment_characters() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "# Comment\n   abc";
    let parser_i = ParserI::new(&parser, pattern);

    parser_i.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_enabled_followed_by_space_and_comments() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let pattern = "   # This is a comment\n   ";
    let parser_i = ParserI::new(&parser, pattern);

    parser_i.bump_space();
}

