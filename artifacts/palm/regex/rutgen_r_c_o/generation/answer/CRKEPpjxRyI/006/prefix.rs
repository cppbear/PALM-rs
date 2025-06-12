// Answer 0

#[test]
fn test_parse_set_class_range_valid_range() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 2, line: 1, column: 3 };
    let prim1 = Primitive::Literal(Literal {
        span: Span::new(start_pos, start_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    });
    let prim2 = Primitive::Literal(Literal {
        span: Span::new(end_pos, end_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'z',
    });

    let parser = Parser {
        pos: Cell::new(start_pos),
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

    let parser_i = ParserI { parser: &parser, pattern: "a-z" };
    
    let _ = parser_i.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 2, line: 1, column: 3 };
    let prim1 = Primitive::Literal(Literal {
        span: Span::new(start_pos, start_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'z',
    });
    let prim2 = Primitive::Literal(Literal {
        span: Span::new(end_pos, end_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    });

    let parser = Parser {
        pos: Cell::new(start_pos),
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

    let parser_i = ParserI { parser: &parser, pattern: "z-a" };

    let _ = parser_i.parse_set_class_range();
}

#[test]
#[should_panic]
fn test_parse_set_class_range_no_second_item() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let prim1 = Primitive::Literal(Literal {
        span: Span::new(start_pos, start_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    });

    let parser = Parser {
        pos: Cell::new(start_pos),
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

    let parser_i = ParserI { parser: &parser, pattern: "a-" };
    
    let _ = parser_i.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_empty_class_item() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let parser = Parser {
        pos: Cell::new(start_pos),
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

    let parser_i = ParserI { parser: &parser, pattern: "[]" };

    let _ = parser_i.parse_set_class_range();
}

