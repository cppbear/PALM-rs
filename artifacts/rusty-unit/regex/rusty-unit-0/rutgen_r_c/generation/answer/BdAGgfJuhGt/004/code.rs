// Answer 0

#[test]
fn test_bump_space_with_no_whitespace_or_comments() {
    // Setup pattern with non-whitespace characters
    let pattern = "abc";
    let mut pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 0,
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
    
    parser_i.bump_space();
    
    // Verify that position remains the same
    assert_eq!(parser_i.pos(), pos);
}

#[test]
fn test_bump_space_with_leading_whitespace() {
    // Setup pattern with leading whitespace
    let pattern = "   abc";
    let mut pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 0,
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
    
    // Bump for the leading whitespace
    for _ in 0..3 {
        parser_i.bump();
    }
    
    parser_i.bump_space();
    
    // Verify that position has advanced
    pos.offset += 3; // Moving past the 3 spaces
    assert_eq!(parser_i.pos(), pos);
}

#[test]
fn test_bump_space_with_trailing_comments() {
    // Setup pattern with whitespace followed by a comment
    let pattern = "   abc # This is a comment\n";
    let mut pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 0,
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

    // Bump space to handle leading whitespace and comments
    parser_i.bump_space();

    // Verify that position has moved past whitespace and comment
    pos.offset += 3; // past spaces
    pos.column += 3; // remains on the same line after spaces
    assert_eq!(parser_i.pos(), pos);
    assert_eq!(parser.comments.borrow().len(), 1); // Confirm comment was added
}

#[test]
fn test_bump_space_with_no_effect_when_ignore_whitespace_false() {
    // Setup parser with ignore_whitespace set to false
    let pattern = "   abc";
    let mut pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, pattern);
    
    parser_i.bump_space();
    
    // Verify that position remains the same
    assert_eq!(parser_i.pos(), pos);
}

