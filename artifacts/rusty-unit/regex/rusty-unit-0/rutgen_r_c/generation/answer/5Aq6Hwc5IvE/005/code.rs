// Answer 0

#[test]
fn test_pop_class_empty_stack() {
    let nested_union = ClassSetUnion {
        span: Span::new(0, 1),
        items: vec![],
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
        stack_class: RefCell::new(vec![]),  // empty stack
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "]");

    let result = std::panic::catch_unwind(|| {
        parser_i.pop_class(nested_union)
    });

    assert!(result.is_err());
}

#[test]
fn test_pop_class_unexpected_op_state() {
    let nested_union = ClassSetUnion {
        span: Span::new(0, 1),
        items: vec![],
    };

    let mut stack_class = vec![
        ClassState::Open { 
            union: ast::ClassSetUnion { span: Span::new(0, 1), items: vec![] }, 
            set: ClassBracketed { span: Span::new(0, 1), negated: false, kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('a'))) },
        },
        ClassState::Op { 
            kind: ast::ClassSetBinaryOpKind::And, 
            lhs: ClassSet::Item(ClassSetItem::Literal(Literal::new('b'))) 
        },
    ];
    
    let parser = Parser {
        pos: Cell::new(Position { offset: 1, line: 1, column: 2 }),
        capture_index: Cell::new(0),
        nest_limit: 0,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(stack_class),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "]");

    let result = std::panic::catch_unwind(|| {
        parser_i.pop_class(nested_union)
    });

    assert!(result.is_err());
}

#[test]
fn test_pop_class_success() {
    let nested_union = ClassSetUnion {
        span: Span::new(0, 4),
        items: vec![],
    };
    
    let mut stack_class = vec![
        ClassState::Open { 
            union: ClassSetUnion { span: Span::new(0, 5), items: vec![] }, 
            set: ClassBracketed { span: Span::new(0, 5), negated: false, kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('a'))) },
        },
    ];
    
    let parser = Parser {
        pos: Cell::new(Position { offset: 5, line: 1, column: 6 }),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(stack_class),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&parser, "]");

    let result = parser_i.pop_class(nested_union).unwrap();

    match result {
        Either::Right(class) => {
            assert_eq!(class.span.start, 0);
            assert_eq!(class.span.end, 5);
            assert!(matches!(class.kind, ClassSet::Item(ClassSetItem::Bracketed(_))));
        }
        _ => panic!("Expected Right variant but got Left"),
    }
}

