// Answer 0

#[test]
fn test_pop_class_success_open() {
    struct MockParser {
        stack_class: RefCell<Vec<ClassState>>,
        pos: Cell<Position>,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let union = ClassSetUnion {
        span: Span::new(0, 3),
        items: vec![],
    };

    let mut parser = MockParser {
        stack_class: RefCell::new(vec![ClassState::Open { union: union.clone(), set: ClassBracketed { span: Span::new(0, 3), negated: false, kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('a')))} }]),
        pos: Cell::new(Position { offset: 3, line: 1, column: 4 }),
    };
    
    let result = parser.pop_class(union);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "unexpected empty character class stack")]
fn test_pop_class_panic_empty_stack() {
    struct MockParser {
        stack_class: RefCell<Vec<ClassState>>,
        pos: Cell<Position>,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let union = ClassSetUnion {
        span: Span::new(0, 3),
        items: vec![],
    };

    let mut parser = MockParser {
        stack_class: RefCell::new(vec![]),
        pos: Cell::new(Position { offset: 3, line: 1, column: 4 }),
    };
    
    parser.pop_class(union);
}

#[test]
#[should_panic(expected = "unexpected ClassState::Op")]
fn test_pop_class_panic_unexpected_op() {
    struct MockParser {
        stack_class: RefCell<Vec<ClassState>>,
        pos: Cell<Position>,
    }
    
    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let union = ClassSetUnion {
        span: Span::new(0, 3),
        items: vec![],
    };

    let mut parser = MockParser {
        stack_class: RefCell::new(vec![ClassState::Op { kind: ClassSetBinaryOpKind::And, lhs: ClassSet::Item(ClassSetItem::Literal(Literal::new('a')))}]),
        pos: Cell::new(Position { offset: 3, line: 1, column: 4 }),
    };
    
    parser.pop_class(union);
}

