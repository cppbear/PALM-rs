// Answer 0

#[test]
fn test_pop_group_end_with_empty_stack() {
    let span = Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } };
    let concat = Concat { span, asts: vec![] };
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
    let parser_i = ParserI::new(&parser, "");
    let _ = parser_i.pop_group_end(concat);
}

#[test]
fn test_pop_group_end_with_single_alternation_on_stack() {
    let span = Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } };
    let concat = Concat { span, asts: vec![] };
    let mut parser = Parser { 
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }), 
        capture_index: Cell::new(1), 
        nest_limit: 10, 
        octal: false, 
        initial_ignore_whitespace: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![GroupState::Alternation(Alternation { span, asts: vec![] })]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "");
    let _ = parser_i.pop_group_end(concat);
}

#[test]
#[should_panic]
fn test_pop_group_end_with_group_on_stack() {
    let span = Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } };
    let concat = Concat { span, asts: vec![] };
    let group_span = Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 2 } };
    let mut parser = Parser { 
        pos: Cell::new(Position { offset: 1, line: 1, column: 2 }), 
        capture_index: Cell::new(1), 
        nest_limit: 10, 
        octal: false, 
        initial_ignore_whitespace: false, 
        ignore_whitespace: Cell::new(false), 
        comments: RefCell::new(vec![]), 
        stack_group: RefCell::new(vec![GroupState::Group { concat: Concat { span: group_span, asts: vec![] }, group: Group { span: group_span, kind: GroupKind::NonCapturing, ast: Box::new(Ast::Empty(group_span)) }, ignore_whitespace: false }]), 
        stack_class: RefCell::new(vec![]), 
        capture_names: RefCell::new(vec![]), 
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "");
    let _ = parser_i.pop_group_end(concat);
}

