// Answer 0

#[test]
fn test_bump_space_with_whitespace_and_comments() {
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
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
    
    let pattern = "   # this is a comment\n   a";
    let parser_i = ParserI::new(&parser, pattern);
    
    parser_i.bump_space();
}

#[test]
fn test_bump_space_with_only_whitespace() {
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
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
fn test_bump_space_with_eof_reached_after_whitespace() {
    let pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
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
    
    let pattern = "   # this is a comment\n";
    let parser_i = ParserI::new(&parser, pattern);
    
    parser_i.bump_space();
}

