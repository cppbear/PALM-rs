// Answer 0

#[test]
fn test_push_alternate_non_empty_concat() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
    };
    let parser_i = ParserI::new(&parser, "|abc");
    let concat = ast::Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 4, line: 1, column: 5 }},
        asts: vec![ast::Ast::Literal(ast::Literal::new('a')), ast::Ast::Literal(ast::Literal::new('b')), ast::Ast::Literal(ast::Literal::new('c'))],
    };
    parser_i.push_alternate(concat);
}

#[test]
fn test_push_alternate_empty_concat() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
    };
    let parser_i = ParserI::new(&parser, "|");
    let concat = ast::Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 }},
        asts: vec![],
    };
    parser_i.push_alternate(concat);
}

#[test]
#[should_panic]
fn test_push_alternate_invalid_character() {
    let parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
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
    };
    let parser_i = ParserI::new(&parser, "abc");
    let concat = ast::Concat {
        span: Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 3, line: 1, column: 4 }},
        asts: vec![ast::Ast::Literal(ast::Literal::new('a')), ast::Ast::Literal(ast::Literal::new('b')), ast::Ast::Literal(ast::Literal::new('c'))],
    };
    parser_i.push_alternate(concat);
}

