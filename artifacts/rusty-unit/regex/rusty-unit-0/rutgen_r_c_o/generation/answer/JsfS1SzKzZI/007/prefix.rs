// Answer 0

#[test]
fn test_visit_class_with_non_negated_single_item() {
    let span = Span::new(1, 10);
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('a'))),
    };
    
    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit_class(&class_bracketed, &mut visitor);
}

#[test]
fn test_visit_class_with_negated_single_item() {
    let span = Span::new(1, 20);
    let class_bracketed = ClassBracketed {
        span,
        negated: true,
        kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('b'))),
    };
    
    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit_class(&class_bracketed, &mut visitor);
}

#[test]
fn test_visit_class_with_binary_operation() {
    let span = Span::new(1, 30);
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal::new('c')));
    let lhs = ClassSet::Item(ClassSetItem::Literal(Literal::new('d')));
    let binary_op = ClassSetBinaryOp {
        span,
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::BinaryOp(binary_op),
    };
    
    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit_class(&class_bracketed, &mut visitor);
}

#[test]
fn test_visit_class_with_union_of_items() {
    let span = Span::new(1, 40);
    let items = vec![
        ClassSetItem::Literal(Literal::new('e')),
        ClassSetItem::Literal(Literal::new('f')),
    ];
    let union = ClassSet::Union(ast::Union { items });

    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: union,
    };
    
    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit_class(&class_bracketed, &mut visitor);
}

#[test]
fn test_visit_class_with_empty_union() {
    let span = Span::new(1, 50);
    let union = ClassSet::Union(ast::Union { items: vec![] });

    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: union,
    };
    
    let mut visitor = TestVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit_class(&class_bracketed, &mut visitor);
}

struct TestVisitor {
    calls: usize,
}

impl TestVisitor {
    fn new() -> Self {
        Self { calls: 0 }
    }
}

impl Visitor for TestVisitor {
    type Output = ();
    type Err = ();

    fn visit_class_set_binary_op_in(&mut self, _: &ClassSetBinaryOp) -> Result<Self::Output, Self::Err> {
        self.calls += 1;
        Ok(())
    }
}

