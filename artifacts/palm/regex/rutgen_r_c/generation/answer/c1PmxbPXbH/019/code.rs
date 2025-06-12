// Answer 0

#[test]
fn test_parse_uncounted_repetition_question() {
    let pattern = "a?";
    let mut concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 2, line: 1, column: 3 }),
        asts: vec![ast::Ast::Literal(ast::Literal { span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'a' })],
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.parse_uncounted_repetition(concat.clone(), ast::RepetitionKind::ZeroOrOne);
    assert!(result.is_ok());

    let concat_result = result.unwrap();
    assert_eq!(concat_result.asts.len(), 2); // Ensure the repetition is added
}

#[test]
fn test_parse_uncounted_repetition_star() {
    let pattern = "b*";
    let mut concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 2, line: 1, column: 3 }),
        asts: vec![ast::Ast::Literal(ast::Literal { span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'b' })],
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.parse_uncounted_repetition(concat.clone(), ast::RepetitionKind::ZeroOrMore);
    assert!(result.is_ok());

    let concat_result = result.unwrap();
    assert_eq!(concat_result.asts.len(), 2); // Ensure the repetition is added
}

#[test]
fn test_parse_uncounted_repetition_plus() {
    let pattern = "c+";
    let mut concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 2, line: 1, column: 3 }),
        asts: vec![ast::Ast::Literal(ast::Literal { span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }), value: 'c' })],
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };

    let result = parser.parse_uncounted_repetition(concat.clone(), ast::RepetitionKind::OneOrMore);
    assert!(result.is_ok());

    let concat_result = result.unwrap();
    assert_eq!(concat_result.asts.len(), 2); // Ensure the repetition is added
}

#[test]
#[should_panic]
fn test_parse_uncounted_repetition_missing() {
    let pattern = "a?";
    let concat = ast::Concat {
        span: ast::Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 2, line: 1, column: 3 }),
        asts: vec![], // No preceding AST to pop
    };

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 100,
            octal: false,
            initial_ignore_whitespace: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern,
    };
    
    parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrOne).unwrap(); // This should panic
}

