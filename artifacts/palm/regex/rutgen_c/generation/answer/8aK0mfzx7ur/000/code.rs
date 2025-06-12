// Answer 0

#[test]
fn test_pop_group_end_no_state() {
    let concat = Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } },
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
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(parser, "");
    let result = parser_i.pop_group_end(concat);
    assert!(result.is_ok());
}

#[test]
fn test_pop_group_end_with_alternation() {
    let concat = Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } },
        asts: vec![],
    };
    
    let group = Group {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } },
        kind: GroupKind::Normal,
        ast: Box::new(Ast::Empty(Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } })),
    };

    let alternation = Alternation {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } },
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
        stack_group: RefCell::new(vec![GroupState::Alternation(alternation), GroupState::Group { group, ignore_whitespace: false }]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(parser, "");
    let result = parser_i.pop_group_end(concat);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_pop_group_end_with_unclosed_group() {
    let concat = Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } },
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
        stack_group: RefCell::new(vec![GroupState::Group { group: Group { span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } }, kind: GroupKind::Normal, ast: Box::new(Ast::Empty(Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } })) }, ignore_whitespace: false }]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(parser, "");
    parser_i.pop_group_end(concat);
}

