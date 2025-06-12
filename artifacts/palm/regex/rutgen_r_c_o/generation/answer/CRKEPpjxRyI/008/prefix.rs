// Answer 0

#[test]
fn test_parse_set_class_range_valid_range() {
    let position1 = Position { offset: 1, line: 1, column: 1 };
    let position2 = Position { offset: 2, line: 1, column: 2 };

    let span1 = Span::new(position1, position1);
    let span2 = Span::new(position2, position2);

    let literal1 = Literal { span: span1, kind: ast::LiteralKind::Verbatim, c: 'a' };
    let literal2 = Literal { span: span2, kind: ast::LiteralKind::Verbatim, c: 'b' };

    let prim1 = Primitive::Literal(literal1);
    let prim2 = Primitive::Literal(literal2);

    let class_set_range = ClassSetRange {
        span: Span::new(position1, position2),
        start: literal1,
        end: literal2,
    };

    let mut parser = Parser {
        pos: Cell::new(position1),
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

    let parser_instance = ParserI { parser: &parser, pattern: "[a-b]" };

    let _ = parser_instance.parse_set_class_range();
}

#[test]
#[should_panic]
fn test_parse_set_class_range_unclosed_class() {
    let position1 = Position { offset: 1, line: 1, column: 1 };

    let span1 = Span::new(position1, position1);

    let literal1 = Literal { span: span1, kind: ast::LiteralKind::Verbatim, c: 'a' };

    let prim1 = Primitive::Literal(literal1);

    let mut parser = Parser {
        pos: Cell::new(position1),
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

    let parser_instance = ParserI { parser: &parser, pattern: "[a-" };

    let _ = parser_instance.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_invalid_range() {
    let position1 = Position { offset: 1, line: 1, column: 1 };
    let position2 = Position { offset: 2, line: 1, column: 2 };

    let span1 = Span::new(position1, position1);
    let span2 = Span::new(position2, position2);

    let literal1 = Literal { span: span1, kind: ast::LiteralKind::Verbatim, c: 'b' };
    let literal2 = Literal { span: span2, kind: ast::LiteralKind::Verbatim, c: 'a' };

    let prim1 = Primitive::Literal(literal1);
    let prim2 = Primitive::Literal(literal2);

    let class_set_range = ClassSetRange {
        span: Span::new(position1, position2),
        start: literal1,
        end: literal2,
    };

    let mut parser = Parser {
        pos: Cell::new(position1),
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

    let parser_instance = ParserI { parser: &parser, pattern: "[b-a]" };

    let _ = parser_instance.parse_set_class_range();
}

