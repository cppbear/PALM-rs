// Answer 0

#[test]
fn test_parse_with_comments_empty_pattern() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "",
    };

    let result = parser_i.parse_with_comments();
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert_eq!(with_comments.ast, Ast::Empty(Span { start: 0, end: 0 }));
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_single_literal() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "a",
    };

    let result = parser_i.parse_with_comments();
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(matches!(with_comments.ast, Ast::Concat { .. }));
    assert!(with_comments.comments.is_empty());
}

#[test]
fn test_parse_with_comments_repetition() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "a?b",
    };

    let result = parser_i.parse_with_comments();
    assert!(result.is_ok());
    let with_comments = result.unwrap();
    assert!(matches!(with_comments.ast, Ast::Concat { .. }));
    assert!(with_comments.comments.is_empty());
}

#[test]
#[should_panic(expected = "parser can only be used once")]
fn test_parse_with_comments_invalid_multiple_use() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "a?",
    };

    let _ = parser_i.parse_with_comments(); // first call
    let _ = parser_i.parse_with_comments(); // second call, should panic
}

#[test]
fn test_parse_with_comments_unclosed_repetition() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "a{2", // Unclosed repetition-should trigger error
    };

    let result = parser_i.parse_with_comments();
    assert!(result.is_err());
}

