// Answer 0

fn test_visit_class_success() {
    struct MockVisitor {
        visited: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();
        
        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            self.visited = true;
            Ok(())
        }
    }

    let ast = ast::ClassBracketed { 
        span: Span::default(), 
        negated: false, 
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::default(),
            negated: false,
            kind: ClassSet::BinaryOp(ClassSetBinaryOp {
                span: Span::default(),
                kind: ClassSetBinaryOpKind::And,
                lhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Literal('a')))),
                rhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Literal('b')))),
            }),
        }))) 
    };

    let mut visitor = MockVisitor { visited: false };
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class(&ast, &mut visitor);
    assert!(result.is_ok());
    assert!(visitor.visited);
}

fn test_visit_class_empty_stack() {
    struct MockVisitor {
        visited: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();
        
        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Err(()) // Simulate failure
        }
    }

    let ast = ast::ClassBracketed { 
        span: Span::default(), 
        negated: false, 
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::default(),
            negated: false,
            kind: ClassSet::BinaryOp(ClassSetBinaryOp {
                span: Span::default(),
                kind: ClassSetBinaryOpKind::And,
                lhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Literal('a')))),
                rhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Literal('b')))),
            }),
        }))) 
    };

    let mut visitor = MockVisitor { visited: false };
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class(&ast, &mut visitor);
    assert!(result.is_err());
}

fn test_visit_class_binary_operation() {
    struct MockVisitor {
        binary_op_visited: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();
        
        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            self.binary_op_visited = true;
            Ok(())
        }
    }

    let ast = ast::ClassBracketed { 
        span: Span::default(), 
        negated: false, 
        kind: ClassSet::BinaryOp(ClassSetBinaryOp {
            span: Span::default(),
            kind: ClassSetBinaryOpKind::Union,
            lhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Literal('a')))),
            rhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Literal('b')))),
        }) 
    };

    let mut visitor = MockVisitor { binary_op_visited: false };
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.visit_class(&ast, &mut visitor);
    assert!(result.is_ok());
    assert!(visitor.binary_op_visited);
}

