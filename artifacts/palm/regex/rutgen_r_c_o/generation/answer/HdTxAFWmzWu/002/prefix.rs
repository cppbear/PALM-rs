// Answer 0

#[test]
fn test_pop_class_binary_lhs() {
    use ast::{ClassSetBinaryOp, ClassSetItem, ClassSet, Span, Literal, ClassSetBinaryOpKind};
    
    let op = ClassSetBinaryOp {
        span: Span::new(0, 1),
        kind: ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('a')))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('b')))),
    };
    
    let induct = ClassFrame::BinaryLHS {
        op: &op,
        rhs: &ClassSet::Item(ClassSetItem::Literal(Literal::new('c'))),
    };

    let visitor = HeapVisitor::new();
    visitor.pop_class(induct);
}

#[test]
fn test_pop_class_binary_rhs() {
    use ast::{ClassSetBinaryOp, ClassSetItem, ClassSet, Span, Literal, ClassSetBinaryOpKind};
    
    let op = ClassSetBinaryOp {
        span: Span::new(0, 1),
        kind: ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('x')))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('y')))),
    };
    
    let induct = ClassFrame::BinaryLHS {
        op: &op,
        rhs: &ClassSet::Item(ClassSetItem::Literal(Literal::new('z'))),
    };

    let visitor = HeapVisitor::new();
    visitor.pop_class(induct);
}

#[test]
fn test_pop_class_empty_union() {
    use ast::{ClassSetBinaryOp, ClassSetItem, ClassSet, Span, Literal, ClassSetBinaryOpKind};
    
    let op = ClassSetBinaryOp {
        span: Span::new(0, 1),
        kind: ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('A')))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('B')))),
    };

    let induct = ClassFrame::BinaryLHS {
        op: &op,
        rhs: &ClassSet::Item(ClassSetItem::Literal(Literal::new('C'))),
    };

    let visitor = HeapVisitor::new();
    visitor.pop_class(induct);
}

#[test]
fn test_pop_class_empty_binary() {
    use ast::{ClassSetBinaryOp, ClassSetItem, ClassSet, Span, Literal, ClassSetBinaryOpKind};
    
    let op = ClassSetBinaryOp {
        span: Span::new(0, 1),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('A')))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('B')))),
    };

    let induct = ClassFrame::Binary {
        op: &op,
    };

    let visitor = HeapVisitor::new();
    visitor.pop_class(induct);
}

