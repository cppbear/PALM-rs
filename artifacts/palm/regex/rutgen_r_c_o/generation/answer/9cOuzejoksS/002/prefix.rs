// Answer 0

#[test]
fn test_visit_class_pre_with_binary_op() {
    let op = ClassSetBinaryOp {
        span: Span::new(0, 10),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::new()),
        rhs: Box::new(ClassSet::new()),
    };

    let ast = ClassInduct::BinaryOp(&op);

    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_item_pre(&mut self, _item: &ClassSetItem) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_class_set_binary_op_pre(&mut self, _op: &ClassSetBinaryOp) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = TestVisitor;
    let heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class_pre(&ast, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_pre_with_different_binary_op() {
    let op = ClassSetBinaryOp {
        span: Span::new(5, 15),
        kind: ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet::new()),
        rhs: Box::new(ClassSet::new()),
    };

    let ast = ClassInduct::BinaryOp(&op);

    struct AnotherTestVisitor;

    impl Visitor for AnotherTestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_item_pre(&mut self, _item: &ClassSetItem) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_class_set_binary_op_pre(&mut self, _op: &ClassSetBinaryOp) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = AnotherTestVisitor;
    let heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class_pre(&ast, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_pre_with_edge_case_binary_op() {
    let op = ClassSetBinaryOp {
        span: Span::new(1, 2),
        kind: ClassSetBinaryOpKind::Difference,
        lhs: Box::new(ClassSet::new()),
        rhs: Box::new(ClassSet::new()),
    };

    let ast = ClassInduct::BinaryOp(&op);

    struct EdgeCaseVisitor;

    impl Visitor for EdgeCaseVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_item_pre(&mut self, _item: &ClassSetItem) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_class_set_binary_op_pre(&mut self, _op: &ClassSetBinaryOp) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let mut visitor = EdgeCaseVisitor;
    let heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class_pre(&ast, &mut visitor).unwrap();
}

