// Answer 0

#[test]
fn test_visit_class_pre_item() {
    struct TestVisitor {
        visited: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_item_pre(&mut self, _item: &ClassSetItem) -> Result<(), Self::Err> {
            self.visited = true;
            Ok(())
        }

        fn visit_class_set_binary_op_pre(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            panic!("Should not visit binary op.");
        }
    }

    let item = ClassSetItem::Literal(Literal::from('a'));
    let ast_induct = ClassInduct::Item(&item);
    let visitor = &mut TestVisitor { visited: false };
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class_pre(&ast_induct, visitor).unwrap();
    assert!(visitor.visited);
}

#[test]
fn test_visit_class_pre_binary_op() {
    struct TestVisitor {
        visited: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_item_pre(&mut self, _item: &ClassSetItem) -> Result<(), Self::Err> {
            panic!("Should not visit item.");
        }

        fn visit_class_set_binary_op_pre(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            self.visited = true;
            Ok(())
        }
    }

    let lhs = Box::new(ClassSet::from(ClassSetItem::Literal(Literal::from('a'))));
    let rhs = Box::new(ClassSet::from(ClassSetItem::Literal(Literal::from('z'))));
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::Union,
        lhs,
        rhs,
    };
    let ast_induct = ClassInduct::BinaryOp(&op);
    let visitor = &mut TestVisitor { visited: false };
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class_pre(&ast_induct, visitor).unwrap();
    assert!(visitor.visited);
}

