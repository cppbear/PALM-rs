// Answer 0

#[test]
fn test_peek_space_whitespace_insensitive_with_non_whitespace_character() {
    let pattern = "   # comment\nabc";
    let parser = Parser {
        pos: Cell::new(Position { offset: 0 }),
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
    let parser_i = ParserI::new(&parser, pattern);
    let _result = parser_i.peek_space();
}

#[test]
fn test_peek_space_after_whitespace_and_comment() {
    let pattern = "   # comment\n  \nxyz";
    let parser = Parser {
        pos: Cell::new(Position { offset: 7 }),
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
    let parser_i = ParserI::new(&parser, pattern);
    let _result = parser_i.peek_space();
}

#[test]
fn test_peek_space_with_no_leading_whitespace() {
    let pattern = "abc # comment";
    let parser = Parser {
        pos: Cell::new(Position { offset: 0 }),
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
    let parser_i = ParserI::new(&parser, pattern);
    let _result = parser_i.peek_space();
}

#[test]
fn test_peek_space_with_multiple_comments() {
    let pattern = "# first comment\n  # second comment\ndef";
    let parser = Parser {
        pos: Cell::new(Position { offset: 0 }),
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
    let parser_i = ParserI::new(&parser, pattern);
    let _result = parser_i.peek_space();
} 

#[test]
fn test_peek_space_with_subsequent_non_whitespace_character() {
    let pattern = "   # comment\n   \n\n!@#";
    let parser = Parser {
        pos: Cell::new(Position { offset: 9 }),
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
    let parser_i = ParserI::new(&parser, pattern);
    let _result = parser_i.peek_space();
}

