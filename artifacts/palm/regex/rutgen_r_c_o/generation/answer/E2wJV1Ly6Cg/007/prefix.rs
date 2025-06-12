// Answer 0

#[test]
fn test_peek_space_with_ignore_whitespace_true_and_comment() {
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
    
    let parser_i = ParserI::new(&parser, "   # comment\n next_character");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_with_ignore_whitespace_true_no_comment_after_space() {
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
    
    let parser_i = ParserI::new(&parser, "   no_comment_here");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_with_ignore_whitespace_true_multiple_spaces_followed_by_comment() {
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
    
    let parser_i = ParserI::new(&parser, "      # this is a comment");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_with_ignore_whitespace_true_pushing_before_comment() {
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
    
    let parser_i = ParserI::new(&parser, "  # A comment starts here");
    let result = parser_i.peek_space();
}

#[test]
fn test_peek_space_with_ignore_whitespace_true_spaces_and_no_end_character() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 1 }),
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
    
    let parser_i = ParserI::new(&parser, " ");
    let result = parser_i.peek_space();
}

