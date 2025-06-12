// Answer 0

#[test]
fn test_pop_class_with_single_item_nested_union() {
    let mut parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![ClassState::Open { 
            union: ClassSetUnion { 
                span: Span::new(0, 1), 
                items: vec![ClassSetItem::Literal(Literal::new('a'))] 
            }, 
            set: ClassBracketed { 
                span: Span::new(0, 1), 
                negated: false, 
                kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('b'))) 
            } 
        }]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let nested_union = ClassSetUnion { span: Span::new(0, 1), items: vec![ClassSetItem::Literal(Literal::new('c'))] };
    parser.pop_class(nested_union);
}

#[test]
fn test_pop_class_with_empty_nested_union() {
    let mut parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![ClassState::Open { 
            union: ClassSetUnion { 
                span: Span::new(0, 1), 
                items: vec![] 
            }, 
            set: ClassBracketed { 
                span: Span::new(0, 1), 
                negated: false, 
                kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('a'))) 
            } 
        }]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let nested_union = ClassSetUnion { span: Span::new(0, 0), items: vec![] };
    parser.pop_class(nested_union);
}

#[test]
#[should_panic]
fn test_pop_class_with_empty_stack() {
    let mut parser = Parser {
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
    let nested_union = ClassSetUnion { span: Span::new(0, 1), items: vec![ClassSetItem::Literal(Literal::new('a'))] };
    parser.pop_class(nested_union);
}

#[test]
fn test_pop_class_with_multiple_operations() {
    let mut parser = Parser {
        pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![
            ClassState::Op { 
                kind: ClassSetBinaryOpKind::Union, 
                lhs: ClassSet::Item(ClassSetItem::Literal(Literal::new('x'))) 
            },
            ClassState::Open { 
                union: ClassSetUnion { 
                    span: Span::new(0, 3), 
                    items: vec![ClassSetItem::Literal(Literal::new('y'))] 
                },
                set: ClassBracketed { 
                    span: Span::new(0, 2), 
                    negated: false, 
                    kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('z'))) 
                }
            }
        ]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    let nested_union = ClassSetUnion { span: Span::new(0, 3), items: vec![ClassSetItem::Literal(Literal::new('w'))] };
    parser.pop_class(nested_union);
}

