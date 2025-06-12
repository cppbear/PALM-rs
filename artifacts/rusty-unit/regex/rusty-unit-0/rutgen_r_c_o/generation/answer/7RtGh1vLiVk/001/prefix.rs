// Answer 0

#[test]
fn test_push_alternate_valid() {
    let concat = ast::Concat {
        span: ast::Span { start: Position { offset: 1, line: 1, column: 1 }, end: Position { offset: 2, line: 1, column: 1 } },
        asts: vec![],
    };
    let parser = Parser {
        pos: Cell::new(Position { offset: 1, line: 1, column: 1 }),
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
    let parser_i = ParserI::new(&parser, "|a");
    let _ = parser_i.push_alternate(concat);
}

#[test]
fn test_push_alternate_multiple_ast() {
    let concat = ast::Concat {
        span: ast::Span { start: Position { offset: 1, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 1 } },
        asts: vec![ast::Ast::Literal(ast::Literal::new("a")), ast::Ast::Literal(ast::Literal::new("b"))],
    };
    let parser = Parser {
        pos: Cell::new(Position { offset: 1, line: 1, column: 1 }),
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
    let parser_i = ParserI::new(&parser, "|ab");
    let _ = parser_i.push_alternate(concat);
}

#[test]
fn test_push_alternate_empty_ast() {
    let concat = ast::Concat {
        span: ast::Span { start: Position { offset: 1, line: 1, column: 1 }, end: Position { offset: 1, line: 1, column: 1 } },
        asts: vec![],
    };
    let parser = Parser {
        pos: Cell::new(Position { offset: 1, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![ast::GroupState::new()]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "|");
    let _ = parser_i.push_alternate(concat);
}

#[test]
#[should_panic]
fn test_push_alternate_panic_on_invalid_char() {
    let concat = ast::Concat {
        span: ast::Span { start: Position { offset: 1, line: 1, column: 1 }, end: Position { offset: 2, line: 1, column: 1 } },
        asts: vec![],
    };
    let parser = Parser {
        pos: Cell::new(Position { offset: 1, line: 1, column: 1 }),
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
    let parser_i = ParserI::new(&parser, "a");
    let _ = parser_i.push_alternate(concat);
}

