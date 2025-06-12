// Answer 0

#[test]
fn test_pop_class_with_valid_inputs() {
    let position = Position { offset: 5, line: 1, column: 6 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![ClassState::Open {
            union: ClassSetUnion { span: Span::new(0, 6), items: vec![] },
            set: ClassBracketed { span: Span::new(0, 8), negated: false, kind: ClassSet::Item(ClassSetItem::Literal(Literal::from('a'))) },
        }]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI::new(&parser, "[a]");
    let nested_union = ClassSetUnion { span: Span::new(0, 3), items: vec![ClassSetItem::Literal(Literal::from('a'))] };

    let _ = parser_i.pop_class(nested_union);
}

#[test]
#[should_panic(expected = "unexpected empty character class stack")]
fn test_pop_class_with_empty_stack() {
    let position = Position { offset: 5, line: 1, column: 6 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI::new(&parser, "]");
    let nested_union = ClassSetUnion { span: Span::new(0, 3), items: vec![] };

    let _ = parser_i.pop_class(nested_union);
}

#[test]
#[should_panic(expected = "unexpected ClassState::Op")]
fn test_pop_class_with_op_state_on_top() {
    let position = Position { offset: 5, line: 1, column: 6 };
    let parser = Parser {
        pos: Cell::new(position),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![ClassState::Op { kind: ClassSetBinaryOpKind::Union, lhs: ClassSet::Item(ClassSetItem::Literal(Literal::from('a'))) }]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let parser_i = ParserI::new(&parser, "]");
    let nested_union = ClassSetUnion { span: Span::new(0, 3), items: vec![] };

    let _ = parser_i.pop_class(nested_union);
}

