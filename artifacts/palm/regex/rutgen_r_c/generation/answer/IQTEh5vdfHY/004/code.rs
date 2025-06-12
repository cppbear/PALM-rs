// Answer 0

#[test]
fn test_visit_class_post_item() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_item_post(&mut self, _item: &ast::ClassSetItem) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn visit_class_set_binary_op_post(&mut self, _op: &ast::ClassSetBinaryOp) -> Result<Self::Output, Self::Err> {
            Err(())
        }
    }

    let item = ast::ClassSetItem::Literal(ast::Literal::new("a"));
    let ast = ClassInduct::Item(&item);
    let mut visitor = TestVisitor;

    let heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class_post(&ast, &mut visitor);

    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_post_binary_op() {
    struct ErrorVisitor;

    impl Visitor for ErrorVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_item_post(&mut self, _item: &ast::ClassSetItem) -> Result<Self::Output, Self::Err> {
            Err(())
        }

        fn visit_class_set_binary_op_post(&mut self, _op: &ast::ClassSetBinaryOp) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let binary_op = ast::ClassSetBinaryOp {
        span: ast::Span::new(0, 10),
        kind: ast::ClassSetBinaryOpKind::Union,
        lhs: Box::new(ast::ClassSet::new()),
        rhs: Box::new(ast::ClassSet::new()),
    };
    let ast = ClassInduct::BinaryOp(&binary_op);
    let mut visitor = ErrorVisitor;

    let heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class_post(&ast, &mut visitor);

    assert_eq!(result, Ok(()));
}

