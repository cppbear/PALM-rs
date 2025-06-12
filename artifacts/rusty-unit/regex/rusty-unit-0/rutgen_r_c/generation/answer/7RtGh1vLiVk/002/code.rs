// Answer 0

#[test]
fn test_push_alternate_valid() {
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

    let parser_i = ParserI::new(&parser, "|abc|def");
    let initial_concat = ast::Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 4, line: 1, column: 5 } },
        asts: vec![Ast::Literal(ast::Literal::from_string("abc"))],
    };

    let result = parser_i.push_alternate(initial_concat).unwrap();

    assert_eq!(result.span.start.offset, 0);
    assert_eq!(result.asts.len(), 0);
}

#[test]
#[should_panic]
fn test_push_alternate_invalid_char() {
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

    let parser_i = ParserI::new(&parser, "abc|def");
    let invalid_concat = ast::Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 3, line: 1, column: 4 } },
        asts: vec![Ast::Literal(ast::Literal::from_string("abc"))],
    };

    parser_i.push_alternate(invalid_concat).unwrap();
}

