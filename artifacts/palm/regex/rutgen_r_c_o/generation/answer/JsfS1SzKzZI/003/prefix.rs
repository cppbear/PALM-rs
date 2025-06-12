// Answer 0

#[test]
fn test_visit_class_normal_union() {
    let span = Span::new(1, 5);
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(ClassSetItem::Bracketed(ClassBracketed {
            span,
            negated: false,
            kind: ClassSet::Item(ClassSetItem::Literal(Literal::from_char('a'))),
        })),
    };
    
    let mut visitor = MockVisitor::new(Ok(()));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack_class.push((ClassInduct::from_bracketed(&class_bracketed), ClassFrame::Union { head: &class_bracketed.kind, tail: &[] }));

    heap_visitor.visit_class(&class_bracketed, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_binary_op_success() {
    let span = Span::new(2, 8);
    let left = ClassSet::Item(ClassSetItem::Bracketed(ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(ClassSetItem::Literal(Literal::from_char('a'))),
    }));
    let right = ClassSet::Item(ClassSetItem::Bracketed(ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(ClassSetItem::Literal(Literal::from_char('b'))),
    }));
    let binary_op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(left),
        rhs: Box::new(right),
    };
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::BinaryOp(binary_op),
    };

    let mut visitor = MockVisitor::new(Ok(()));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack_class.push((ClassInduct::from_bracketed(&class_bracketed), ClassFrame::BinaryLHS { op: &binary_op, lhs: &binary_op.lhs, rhs: &binary_op.rhs }));

    heap_visitor.visit_class(&class_bracketed, &mut visitor).unwrap();
}

#[test]
#[should_panic]
fn test_visit_class_with_post_visit_error() {
    let span = Span::new(3, 6);
    let class_bracketed = ClassBracketed {
        span,
        negated: true,
        kind: ClassSet::Item(ClassSetItem::Bracketed(ClassBracketed {
            span,
            negated: true,
            kind: ClassSet::Item(ClassSetItem::Literal(Literal::from_char('c'))),
        })),
    };

    let mut visitor = MockVisitor::new(Err("Error".to_string()));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack_class.push((ClassInduct::from_bracketed(&class_bracketed), ClassFrame::Union { head: &class_bracketed.kind, tail: &[] }));

    heap_visitor.visit_class(&class_bracketed, &mut visitor).unwrap();
} 

#[test]
fn test_visit_class_with_multiple_inductive_steps() {
    let span = Span::new(1, 10);
    let class_bracketed_1 = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(ClassSetItem::Bracketed(ClassBracketed {
            span,
            negated: false,
            kind: ClassSet::Item(ClassSetItem::Literal(Literal::from_char('x'))),
        })),
    };
    let class_bracketed_2 = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::BinaryOp(ClassSetBinaryOp {
            span,
            kind: ClassSetBinaryOpKind::Intersect,
            lhs: Box::new(ClassSet::Item(ClassSetItem::Bracketed(class_bracketed_1))),
            rhs: Box::new(ClassSet::Item(ClassSetItem::Bracketed(ClassBracketed {
                span,
                negated: false,
                kind: ClassSet::Item(ClassSetItem::Literal(Literal::from_char('y'))),
            }))),
        }),
    };
    
    let mut visitor = MockVisitor::new(Ok(()));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack_class.push((ClassInduct::from_bracketed(&class_bracketed_2), ClassFrame::BinaryLHS { op: &class_bracketed_2.kind, lhs: &class_bracketed_2.kind, rhs: &class_bracketed_2.kind }));

    heap_visitor.visit_class(&class_bracketed_2, &mut visitor).unwrap();
} 

#[test]
fn test_visit_class_with_empty_tail() {
    let span = Span::new(1, 5);
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(ClassSetItem::Union(ast::ClassSetUnion {
            items: vec![],
        })),
    };
    
    let mut visitor = MockVisitor::new(Ok(()));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack_class.push((ClassInduct::from_bracketed(&class_bracketed), ClassFrame::Union { head: &class_bracketed.kind, tail: &[] }));

    heap_visitor.visit_class(&class_bracketed, &mut visitor).unwrap();
} 

struct MockVisitor {
    result: Result<(), String>,
}

impl MockVisitor {
    fn new(result: Result<(), String>) -> Self {
        MockVisitor { result }
    }
}

impl Visitor for MockVisitor {
    type Output = ();
    type Err = String;

    fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
        Ok(())
    }
}

