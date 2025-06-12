// Answer 0

#[test]
fn test_visit_class_single_item() {
    struct TestVisitor {
        encountered: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            self.encountered = true;
            Ok(())
        }
    }

    let span = Span::new(0, 1); // Example span, adjust as needed
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::new(0, 1),
            negated: false,
            kind: ClassSet::Item(ast::ClassSetItem::Literal(Literal::new('a'))),
        })),
    };

    let mut visitor = TestVisitor { encountered: false };
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class(&class_bracketed, &mut visitor);
    
    assert!(result.is_ok());
    assert!(!visitor.encountered);
}

#[test]
fn test_visit_class_binary_operation() {
    struct TestVisitor {
        encountered: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            self.encountered = true;
            Ok(())
        }
    }

    let span = Span::new(0, 5); // Example span, adjust as needed
    let binary_op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::Item(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::new(0, 2),
            negated: false,
            kind: ClassSet::Item(ast::ClassSetItem::Literal(Literal::new('a'))),
        }))),
        rhs: Box::new(ClassSet::Item(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::new(2, 5),
            negated: false,
            kind: ClassSet::Item(ast::ClassSetItem::Literal(Literal::new('b'))),
        }))),
    };

    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::BinaryOp(binary_op),
    };

    let mut visitor = TestVisitor { encountered: false };
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class(&class_bracketed, &mut visitor);
    
    assert!(result.is_ok());
    assert!(visitor.encountered);
}

