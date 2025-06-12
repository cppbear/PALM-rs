// Answer 0

#[test]
fn test_bump_and_bump_space_no_space() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };
    
    let pattern = "abc";
    let parser_i = ParserI::new(parser, pattern);
    
    assert_eq!(parser_i.bump_and_bump_space(), true);
    assert_eq!(parser_i.offset(), 1);
}

#[test]
fn test_bump_and_bump_space_with_space() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };

    let pattern = "a b";
    let parser_i = ParserI::new(parser, pattern);
    
    assert_eq!(parser_i.bump_and_bump_space(), true);
    assert_eq!(parser_i.offset(), 2);
}

#[test]
fn test_bump_and_bump_space_eof() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(Vec::new()),
        stack_group: RefCell::new(Vec::new()),
        stack_class: RefCell::new(Vec::new()),
        capture_names: RefCell::new(Vec::new()),
        scratch: RefCell::new(String::new()),
    };

    let pattern = ""; // Empty pattern
    let parser_i = ParserI::new(parser, pattern);
    
    assert_eq!(parser_i.bump_and_bump_space(), false);
}

