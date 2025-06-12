// Answer 0

#[test]
fn test_parse_counted_repetition_invalid_range() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start_position, end_position);
    
    let mut concat = Concat {
        span: span,
        asts: vec![Ast::Literal(ast::Literal { span })],
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_position),
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
        },
        pattern: "{1,2}".into(),
    };

    // Set up needed state to mimic environment
    parser.parser.pos.set(start_position);
    parser.parser.scratch.replace("1".to_string());

    // Emulate necessary method behaviors
    fn bump_and_bump_space() -> bool {
        true
    }
    fn is_eof() -> bool {
        false
    }
    
    // Bumping a space should always be true in this test case
    let original_bump_and_bump_space = parser.bump_and_bump_space;
    parser.bump_and_bump_space = bump_and_bump_space;

    // Call the function with the test input
    let _ = parser.parse_counted_repetition(concat);

    // Reset the bump_and_bump_space function after test
    parser.bump_and_bump_space = original_bump_and_bump_space;
}

#[test]
fn test_parse_counted_repetition_with_invalid_eof() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 6, line: 1, column: 7 };
    let span = Span::new(start_position, end_position);

    let mut concat = Concat {
        span: span,
        asts: vec![Ast::Literal(ast::Literal { span })],
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_position),
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
        },
        pattern: "{123,456}".into(),
    };

    // Set up needed state to mimic environment
    parser.parser.pos.set(start_position);
    parser.parser.scratch.replace("123".to_string());

    // Call the function with the test input
    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_unexpected_char() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 7, line: 1, column: 8 };
    let span = Span::new(start_position, end_position);

    let mut concat = Concat {
        span: span,
        asts: vec![Ast::Literal(ast::Literal { span })],
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_position),
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
        },
        pattern: "{5,*}".into(),
    };

    // Set up needed state to mimic environment
    parser.parser.pos.set(start_position);
    parser.parser.scratch.replace("5".to_string());

    // Call the function with the test input
    let _ = parser.parse_counted_repetition(concat);
}

