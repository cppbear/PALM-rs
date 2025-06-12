// Answer 0

#[test]
fn test_pop_class_union_with_tail() {
    struct DummyAst;
    
    let item1 = ast::ClassSetItem::Literal(DummyAst);
    let item2 = ast::ClassSetItem::Literal(DummyAst);
    let frame = ClassFrame::Union { head: &item1, tail: &[item2] };
    
    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(frame);
    
    assert!(result.is_some());
    if let Some(ClassFrame::Union { head, tail }) = result {
        assert_eq!(head, &item2);
        assert_eq!(tail.len(), 0);
    }
}

#[test]
fn test_pop_class_union_empty_tail() {
    struct DummyAst;

    let item = ast::ClassSetItem::Literal(DummyAst);
    let frame = ClassFrame::Union { head: &item, tail: &[] };
    
    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(frame);
    
    assert!(result.is_none());
}

#[test]
fn test_pop_class_binary() {
    struct DummyAst;
    
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::default(),
        lhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal(DummyAst))),
        rhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal(DummyAst))),
    };
    
    let frame = ClassFrame::Binary { op: &op };
    
    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(frame);
    
    assert!(result.is_none());
}

#[test]
fn test_pop_class_binary_lhs() {
    struct DummyAst;
    
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::default(),
        lhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal(DummyAst))),
        rhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal(DummyAst))),
    };
    let frame = ClassFrame::BinaryLHS { op: &op, lhs: &ClassSet::Item(ast::ClassSetItem::Literal(DummyAst)), rhs: &ClassSet::Item(ast::ClassSetItem::Literal(DummyAst)) };
    
    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(frame);
    
    assert!(result.is_some());
    if let Some(ClassFrame::BinaryRHS { op: _, rhs }) = result {
        assert!(matches!(rhs, &ClassSet::Item(ast::ClassSetItem::Literal(DummyAst))));
    }
}

#[test]
fn test_pop_class_binary_rhs() {
    struct DummyAst;
    
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::default(),
        lhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal(DummyAst))),
        rhs: Box::new(ClassSet::Item(ast::ClassSetItem::Literal(DummyAst))),
    };
    let frame = ClassFrame::BinaryRHS { op: &op };
    
    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(frame);
    
    assert!(result.is_none());
}

