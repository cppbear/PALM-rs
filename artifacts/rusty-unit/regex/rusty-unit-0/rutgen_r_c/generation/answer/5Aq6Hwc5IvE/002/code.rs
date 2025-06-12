// Answer 0

#[test]
fn test_pop_class_with_valid_open_state() {
    let mut pos = Position { offset: 0, line: 1, column: 1 };
    let mut parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![
            ClassState::Open {
                union: ClassSetUnion {
                    span: Span::new(0, 1),
                    items: vec![],
                },
                set: ClassBracketed {
                    span: Span::new(0, 1),
                    negated: false,
                    kind: ClassSet::Item(ClassSetItem::Literal(Literal::from('a'))),
                },
            },
        ]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&mut parser, "a");
    let nested_union = ClassSetUnion {
        span: Span::new(0, 1),
        items: vec![ClassSetItem::Literal(Literal::from('b'))],
    };

    let result = parser_i.pop_class(nested_union);
    assert_eq!(result.is_ok(), true);
    if let Ok(Either::Right(class)) = result {
        if let ast::Class::Bracketed(set) = class {
            assert_eq!(set.negated, false);
            assert_eq!(set.span.start, 0);
            assert_eq!(set.span.end, 1);
        } else {
            panic!("Expected a Bracketed class");
        }
    }
}

#[test]
#[should_panic(expected = "unexpected empty character class stack")]
fn test_pop_class_with_empty_stack() {
    let mut pos = Position { offset: 0, line: 1, column: 1 };
    let mut parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]), // Empty stack
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&mut parser, "a");
    let nested_union = ClassSetUnion {
        span: Span::new(0, 1),
        items: vec![],
    };

    parser_i.pop_class(nested_union);
}

#[test]
#[should_panic(expected = "unexpected ClassState::Op")]
fn test_pop_class_with_op_state() {
    let mut pos = Position { offset: 0, line: 1, column: 1 };
    let mut parser = Parser {
        pos: Cell::new(pos),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![
            ClassState::Op {
                kind: ClassSetBinaryOpKind::And,
                lhs: ClassSet::Item(ClassSetItem::Literal(Literal::from('c'))),
            },
        ]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let parser_i = ParserI::new(&mut parser, "c");
    let nested_union = ClassSetUnion {
        span: Span::new(0, 1),
        items: vec![],
    };

    parser_i.pop_class(nested_union);
}

