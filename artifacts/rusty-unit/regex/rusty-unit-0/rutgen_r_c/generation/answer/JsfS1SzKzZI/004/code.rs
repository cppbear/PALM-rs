// Answer 0

fn test_visit_class_success() {
    struct MockVisitor {
        visit_class_pre_called: bool,
        visit_class_post_called: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _: &ast::ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
        
        fn visit_class_pre(&mut self, _: &ClassInduct) -> Result<(), Self::Err> {
            self.visit_class_pre_called = true;
            Ok(())
        }

        fn visit_class_post(&mut self, _: &ClassInduct) -> Result<(), Self::Err> {
            self.visit_class_post_called = true;
            Ok(())
        }
    }

    let mut visitor = MockVisitor {
        visit_class_pre_called: false,
        visit_class_post_called: false,
    };
    
    let ast = ast::ClassBracketed {
        span: Span { start: 0, end: 10 },
        negated: false,
        kind: ClassSet::Item(ast::ClassSetItem::Bracketed(ast::ClassBracketed {
            span: Span { start: 0, end: 10 },
            negated: false,
            kind: ClassSet::Item(Box::new(ast::ClassSetItem::Empty)),
        })),
    };

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class(&ast, &mut visitor);

    assert!(result.is_ok());
    assert!(visitor.visit_class_pre_called);
    assert!(visitor.visit_class_post_called);
}

fn test_visit_class_induct_class_true() {
    struct MockVisitor {
        visit_class_pre_called: bool,
        visit_class_post_called: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _: &ast::ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_class_pre(&mut self, _: &ClassInduct) -> Result<(), Self::Err> {
            self.visit_class_pre_called = true;
            Ok(())
        }

        fn visit_class_post(&mut self, _: &ClassInduct) -> Result<(), Self::Err> {
            self.visit_class_post_called = true;
            Ok(())
        }
    }

    let mut visitor = MockVisitor {
        visit_class_pre_called: false,
        visit_class_post_called: false,
    };

    let ast = ast::ClassBracketed {
        span: Span { start: 0, end: 10 },
        negated: false,
        kind: ClassSet::BinaryOp(Box::new(ast::ClassSetBinaryOp {
            span: Span { start: 0, end: 10 },
            kind: ast::ClassSetBinaryOpKind::Union,
            lhs: Box::new(ClassSet::Item(ast::ClassSetItem::Empty)),
            rhs: Box::new(ClassSet::Item(ast::ClassSetItem::Empty)),
        })),
    };

    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class(&ast, &mut visitor);

    assert!(result.is_ok());
    assert!(visitor.visit_class_pre_called);
    assert!(visitor.visit_class_post_called);
}

fn test_visit_class_stack_pop_none() {
    struct MockVisitor {
        visit_class_pre_called: bool,
        visit_class_post_called: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _: &ast::ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }

        fn visit_class_pre(&mut self, _: &ClassInduct) -> Result<(), Self::Err> {
            self.visit_class_pre_called = true;
            Ok(())
        }

        fn visit_class_post(&mut self, _: &ClassInduct) -> Result<(), Self::Err> {
            self.visit_class_post_called = true;
            Ok(())
        }
    }

    let mut visitor = MockVisitor {
        visit_class_pre_called: false,
        visit_class_post_called: false,
    };

    let ast = ast::ClassBracketed {
        span: Span { start: 0, end: 10 },
        negated: false,
        kind: ClassSet::BinaryOp(Box::new(ast::ClassSetBinaryOp {
            span: Span { start: 0, end: 10 },
            kind: ast::ClassSetBinaryOpKind::Union,
            lhs: Box::new(ClassSet::Item(ast::ClassSetItem::Empty)),
            rhs: Box::new(ClassSet::Item(ast::ClassSetItem::Empty)),
        })),
    };

    let mut heap_visitor = HeapVisitor::new();
    // Ensure that the stack is empty
    let result = heap_visitor.visit_class(&ast, &mut visitor);

    assert!(result.is_ok());
    assert!(visitor.visit_class_pre_called);
    assert!(visitor.visit_class_post_called);
}

