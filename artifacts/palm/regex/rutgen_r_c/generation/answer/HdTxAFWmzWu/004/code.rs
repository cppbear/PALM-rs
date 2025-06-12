// Answer 0

#[test]
fn test_pop_class_union_empty_tail() {
    struct MockAst;
    let tail: &[MockAst] = &[];
    let induct = ClassFrame::Union { head: &MockAst, tail };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);
    
    assert_eq!(result, None);
}

#[test]
fn test_pop_class_union_non_empty_tail() {
    struct MockAst;
    let tail: &[MockAst] = &[
        MockAst,
        MockAst,
    ];
    let induct = ClassFrame::Union { head: &MockAst, tail };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);

    if let Some(ClassFrame::Union { head, tail }) = result {
        assert_eq!(head, &tail[0]);
        assert_eq!(tail.len(), 1);
    } else {
        panic!("Expected a Some variant with Union frame");
    }
}

#[test]
fn test_pop_class_binary() {
    struct MockAst;
    let induct = ClassFrame::Binary { op: &ClassSetBinaryOp { span: Span, kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal))), rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal))) } };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);

    assert_eq!(result, None);
}

#[test]
fn test_pop_class_binary_lhs() {
    struct MockAst;
    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal));
    let induct = ClassFrame::BinaryLHS {
        op: &ClassSetBinaryOp { span: Span, kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal))), rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal))) },
        lhs: &ClassSet::Item(ClassSetItem::Literal(Literal)),
        rhs: &rhs,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);

    if let Some(ClassFrame::BinaryRHS { op, rhs }) = result {
        assert_eq!(op.kind, ClassSetBinaryOpKind::Union);
        assert_eq!(rhs, &rhs);
    } else {
        panic!("Expected a Some variant with BinaryRHS frame");
    }
}

#[test]
fn test_pop_class_binary_rhs() {
    struct MockAst;
    let induct = ClassFrame::BinaryRHS {
        op: &ClassSetBinaryOp { span: Span, kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal))), rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal))) },
        rhs: &ClassSet::Item(ClassSetItem::Literal(Literal)),
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);

    assert_eq!(result, None);
}

