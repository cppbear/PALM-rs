// Answer 0

#[test]
fn test_pop_group_end_empty_stack() {
    let span = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 0, line: 1, column: 1 },
    };
    let concat = Concat { span, asts: vec![] };
    
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
    
    let parser_instance = ParserI::new(&parser, "");
    
    let result = parser_instance.pop_group_end(concat);
    assert!(result.is_ok());
}

#[test]
fn test_pop_group_end_with_alternation() {
    let span = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 0, line: 1, column: 1 },
    };
    let concat = Concat { span, asts: vec![] };

    let alt = Alternation {
        span,
        asts: vec![],
    };

    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![GroupState::Alternation(alt)]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, "");

    let result = parser_instance.pop_group_end(concat);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_pop_group_end_with_unclosed_group() {
    let span = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 0, line: 1, column: 1 },
    };
    let concat = Concat { span, asts: vec![] };

    let group = Group {
        span,
        kind: ast::GroupKind::Default,
        ast: Box::new(Ast::Empty(span)),
    };

    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![GroupState::Group { concat: concat.clone(), group, ignore_whitespace: false }]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, "");

    let result = parser_instance.pop_group_end(concat);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_pop_group_end_with_multiple_alternations() {
    let span = Span {
        start: Position { offset: 0, line: 1, column: 1 },
        end: Position { offset: 0, line: 1, column: 1 },
    };
    let concat = Concat { span, asts: vec![] };

    let alt1 = Alternation { span, asts: vec![] };
    let alt2 = Alternation { span, asts: vec![] };

    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![GroupState::Alternation(alt1), GroupState::Alternation(alt2)]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_instance = ParserI::new(&parser, "");

    let result = parser_instance.pop_group_end(concat);
    assert!(result.is_err());
}

