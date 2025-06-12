// Answer 0

#[test]
fn test_push_class_op_intersection() {
    let span = Span { start: 0, end: 1 };
    let class_set_union = ClassSetUnion { span, items: vec![ClassSetItem::Literal(Literal::from('a'))] };
    let parser = Parser {
        pos: Cell::new(0),
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
    let parser_instance = ParserI::new(&parser, "a");
    parser_instance.push_class_op(ClassSetBinaryOpKind::Intersection, class_set_union);
}

#[test]
fn test_push_class_op_difference() {
    let span = Span { start: 0, end: 2 };
    let class_set_union = ClassSetUnion { span, items: vec![ClassSetItem::Literal(Literal::from('b'))] };
    let parser = Parser {
        pos: Cell::new(0),
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
    let parser_instance = ParserI::new(&parser, "b");
    parser_instance.push_class_op(ClassSetBinaryOpKind::Difference, class_set_union);
}

#[test]
fn test_push_class_op_symmetric_difference() {
    let span = Span { start: 0, end: 3 };
    let class_set_union = ClassSetUnion { span, items: vec![ClassSetItem::Literal(Literal::from('c'))] };
    let parser = Parser {
        pos: Cell::new(0),
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
    let parser_instance = ParserI::new(&parser, "c");
    parser_instance.push_class_op(ClassSetBinaryOpKind::SymmetricDifference, class_set_union);
}

