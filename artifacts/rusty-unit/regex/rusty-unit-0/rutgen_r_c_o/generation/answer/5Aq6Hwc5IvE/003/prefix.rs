// Answer 0

#[test]
fn test_pop_class_valid_case() {
    let nested_union = ClassSetUnion {
        span: Span::new(0, 5),
        items: vec![
            ClassSetItem::Literal(Literal::from('a')),
            ClassSetItem::Literal(Literal::from('b')),
        ],
    };

    let parser_state = Parser {
        pos: Cell::new(Position { offset: 5, line: 1, column: 6 }),
        stack_class: RefCell::new(vec![ClassState::Open {
            union: ClassSetUnion {
                span: Span::new(0, 0),
                items: vec![],
            },
            set: ClassBracketed {
                span: Span::new(0, 5),
                negated: false,
                kind: ClassSet::Item(ClassSetItem::Bracketed(Box::new(nested_union.clone()))),
            },
        }]),
        ..Parser::default()
    };

    let parser_instance = ParserI::new(&parser_state, "[ab]");
    let result = parser_instance.pop_class(nested_union);
}

#[test]
#[should_panic]
fn test_pop_class_empty_stack_case() {
    let nested_union = ClassSetUnion {
        span: Span::new(0, 5),
        items: vec![ClassSetItem::Literal(Literal::from('c'))],
    };

    let parser_state = Parser {
        stack_class: RefCell::new(vec![]),
        ..Parser::default()
    };

    let parser_instance = ParserI::new(&parser_state, "[c]");
    let result = parser_instance.pop_class(nested_union);
}

#[test]
#[should_panic]
fn test_pop_class_unexpected_op_state_case() {
    let nested_union = ClassSetUnion {
        span: Span::new(0, 5),
        items: vec![ClassSetItem::Literal(Literal::from('d'))],
    };

    let parser_state = Parser {
        stack_class: RefCell::new(vec![ClassState::Op {
            kind: ast::ClassSetBinaryOpKind::Union,
            lhs: ClassSet::Item(ClassSetItem::Literal(Literal::from('e'))),
        }]),
        ..Parser::default()
    };

    let parser_instance = ParserI::new(&parser_state, "[d]");
    let result = parser_instance.pop_class(nested_union);
}

#[test]
fn test_pop_class_nested_class_case() {
    let nested_union = ClassSetUnion {
        span: Span::new(0, 6),
        items: vec![ClassSetItem::Literal(Literal::from('x'))],
    };

    let parser_state = Parser {
        stack_class: RefCell::new(vec![ClassState::Open {
            union: ClassSetUnion {
                span: Span::new(5, 6),
                items: vec![],
            },
            set: ClassBracketed {
                span: Span::new(5, 10),
                negated: false,
                kind: ClassSet::Item(ClassSetItem::Bracketed(Box::new(nested_union.clone()))),
            },
        }]),
        ..Parser::default()
    };

    let parser_instance = ParserI::new(&parser_state, "[x]");
    let result = parser_instance.pop_class(nested_union);
}

#[test]
fn test_pop_class_large_nested_union_case() {
    let nested_union = ClassSetUnion {
        span: Span::new(0, 1000),
        items: (0..1000).map(|i| ClassSetItem::Literal(Literal::from('a' + i as u8))).collect(),
    };

    let parser_state = Parser {
        stack_class: RefCell::new(vec![ClassState::Open {
            union: ClassSetUnion {
                span: Span::new(0, 0),
                items: vec![],
            },
            set: ClassBracketed {
                span: Span::new(0, 1000),
                negated: false,
                kind: ClassSet::Item(ClassSetItem::Bracketed(Box::new(nested_union.clone()))),
            },
        }]),
        ..Parser::default()
    };

    let parser_instance = ParserI::new(&parser_state, "[".to_owned() + &"a".repeat(1000) + "]");
    let result = parser_instance.pop_class(nested_union);
}

