// Answer 0

#[test]
fn test_bump_space_with_ignore_whitespace_enabled() {
    // Create the starting position.
    let start_position = Position { offset: 0, line: 1, column: 1 };

    // Initialize Parser with ignore_whitespace set to true.
    let parser = Parser {
        pos: Cell::new(start_position),
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

    // Create a ParserI instance with a pattern that contains whitespace and a comment.
    let pattern = "    # This is a comment\n   a";
    let parser_i = ParserI::new(&parser, pattern);
    
    // Call bump_space without any external context.
    parser_i.bump_space();

    // Validate that the parser position has advanced past whitespaces and comments.
    let expected_position = Position { offset: 15, line: 2, column: 4 };
    assert_eq!(parser_i.pos(), expected_position);

    // Check if the comment has been captured correctly.
    let comments = parser.comments.borrow();
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].comment, "This is a comment");
    assert!(comments[0].span.is_one_line());
}

#[test]
fn test_bump_space_with_ignore_whitespace_disabled() {
    // Create the starting position.
    let start_position = Position { offset: 0, line: 1, column: 1 };

    // Initialize Parser with ignore_whitespace set to false.
    let parser = Parser {
        pos: Cell::new(start_position),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: true,
        initial_ignore_whitespace: true,
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    // Create a ParserI instance with a pattern.
    let pattern = "    # This is a comment\n   a";
    let parser_i = ParserI::new(&parser, pattern);
    
    // Call bump_space when ignore_whitespace is disabled.
    parser_i.bump_space();

    // Validate that the parser position remains the same since bump_space should do nothing.
    assert_eq!(parser_i.pos(), start_position);

    // Validate that no comments have been captured.
    let comments = parser.comments.borrow();
    assert_eq!(comments.len(), 0);
}

