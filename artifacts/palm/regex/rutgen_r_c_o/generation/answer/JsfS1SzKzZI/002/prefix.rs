// Answer 0

#[test]
fn test_visit_class_negated_union() {
    let span = Span::new(0, 5);
    let class_set_binary_op = ClassSetBinaryOp {
        span: Span::new(0, 5),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::Item(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: span.clone(),
            negated: false,
            kind: ClassSet::Item(ast::ClassSetItem::Literal('a')),
        }))),
        rhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal('b'))),
    };
    let class_bracketed = ClassBracketed {
        span,
        negated: true,
        kind: ClassSet::BinaryOp(class_set_binary_op),
    };

    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    };

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&class_bracketed, &mut visitor).unwrap_err();
}

#[test]
fn test_visit_class_negated_binary() {
    let span = Span::new(5, 10);
    let class_set_binary_op = ClassSetBinaryOp {
        span: Span::new(5, 10),
        kind: ClassSetBinaryOpKind::Binary,
        lhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal('c'))),
        rhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal('d'))),
    };
    let class_bracketed = ClassBracketed {
        span,
        negated: true,
        kind: ClassSet::BinaryOp(class_set_binary_op),
    };

    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Err(())
        }
    };

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&class_bracketed, &mut visitor).unwrap_err();
}

#[test]
fn test_visit_class_non_negated_single() {
    let span = Span::new(0, 3);
    let class_set_item = ast::ClassSetItem::Bracketed(ClassBracketed {
        span: span.clone(),
        negated: false,
        kind: ClassSet::Item(ast::ClassSetItem::Literal('e')),
    });
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(class_set_item),
    };

    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    };

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.visit_class(&class_bracketed, &mut visitor).unwrap_err();
}

