// Answer 0

#[test]
fn test_pop_group_end_with_empty_stack() {
    let span = ast::Span { start: ast::Position { offset: 0, line: 1, column: 1 }, end: ast::Position { offset: 0, line: 1, column: 1 } };
    let concat = ast::Concat { span, asts: vec![] };
    
    let parser = std::cell::RefCell::new(Parser {
        pos: Cell::new(ast::Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new("".to_string()),
    });

    let parser_instance = ParserI::new(&parser, "");
    
    let result = parser_instance.pop_group_end(concat);
    assert!(result.is_ok());
}

#[test]
fn test_pop_group_end_with_single_alternation_on_stack() {
    let span = ast::Span { start: ast::Position { offset: 0, line: 1, column: 1 }, end: ast::Position { offset: 0, line: 1, column: 1 } };
    let concat = ast::Concat { span, asts: vec![] };

    let alternation_span = ast::Span { start: ast::Position { offset: 0, line: 1, column: 1 }, end: ast::Position { offset: 1, line: 1, column: 2 } };
    let alternation = ast::Alternation { span: alternation_span, asts: vec![] };
    
    let mut stack = vec![ast::GroupState::Alternation(alternation)];
    let parser = std::cell::RefCell::new(Parser {
        pos: Cell::new(ast::Position { offset: 1, line: 1, column: 2 }),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(stack),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new("".to_string()),
    });

    let parser_instance = ParserI::new(&parser, "");
    
    let result = parser_instance.pop_group_end(concat);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "GroupUnclosed")]
fn test_pop_group_end_with_group_on_stack() {
    let span = ast::Span { start: ast::Position { offset: 0, line: 1, column: 1 }, end: ast::Position { offset: 0, line: 1, column: 1 } };
    let concat = ast::Concat { span, asts: vec![] };

    let group_span = ast::Span { start: ast::Position { offset: 1, line: 1, column: 2 }, end: ast::Position { offset: 1, line: 1, column: 3 } };
    let group = ast::Group { span: group_span, kind: ast::GroupKind::Normal, ast: Box::new(ast::Ast::Empty(span)) };

    let mut stack = vec![ast::GroupState::Group { concat: concat.clone(), group, ignore_whitespace: false }];
    let parser = std::cell::RefCell::new(Parser {
        pos: Cell::new(ast::Position { offset: 1, line: 1, column: 2 }),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(stack),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new("".to_string()),
    });

    let parser_instance = ParserI::new(&parser, "");
    
    let _ = parser_instance.pop_group_end(concat);
}

