// Answer 0

#[test]
fn test_visit_class_with_valid_input() {
    struct MockVisitor {
        visited: bool,
    }

    impl MockVisitor {
        fn new() -> Self {
            MockVisitor { visited: false }
        }
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor::new();
    let span = Span::new(0, 10);
    let class_kind = ClassSet::Item(ast::ClassSetItem::Bracketed(ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Literal(Literal::new('a')))),
    }));

    let ast_class = ClassBracketed {
        span,
        negated: false,
        kind: class_kind,
    };

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class(&ast_class, &mut visitor);

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_visit_class_with_inductive_case_fails_pre_visit() {
    struct MockVisitor {
        should_fail_pre: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor { should_fail_pre: true };
    let span = Span::new(0, 10);
    let class_kind = ClassSet::Item(ast::ClassSetItem::Bracketed(ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Literal(Literal::new('a')))),
    }));

    let ast_class = ClassBracketed {
        span,
        negated: false,
        kind: class_kind,
    };

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&ast_class, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_with_inductive_case_no_post_visit() {
    struct MockVisitor {}

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = MockVisitor {};
    let span = Span::new(0, 10);
    let class_kind = ClassSet::Item(ast::ClassSetItem::Bracketed(ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Literal(Literal::new('a')))),
    }));

    let ast_class = ClassBracketed {
        span,
        negated: false,
        kind: class_kind,
    };

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class(&ast_class, &mut visitor);

    assert!(result.is_err());
}

